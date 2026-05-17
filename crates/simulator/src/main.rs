use fuel_model::FuelPriceEurPerLiter;
use simulator::{SimSample, Simulation};

fn main() {
    let fuel_price = FuelPriceEurPerLiter::new(1.80).expect("hardcoded fuel price is valid");
    let mut simulation = Simulation::new(fuel_price);

    let samples = [
        SimSample {
            speed_kmh: None,
            maf_g_per_s: None,
        },
        SimSample {
            speed_kmh: Some(30),
            maf_g_per_s: Some(4.0),
        },
        SimSample {
            speed_kmh: Some(80),
            maf_g_per_s: Some(12.0),
        },
        SimSample {
            speed_kmh: None,
            maf_g_per_s: Some(8.0),
        },
    ];

    for sample in samples {
        let display = simulation
            .apply_sample(sample)
            .expect("hardcoded simulation sample is valid");

        println!(
            "speed={} km/h fuel_rate={} L/h cost={} EUR/h",
            format_optional_u8(display.speed_kmh),
            format_optional_f32(display.fuel_rate_l_per_hour.map(|rate| rate.value())),
            format_optional_f32(display.cost_eur_per_hour.map(|cost| cost.value())),
        );
    }
}

fn format_optional_u8(value: Option<u8>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "-".to_string(),
    }
}

fn format_optional_f32(value: Option<f32>) -> String {
    match value {
        Some(value) => format!("{value:.2}"),
        None => "-".to_string(),
    }
}
