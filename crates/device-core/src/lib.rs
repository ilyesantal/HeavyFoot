//! Hardware-independent device state and application coordination.
//!
//! This crate contains state and presentation models that do not depend on
//! ESP32, HAL, display, or CAN driver code.

#![cfg_attr(not(test), no_std)]

use can_core::{CanBus, CanError};
use fuel_model::{
    consumption_from_rate_and_speed, cost_per_100_km, cost_per_hour,
    gasoline_maf_to_fuel_rate_l_per_hour, FuelConsumptionLitersPer100Km, FuelPriceEurPerLiter,
    FuelRateLitersPerHour, MoneyEur, ValueError,
};
use obd_core::{Mode01Pid, Mode01Value, ObdClient, ObdError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PollSlot {
    VehicleSpeed,
    MafAirFlowRate,
}

/// Simple PID scheduler for the initial polling sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PollScheduler {
    next: PollSlot,
}

impl PollScheduler {
    /// Creates a scheduler starting with vehicle speed.
    pub fn new() -> Self {
        Self {
            next: PollSlot::VehicleSpeed,
        }
    }

    /// Returns the next PID to request and advances the sequence.
    /// Current simple round-robin polling order:
    /// VehicleSpeed -> MafAirFlowRate -> repeat
    #[must_use]
    pub fn next_pid(&mut self) -> Mode01Pid {
        match self.next {
            PollSlot::VehicleSpeed => {
                self.next = PollSlot::MafAirFlowRate;
                Mode01Pid::VehicleSpeed
            }
            PollSlot::MafAirFlowRate => {
                self.next = PollSlot::VehicleSpeed;
                Mode01Pid::MafAirFlowRate
            }
        }
    }
}

impl Default for PollScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors returned by the device runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeError {
    /// CAN bus operation failed.
    Can(CanError),
    /// OBD request or response handling failed.
    Obd(ObdError),
    /// Device state update failed.
    Value(ValueError),
}

impl From<CanError> for RuntimeError {
    fn from(error: CanError) -> Self {
        Self::Can(error)
    }
}

impl From<ObdError> for RuntimeError {
    fn from(error: ObdError) -> Self {
        Self::Obd(error)
    }
}

impl From<ValueError> for RuntimeError {
    fn from(error: ValueError) -> Self {
        Self::Value(error)
    }
}

/// Single-step hardware-independent device runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceRuntime<B: CanBus> {
    state: DeviceState,
    scheduler: PollScheduler,
    obd_client: ObdClient,
    bus: B,
}

impl<B: CanBus> DeviceRuntime<B> {
    /// Creates a runtime from a CAN bus implementation and fuel price.
    pub fn new(bus: B, fuel_price: FuelPriceEurPerLiter) -> Self {
        Self {
            state: DeviceState::new(fuel_price),
            scheduler: PollScheduler::new(),
            obd_client: ObdClient::new(),
            bus,
        }
    }

    /// Returns the current device state.
    pub fn state(&self) -> &DeviceState {
        &self.state
    }

    /// Returns the underlying bus implementation.
    pub fn bus(&self) -> &B {
        &self.bus
    }

    /// Runs one polling step and returns the current display model.
    pub fn step(&mut self) -> Result<DisplayModel, RuntimeError> {
        let pid = self.scheduler.next_pid();
        let request = self.obd_client.request_frame(pid)?;

        self.bus.send(&request)?;

        if let Some(response) = self.bus.receive()? {
            let value = self.obd_client.parse_response(&response, pid)?;
            self.state.update_from_obd_value(value)?;
        }

        Ok(self.state.display_model())
    }
}

/// Hardware-independent device state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceState {
    fuel_price: FuelPriceEurPerLiter,
    latest_vehicle_speed_kmh: Option<u8>,
    latest_maf_air_flow_grams_per_second: Option<f32>,
}

impl DeviceState {
    /// Creates a new device state with no vehicle readings yet.
    pub fn new(fuel_price: FuelPriceEurPerLiter) -> Self {
        Self {
            fuel_price,
            latest_vehicle_speed_kmh: None,
            latest_maf_air_flow_grams_per_second: None,
        }
    }

    /// Returns the configured fuel price.
    pub fn fuel_price(&self) -> FuelPriceEurPerLiter {
        self.fuel_price
    }

    /// Returns the latest vehicle speed in km/h, if available.
    pub fn latest_vehicle_speed_kmh(&self) -> Option<u8> {
        self.latest_vehicle_speed_kmh
    }

    /// Returns the latest MAF air flow in g/s, if available.
    pub fn latest_maf_air_flow_grams_per_second(&self) -> Option<f32> {
        self.latest_maf_air_flow_grams_per_second
    }

    /// Updates state from a parsed OBD-II Mode 01 value.
    pub fn update_from_obd_value(&mut self, value: Mode01Value) -> Result<(), ValueError> {
        match value {
            Mode01Value::SupportedPids01To20(_) => {}
            Mode01Value::VehicleSpeedKmh(speed) => {
                self.latest_vehicle_speed_kmh = Some(speed);
            }
            Mode01Value::MafAirFlowRateGramsPerSecond(maf) => {
                gasoline_maf_to_fuel_rate_l_per_hour(maf)?;
                self.latest_maf_air_flow_grams_per_second = Some(maf);
            }
        }

        Ok(())
    }

    /// Builds the current display model from stored readings.
    pub fn display_model(&self) -> DisplayModel {
        let fuel_rate_l_per_hour = self
            .latest_maf_air_flow_grams_per_second
            .and_then(|maf| gasoline_maf_to_fuel_rate_l_per_hour(maf).ok());

        let cost_eur_per_hour =
            fuel_rate_l_per_hour.map(|rate| cost_per_hour(self.fuel_price, rate));
        let consumption_l_per_100km = match (fuel_rate_l_per_hour, self.latest_vehicle_speed_kmh) {
            (Some(rate), Some(speed)) => consumption_from_rate_and_speed(rate, speed),
            _ => None,
        };
        let cost_eur_per_100km = consumption_l_per_100km
            .map(|consumption| cost_per_100_km(self.fuel_price, consumption));

        DisplayModel {
            speed_kmh: self.latest_vehicle_speed_kmh,
            fuel_rate_l_per_hour,
            cost_eur_per_hour,
            consumption_l_per_100km,
            cost_eur_per_100km,
        }
    }
}

/// Values intended for rendering on a display.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayModel {
    /// Vehicle speed in km/h.
    pub speed_kmh: Option<u8>,
    /// Fuel rate in liters per hour.
    pub fuel_rate_l_per_hour: Option<FuelRateLitersPerHour>,
    /// Fuel cost in euros per hour.
    pub cost_eur_per_hour: Option<MoneyEur>,
    /// Live fuel consumption in liters per 100 kilometers.
    pub consumption_l_per_100km: Option<FuelConsumptionLitersPer100Km>,
    /// Live fuel cost in euros per 100 kilometers.
    pub cost_eur_per_100km: Option<MoneyEur>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use can_core::{CanFrame, CanId};

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct MockCanBus {
        sent: [Option<CanFrame>; 4],
        sent_len: usize,
        received: [Option<CanFrame>; 4],
        receive_index: usize,
    }

    impl MockCanBus {
        fn new(received: [Option<CanFrame>; 4]) -> Self {
            Self {
                sent: [None; 4],
                sent_len: 0,
                received,
                receive_index: 0,
            }
        }

        fn sent_frame(&self, index: usize) -> CanFrame {
            self.sent[index].unwrap()
        }
    }

    impl CanBus for MockCanBus {
        fn send(&mut self, frame: &CanFrame) -> Result<(), CanError> {
            self.sent[self.sent_len] = Some(*frame);
            self.sent_len += 1;
            Ok(())
        }

        fn receive(&mut self) -> Result<Option<CanFrame>, CanError> {
            if self.receive_index >= self.received.len() {
                return Ok(None);
            }

            let frame = self.received[self.receive_index];
            self.receive_index += 1;
            Ok(frame)
        }
    }

    fn state_with_price(price: f32) -> DeviceState {
        DeviceState::new(FuelPriceEurPerLiter::new(price).unwrap())
    }

    fn runtime_with_responses(received: [Option<CanFrame>; 4]) -> DeviceRuntime<MockCanBus> {
        DeviceRuntime::new(
            MockCanBus::new(received),
            FuelPriceEurPerLiter::new(2.0).unwrap(),
        )
    }

    fn response_frame(data: [u8; 8]) -> CanFrame {
        CanFrame::new(CanId::new(0x7e8).unwrap(), data, 8).unwrap()
    }

    fn speed_response(speed: u8) -> CanFrame {
        response_frame([0x03, 0x41, 0x0d, speed, 0, 0, 0, 0])
    }

    fn maf_response(raw_a: u8, raw_b: u8) -> CanFrame {
        response_frame([0x04, 0x41, 0x10, raw_a, raw_b, 0, 0, 0])
    }

    fn assert_close(actual: f32, expected: f32) {
        let difference = (actual - expected).abs();
        assert!(difference < 0.000_1, "actual={actual}, expected={expected}");
    }

    #[test]
    fn initial_state_has_no_vehicle_readings() {
        let state = state_with_price(1.8);
        let display = state.display_model();

        assert_eq!(state.fuel_price().value(), 1.8);
        assert_eq!(state.latest_vehicle_speed_kmh(), None);
        assert_eq!(state.latest_maf_air_flow_grams_per_second(), None);
        assert_eq!(display.speed_kmh, None);
        assert_eq!(display.fuel_rate_l_per_hour, None);
        assert_eq!(display.cost_eur_per_hour, None);
        assert_eq!(display.consumption_l_per_100km, None);
        assert_eq!(display.cost_eur_per_100km, None);
    }

    #[test]
    fn poll_scheduler_repeats_vehicle_speed_then_maf() {
        let mut scheduler = PollScheduler::new();

        assert_eq!(scheduler.next_pid(), Mode01Pid::VehicleSpeed);
        assert_eq!(scheduler.next_pid(), Mode01Pid::MafAirFlowRate);
        assert_eq!(scheduler.next_pid(), Mode01Pid::VehicleSpeed);
        assert_eq!(scheduler.next_pid(), Mode01Pid::MafAirFlowRate);
    }

    #[test]
    fn runtime_scheduler_alternates_requests() {
        let mut runtime = runtime_with_responses([None, None, None, None]);

        runtime.step().unwrap();
        runtime.step().unwrap();
        runtime.step().unwrap();

        assert_eq!(
            runtime.bus().sent_frame(0).data(),
            [0x02, 0x01, 0x0d, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            runtime.bus().sent_frame(1).data(),
            [0x02, 0x01, 0x10, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            runtime.bus().sent_frame(2).data(),
            [0x02, 0x01, 0x0d, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn runtime_updates_speed_from_received_frame() {
        let mut runtime = runtime_with_responses([Some(speed_response(88)), None, None, None]);

        let display = runtime.step().unwrap();

        assert_eq!(runtime.state().latest_vehicle_speed_kmh(), Some(88));
        assert_eq!(display.speed_kmh, Some(88));
    }

    #[test]
    fn runtime_updates_maf_from_received_frame() {
        let mut runtime =
            runtime_with_responses([None, Some(maf_response(0x03, 0xe8)), None, None]);

        runtime.step().unwrap();
        let display = runtime.step().unwrap();

        let expected_rate = 10.0 * 3600.0 / (14.7 * 745.0);
        assert_eq!(
            runtime.state().latest_maf_air_flow_grams_per_second(),
            Some(10.0)
        );
        assert_close(display.fuel_rate_l_per_hour.unwrap().value(), expected_rate);
        assert_close(
            display.cost_eur_per_hour.unwrap().value(),
            expected_rate * 2.0,
        );
    }

    #[test]
    fn runtime_handles_no_response_frame() {
        let mut runtime = runtime_with_responses([None, None, None, None]);

        let display = runtime.step().unwrap();

        assert_eq!(runtime.state().latest_vehicle_speed_kmh(), None);
        assert_eq!(runtime.state().latest_maf_air_flow_grams_per_second(), None);
        assert_eq!(display.speed_kmh, None);
        assert_eq!(display.fuel_rate_l_per_hour, None);
    }

    #[test]
    fn updates_vehicle_speed() {
        let mut state = state_with_price(1.8);

        state
            .update_from_obd_value(Mode01Value::VehicleSpeedKmh(88))
            .unwrap();

        assert_eq!(state.latest_vehicle_speed_kmh(), Some(88));
        let display = state.display_model();
        assert_eq!(display.speed_kmh, Some(88));
        assert_eq!(display.consumption_l_per_100km, None);
        assert_eq!(display.cost_eur_per_100km, None);
    }

    #[test]
    fn updates_maf_air_flow() {
        let mut state = state_with_price(1.8);

        state
            .update_from_obd_value(Mode01Value::MafAirFlowRateGramsPerSecond(10.0))
            .unwrap();

        assert_eq!(state.latest_maf_air_flow_grams_per_second(), Some(10.0));

        let display = state.display_model();
        let fuel_rate = display.fuel_rate_l_per_hour.unwrap();
        assert_close(fuel_rate.value(), 10.0 * 3600.0 / (14.7 * 745.0));
        assert_eq!(display.consumption_l_per_100km, None);
        assert_eq!(display.cost_eur_per_100km, None);
    }

    #[test]
    fn display_model_includes_cost_calculation() {
        let mut state = state_with_price(2.0);

        state
            .update_from_obd_value(Mode01Value::VehicleSpeedKmh(50))
            .unwrap();
        state
            .update_from_obd_value(Mode01Value::MafAirFlowRateGramsPerSecond(10.0))
            .unwrap();

        let display = state.display_model();
        let expected_rate = 10.0 * 3600.0 / (14.7 * 745.0);

        assert_eq!(display.speed_kmh, Some(50));
        assert_close(display.fuel_rate_l_per_hour.unwrap().value(), expected_rate);
        assert_close(
            display.cost_eur_per_hour.unwrap().value(),
            expected_rate * 2.0,
        );
        assert_close(
            display.consumption_l_per_100km.unwrap().value(),
            expected_rate / 50.0 * 100.0,
        );
        assert_close(
            display.cost_eur_per_100km.unwrap().value(),
            expected_rate / 50.0 * 100.0 * 2.0,
        );
    }

    #[test]
    fn display_model_omits_per_100km_values_at_zero_speed() {
        let mut state = state_with_price(2.0);

        state
            .update_from_obd_value(Mode01Value::VehicleSpeedKmh(0))
            .unwrap();
        state
            .update_from_obd_value(Mode01Value::MafAirFlowRateGramsPerSecond(10.0))
            .unwrap();

        let display = state.display_model();
        assert_eq!(display.speed_kmh, Some(0));
        assert!(display.fuel_rate_l_per_hour.is_some());
        assert!(display.cost_eur_per_hour.is_some());
        assert_eq!(display.consumption_l_per_100km, None);
        assert_eq!(display.cost_eur_per_100km, None);
    }
}
