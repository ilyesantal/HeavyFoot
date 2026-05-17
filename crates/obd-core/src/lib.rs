//! OBD-II protocol definitions and request/response parsing.
//!
//! This crate is intentionally skeletal. It should contain protocol logic that
//! can be tested without a vehicle, ESP32 board, or CAN peripheral.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        assert!(true);
    }
}
