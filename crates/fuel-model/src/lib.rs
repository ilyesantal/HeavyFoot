//! Fuel consumption, trip, and cost calculation primitives.
//!
//! The API uses small validated newtypes around `f32`. Values are non-negative
//! and finite. The crate is `no_std` compatible and does not allocate.

#![cfg_attr(not(test), no_std)]

const GASOLINE_AFR: f32 = 14.7;
const GASOLINE_DENSITY_GRAMS_PER_LITER: f32 = 745.0;
const SECONDS_PER_HOUR: f32 = 3600.0;

/// Error returned when constructing a measurement from an invalid value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueError {
    /// The supplied value was negative.
    Negative,
    /// The supplied value was NaN or infinite.
    NonFinite,
}

fn validate_non_negative_finite(value: f32) -> Result<f32, ValueError> {
    if !value.is_finite() {
        Err(ValueError::NonFinite)
    } else if value < 0.0 {
        Err(ValueError::Negative)
    } else {
        Ok(value)
    }
}

/// Fuel price in euros per liter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuelPriceEurPerLiter(f32);

impl FuelPriceEurPerLiter {
    /// Creates a validated fuel price.
    pub fn new(value: f32) -> Result<Self, ValueError> {
        validate_non_negative_finite(value).map(Self)
    }

    /// Returns the inner value in euros per liter.
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Fuel consumption in liters per 100 kilometers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuelConsumptionLitersPer100Km(f32);

impl FuelConsumptionLitersPer100Km {
    /// Creates a validated fuel consumption value.
    pub fn new(value: f32) -> Result<Self, ValueError> {
        validate_non_negative_finite(value).map(Self)
    }

    /// Returns the inner value in liters per 100 kilometers.
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Fuel rate in liters per hour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FuelRateLitersPerHour(f32);

impl FuelRateLitersPerHour {
    /// Creates a validated fuel rate.
    pub fn new(value: f32) -> Result<Self, ValueError> {
        validate_non_negative_finite(value).map(Self)
    }

    /// Returns the inner value in liters per hour.
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Distance in kilometers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DistanceKm(f32);

impl DistanceKm {
    /// Creates a validated distance.
    pub fn new(value: f32) -> Result<Self, ValueError> {
        validate_non_negative_finite(value).map(Self)
    }

    /// Returns the inner value in kilometers.
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Money amount in euros.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoneyEur(f32);

impl MoneyEur {
    /// Creates a validated money amount.
    pub fn new(value: f32) -> Result<Self, ValueError> {
        validate_non_negative_finite(value).map(Self)
    }

    /// Returns the inner value in euros.
    pub fn value(self) -> f32 {
        self.0
    }
}

/// Calculates fuel cost per 100 kilometers.
pub fn cost_per_100_km(
    price: FuelPriceEurPerLiter,
    consumption: FuelConsumptionLitersPer100Km,
) -> MoneyEur {
    MoneyEur(price.value() * consumption.value())
}

/// Calculates fuel cost per hour.
pub fn cost_per_hour(price: FuelPriceEurPerLiter, rate: FuelRateLitersPerHour) -> MoneyEur {
    MoneyEur(price.value() * rate.value())
}

/// Calculates live fuel consumption from fuel rate and vehicle speed.
///
/// Returns `None` when speed is zero because L/100km is undefined while
/// stationary.
pub fn consumption_from_rate_and_speed(
    rate: FuelRateLitersPerHour,
    speed_kmh: u8,
) -> Option<FuelConsumptionLitersPer100Km> {
    if speed_kmh == 0 {
        None
    } else {
        Some(FuelConsumptionLitersPer100Km(
            rate.value() / f32::from(speed_kmh) * 100.0,
        ))
    }
}

/// Calculates trip fuel cost from distance and average consumption.
pub fn trip_cost(
    price: FuelPriceEurPerLiter,
    distance: DistanceKm,
    consumption: FuelConsumptionLitersPer100Km,
) -> MoneyEur {
    MoneyEur(price.value() * consumption.value() * distance.value() / 100.0)
}

/// Converts gasoline MAF air flow in g/s to approximate fuel rate in L/h.
pub fn gasoline_maf_to_fuel_rate_l_per_hour(
    maf_grams_per_second: f32,
) -> Result<FuelRateLitersPerHour, ValueError> {
    let maf_grams_per_second = validate_non_negative_finite(maf_grams_per_second)?;
    let liters_per_hour =
        maf_grams_per_second * SECONDS_PER_HOUR / (GASOLINE_AFR * GASOLINE_DENSITY_GRAMS_PER_LITER);

    FuelRateLitersPerHour::new(liters_per_hour)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        let difference = (actual - expected).abs();
        assert!(difference < 0.000_1, "actual={actual}, expected={expected}");
    }

    #[test]
    fn constructors_accept_normal_values() {
        assert_eq!(FuelPriceEurPerLiter::new(1.75).unwrap().value(), 1.75);
        assert_eq!(
            FuelConsumptionLitersPer100Km::new(6.5).unwrap().value(),
            6.5
        );
        assert_eq!(FuelRateLitersPerHour::new(0.8).unwrap().value(), 0.8);
        assert_eq!(DistanceKm::new(42.0).unwrap().value(), 42.0);
        assert_eq!(MoneyEur::new(12.3).unwrap().value(), 12.3);
    }

    #[test]
    fn constructors_accept_zero_values() {
        assert_eq!(FuelPriceEurPerLiter::new(0.0).unwrap().value(), 0.0);
        assert_eq!(
            FuelConsumptionLitersPer100Km::new(0.0).unwrap().value(),
            0.0
        );
        assert_eq!(FuelRateLitersPerHour::new(0.0).unwrap().value(), 0.0);
        assert_eq!(DistanceKm::new(0.0).unwrap().value(), 0.0);
        assert_eq!(MoneyEur::new(0.0).unwrap().value(), 0.0);
    }

    #[test]
    fn constructors_reject_negative_values() {
        assert_eq!(FuelPriceEurPerLiter::new(-1.0), Err(ValueError::Negative));
        assert_eq!(
            FuelConsumptionLitersPer100Km::new(-1.0),
            Err(ValueError::Negative)
        );
        assert_eq!(FuelRateLitersPerHour::new(-1.0), Err(ValueError::Negative));
        assert_eq!(DistanceKm::new(-1.0), Err(ValueError::Negative));
        assert_eq!(MoneyEur::new(-1.0), Err(ValueError::Negative));
    }

    #[test]
    fn constructors_reject_non_finite_values() {
        assert_eq!(
            FuelPriceEurPerLiter::new(f32::NAN),
            Err(ValueError::NonFinite)
        );
        assert_eq!(DistanceKm::new(f32::INFINITY), Err(ValueError::NonFinite));
    }

    #[test]
    fn calculates_cost_per_100_km() {
        let price = FuelPriceEurPerLiter::new(1.80).unwrap();
        let consumption = FuelConsumptionLitersPer100Km::new(6.5).unwrap();

        assert_close(cost_per_100_km(price, consumption).value(), 11.7);
    }

    #[test]
    fn calculates_cost_per_hour() {
        let price = FuelPriceEurPerLiter::new(1.80).unwrap();
        let rate = FuelRateLitersPerHour::new(0.75).unwrap();

        assert_close(cost_per_hour(price, rate).value(), 1.35);
    }

    #[test]
    fn calculates_consumption_from_rate_and_speed() {
        let rate = FuelRateLitersPerHour::new(6.0).unwrap();

        let consumption = consumption_from_rate_and_speed(rate, 60).unwrap();

        assert_close(consumption.value(), 10.0);
    }

    #[test]
    fn consumption_from_rate_and_speed_handles_zero_rate() {
        let rate = FuelRateLitersPerHour::new(0.0).unwrap();

        let consumption = consumption_from_rate_and_speed(rate, 80).unwrap();

        assert_eq!(consumption.value(), 0.0);
    }

    #[test]
    fn consumption_from_rate_and_speed_returns_none_at_zero_speed() {
        let rate = FuelRateLitersPerHour::new(6.0).unwrap();

        assert_eq!(consumption_from_rate_and_speed(rate, 0), None);
    }

    #[test]
    fn calculates_trip_cost() {
        let price = FuelPriceEurPerLiter::new(1.80).unwrap();
        let distance = DistanceKm::new(250.0).unwrap();
        let consumption = FuelConsumptionLitersPer100Km::new(6.5).unwrap();

        assert_close(trip_cost(price, distance, consumption).value(), 29.25);
    }

    #[test]
    fn calculations_handle_zero_values() {
        let price = FuelPriceEurPerLiter::new(0.0).unwrap();
        let distance = DistanceKm::new(0.0).unwrap();
        let consumption = FuelConsumptionLitersPer100Km::new(0.0).unwrap();
        let rate = FuelRateLitersPerHour::new(0.0).unwrap();

        assert_eq!(cost_per_100_km(price, consumption).value(), 0.0);
        assert_eq!(cost_per_hour(price, rate).value(), 0.0);
        assert_eq!(trip_cost(price, distance, consumption).value(), 0.0);
    }

    #[test]
    fn converts_normal_maf_to_fuel_rate() {
        let rate = gasoline_maf_to_fuel_rate_l_per_hour(10.0).unwrap();

        assert_close(rate.value(), 10.0 * 3600.0 / (14.7 * 745.0));
    }

    #[test]
    fn converts_zero_maf_to_zero_fuel_rate() {
        let rate = gasoline_maf_to_fuel_rate_l_per_hour(0.0).unwrap();

        assert_eq!(rate.value(), 0.0);
    }

    #[test]
    fn rejects_negative_maf() {
        assert_eq!(
            gasoline_maf_to_fuel_rate_l_per_hour(-1.0),
            Err(ValueError::Negative)
        );
    }

    #[test]
    fn rejects_non_finite_maf() {
        assert_eq!(
            gasoline_maf_to_fuel_rate_l_per_hour(f32::NAN),
            Err(ValueError::NonFinite)
        );
        assert_eq!(
            gasoline_maf_to_fuel_rate_l_per_hour(f32::INFINITY),
            Err(ValueError::NonFinite)
        );
    }
}
