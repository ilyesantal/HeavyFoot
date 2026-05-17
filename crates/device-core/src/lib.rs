//! Hardware-independent device state and application coordination.
//!
//! This crate is intentionally skeletal. It should define application behavior
//! against traits rather than concrete ESP32 drivers.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        assert!(true);
    }
}
