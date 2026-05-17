//! Host-side simulation support for device behavior.
//!
//! This crate feeds fake OBD values into `device-core` and returns display
//! models. It intentionally has no CLI yet.

use device_core::{DeviceState, DisplayModel};
use fuel_model::{FuelPriceEurPerLiter, ValueError};
use obd_core::Mode01Value;

/// One simulated OBD sample.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SimSample {
    /// Vehicle speed in km/h.
    pub speed_kmh: Option<u8>,
    /// MAF air flow in g/s.
    pub maf_g_per_s: Option<f32>,
}

/// Host-side simulation state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Simulation {
    state: DeviceState,
}

impl Simulation {
    /// Creates a simulation with the supplied fuel price.
    pub fn new(fuel_price: FuelPriceEurPerLiter) -> Self {
        Self {
            state: DeviceState::new(fuel_price),
        }
    }

    pub fn state(&self) -> &DeviceState {
        &self.state
    }

    /// Applies one simulated sample and returns the current display model.
    pub fn apply_sample(&mut self, sample: SimSample) -> Result<DisplayModel, ValueError> {
        if let Some(speed) = sample.speed_kmh {
            self.state
                .update_from_obd_value(Mode01Value::VehicleSpeedKmh(speed))?;
        }

        if let Some(maf) = sample.maf_g_per_s {
            self.state
                .update_from_obd_value(Mode01Value::MafAirFlowRateGramsPerSecond(maf))?;
        }

        Ok(self.state.display_model())
    }
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
}
