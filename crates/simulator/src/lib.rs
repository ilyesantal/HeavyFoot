//! Host-side simulation support for device behavior.
//!
//! This crate feeds fake OBD values into `device-core` and returns display
//! models. It intentionally has no CLI yet.

use can_core::{CanFrame, CanId};
use device_core::{DeviceState, DisplayModel};
use fuel_model::{gasoline_maf_to_fuel_rate_l_per_hour, FuelPriceEurPerLiter, ValueError};
use obd_core::{Mode01Pid, ObdClient, ObdError};

const SIM_ECU_RESPONSE_ID: u16 = 0x7e8;

/// One simulated OBD sample.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimSample {
    /// Vehicle speed in km/h.
    pub speed_kmh: Option<u8>,
    /// MAF air flow in g/s.
    pub maf_g_per_s: Option<f32>,
}

/// Errors returned by frame-based simulation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SimulationError {
    /// OBD parsing failed.
    Obd(ObdError),
    /// Device state update failed.
    Value(ValueError),
}

impl From<ObdError> for SimulationError {
    fn from(error: ObdError) -> Self {
        Self::Obd(error)
    }
}

impl From<ValueError> for SimulationError {
    fn from(error: ValueError) -> Self {
        Self::Value(error)
    }
}

/// Host-side simulation state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Simulation {
    state: DeviceState,
    obd_client: ObdClient,
}

impl Simulation {
    /// Creates a simulation with the supplied fuel price.
    pub fn new(fuel_price: FuelPriceEurPerLiter) -> Self {
        Self {
            state: DeviceState::new(fuel_price),
            obd_client: ObdClient::new(),
        }
    }

    pub fn state(&self) -> &DeviceState {
        &self.state
    }

    /// Applies one simulated sample and returns the current display model.
    pub fn apply_sample(&mut self, sample: SimSample) -> Result<DisplayModel, SimulationError> {
        if let Some(speed) = sample.speed_kmh {
            let frame = vehicle_speed_response(speed);
            self.apply_frame(frame, Mode01Pid::VehicleSpeed)?;
        }

        if let Some(maf) = sample.maf_g_per_s {
            let frame = maf_response(maf)?;
            self.apply_frame(frame, Mode01Pid::MafAirFlowRate)?;
        }

        Ok(self.state.display_model())
    }

    /// Applies one fake ECU response frame and returns the current display model.
    pub fn apply_frame(
        &mut self,
        frame: CanFrame,
        expected_pid: Mode01Pid,
    ) -> Result<DisplayModel, SimulationError> {
        let value = self.obd_client.parse_response(&frame, expected_pid)?;
        self.state.update_from_obd_value(value)?;

        Ok(self.state.display_model())
    }
}

/// Encodes a fake ECU response for vehicle speed.
pub fn vehicle_speed_response(speed_kmh: u8) -> CanFrame {
    let data = [0x03, 0x41, 0x0d, speed_kmh, 0, 0, 0, 0];

    CanFrame::new(CanId::new(SIM_ECU_RESPONSE_ID).unwrap(), data, 8).unwrap()
}

/// Encodes a fake ECU response for MAF air flow.
pub fn maf_response(maf_g_per_s: f32) -> Result<CanFrame, ValueError> {
    gasoline_maf_to_fuel_rate_l_per_hour(maf_g_per_s)?;

    let raw = (maf_g_per_s * 100.0) as u16;
    let a = (raw / 256) as u8;
    let b = (raw % 256) as u8;
    let data = [0x04, 0x41, 0x10, a, b, 0, 0, 0];

    Ok(CanFrame::new(CanId::new(SIM_ECU_RESPONSE_ID).unwrap(), data, 8).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulation_with_price(price: f32) -> Simulation {
        Simulation::new(FuelPriceEurPerLiter::new(price).unwrap())
    }

    fn assert_close(actual: f32, expected: f32) {
        let difference = (actual - expected).abs();
        assert!(difference < 0.000_1, "actual={actual}, expected={expected}");
    }

    #[test]
    fn empty_sample_gives_empty_display_values() {
        let mut simulation = simulation_with_price(2.0);

        let display = simulation.apply_sample(SimSample::default()).unwrap();

        assert_eq!(display.speed_kmh, None);
        assert_eq!(display.fuel_rate_l_per_hour, None);
        assert_eq!(display.cost_eur_per_hour, None);
    }

    #[test]
    fn encodes_vehicle_speed_response() {
        let frame = vehicle_speed_response(88);

        assert_eq!(frame.id().value(), 0x7e8);
        assert_eq!(frame.data(), [0x03, 0x41, 0x0d, 88, 0, 0, 0, 0]);
    }

    #[test]
    fn encodes_maf_response() {
        let frame = maf_response(4.0).unwrap();

        assert_eq!(frame.id().value(), 0x7e8);
        assert_eq!(frame.data(), [0x04, 0x41, 0x10, 0x01, 0x90, 0, 0, 0]);
    }

    #[test]
    fn speed_only_sample_updates_speed() {
        let mut simulation = simulation_with_price(2.0);

        let display = simulation
            .apply_sample(SimSample {
                speed_kmh: Some(88),
                maf_g_per_s: None,
            })
            .unwrap();

        assert_eq!(display.speed_kmh, Some(88));
        assert_eq!(display.fuel_rate_l_per_hour, None);
        assert_eq!(display.cost_eur_per_hour, None);
    }

    #[test]
    fn maf_only_sample_calculates_fuel_rate_and_cost() {
        let mut simulation = simulation_with_price(2.0);

        let display = simulation
            .apply_sample(SimSample {
                speed_kmh: None,
                maf_g_per_s: Some(10.0),
            })
            .unwrap();

        let expected_rate = 10.0 * 3600.0 / (14.7 * 745.0);
        assert_eq!(display.speed_kmh, None);
        assert_close(display.fuel_rate_l_per_hour.unwrap().value(), expected_rate);
        assert_close(
            display.cost_eur_per_hour.unwrap().value(),
            expected_rate * 2.0,
        );
    }

    #[test]
    fn multiple_samples_preserve_latest_values() {
        let mut simulation = simulation_with_price(2.0);

        simulation
            .apply_sample(SimSample {
                speed_kmh: Some(50),
                maf_g_per_s: Some(8.0),
            })
            .unwrap();

        let display = simulation
            .apply_sample(SimSample {
                speed_kmh: None,
                maf_g_per_s: Some(10.0),
            })
            .unwrap();

        let expected_rate = 10.0 * 3600.0 / (14.7 * 745.0);
        assert_eq!(display.speed_kmh, Some(50));
        assert_close(display.fuel_rate_l_per_hour.unwrap().value(), expected_rate);
    }

    #[test]
    fn apply_frame_rejects_wrong_pid() {
        let mut simulation = simulation_with_price(2.0);
        let frame = vehicle_speed_response(88);

        assert_eq!(
            simulation.apply_frame(frame, Mode01Pid::MafAirFlowRate),
            Err(SimulationError::Obd(ObdError::WrongPid))
        );
    }
}
