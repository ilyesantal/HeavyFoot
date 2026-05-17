//! CAN frame types and transport-facing abstractions.
//!
//! This crate is intentionally skeletal. It should stay hardware-agnostic so
//! ESP32 TWAI integration can live outside the shared core crates.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        assert!(true);
    }
}
