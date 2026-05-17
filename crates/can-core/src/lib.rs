//! CAN frame types and transport-facing abstractions.
//!
//! This crate is hardware-agnostic. It supports standard 11-bit CAN identifiers
//! only for now and is intended to be usable from `no_std` firmware crates.

#![cfg_attr(not(test), no_std)]

/// Maximum value for a standard 11-bit CAN identifier.
pub const MAX_STANDARD_ID: u16 = 0x7ff;
pub const MAX_DATA_LEN: u8 = 8;

/// A standard 11-bit CAN identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanId(u16);

impl CanId {
    /// Creates a standard 11-bit CAN identifier.
    pub fn new(value: u16) -> Result<Self, CanError> {
        if value <= MAX_STANDARD_ID {
            Ok(Self(value))
        } else {
            Err(CanError::InvalidId)
        }
    }

    /// Returns the raw 11-bit identifier value.
    pub fn value(self) -> u16 {
        self.0
    }
}

/// A CAN data frame with up to 8 payload bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanFrame {
    id: CanId,
    data: [u8; 8],
    len: u8,
}

impl CanFrame {
    /// Creates a CAN frame after validating the payload length.
    pub fn new(id: CanId, data: [u8; 8], len: u8) -> Result<Self, CanError> {
        if len <= MAX_DATA_LEN {
            Ok(Self { id, data, len })
        } else {
            Err(CanError::InvalidLength)
        }
    }

    /// Returns the frame identifier.
    pub fn id(&self) -> CanId {
        self.id
    }

    /// Returns the full fixed-size data buffer.
    pub fn data(&self) -> [u8; 8] {
        self.data
    }

    /// Returns the payload length in bytes.
    pub fn len(&self) -> u8 {
        self.len
    }

    /// Returns true when the payload length is zero.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the active payload bytes.
    pub fn payload(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

/// Errors exposed by CAN abstractions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanError {
    /// Identifier is not supported by this crate.
    InvalidId,
    /// Payload length is invalid.
    InvalidLength,
    /// Send operation could not be completed.
    SendFailed,
    /// Receive operation could not be completed.
    ReceiveFailed,
    /// Operation timed out.
    Timeout,
    /// Bus entered an error state.
    BusOff,
}

/// Hardware-neutral CAN bus interface.
pub trait CanBus {
    /// Sends one CAN frame.
    fn send(&mut self, frame: &CanFrame) -> Result<(), CanError>;

    /// Receives one CAN frame if one is available.
    fn receive(&mut self) -> Result<Option<CanFrame>, CanError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_standard_ids() {
        assert_eq!(CanId::new(0x000).unwrap().value(), 0x000);
        assert_eq!(CanId::new(0x7ff).unwrap().value(), 0x7ff);
    }

    #[test]
    fn rejects_invalid_standard_ids() {
        assert_eq!(CanId::new(0x800), Err(CanError::InvalidId));
    }

    #[test]
    fn accepts_valid_frames() {
        let id = CanId::new(0x123).unwrap();
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let frame = CanFrame::new(id, data, 8).unwrap();

        assert_eq!(frame.id(), id);
        assert_eq!(frame.data(), data);
        assert_eq!(frame.len(), 8);
        assert!(!frame.is_empty());
    }

    #[test]
    fn accepts_zero_length_frames() {
        let id = CanId::new(0x123).unwrap();
        let frame = CanFrame::new(id, [0; 8], 0).unwrap();

        assert_eq!(frame.len(), 0);
        assert!(frame.is_empty());
        assert_eq!(frame.payload(), &[]);
    }

    #[test]
    fn rejects_invalid_lengths() {
        let id = CanId::new(0x123).unwrap();

        assert_eq!(CanFrame::new(id, [0; 8], 9), Err(CanError::InvalidLength));
    }

    #[test]
    fn payload_returns_only_active_bytes() {
        let id = CanId::new(0x123).unwrap();
        let frame = CanFrame::new(id, [10, 20, 30, 40, 50, 60, 70, 80], 3).unwrap();

        assert_eq!(frame.payload(), &[10, 20, 30]);
    }
}
