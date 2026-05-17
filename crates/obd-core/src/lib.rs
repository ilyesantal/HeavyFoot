//! OBD-II protocol definitions and request/response parsing.
//!
//! This crate currently supports only OBD-II Mode 01 single-frame CAN requests
//! and responses. ISO-TP multi-frame support is intentionally out of scope.

#![cfg_attr(not(test), no_std)]

use can_core::{CanError, CanFrame, CanId};

/// Functional OBD-II request CAN identifier.
pub const FUNCTIONAL_REQUEST_CAN_ID: u16 = 0x7df;

/// First standard ECU response CAN identifier.
pub const ECU_RESPONSE_ID_START: u16 = 0x7e8;

/// Last standard ECU response CAN identifier.
pub const ECU_RESPONSE_ID_END: u16 = 0x7ef;

const MODE_01_REQUEST_SERVICE: u8 = 0x01;
const MODE_01_RESPONSE_SERVICE: u8 = 0x41;

/// Supported Mode 01 PIDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode01Pid {
    /// Supported PIDs from 0x01 through 0x20.
    SupportedPids01To20 = 0x00,
    /// Vehicle speed in km/h.
    VehicleSpeed = 0x0d,
    /// MAF air flow rate in g/s.
    MafAirFlowRate = 0x10,
}

impl Mode01Pid {
    /// Returns the raw PID byte.
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Parsed Mode 01 response value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode01Value {
    /// PID 0x00 supported-PID bitfield.
    SupportedPids01To20([u8; 4]),
    /// PID 0x0D vehicle speed in km/h.
    VehicleSpeedKmh(u8),
    /// PID 0x10 MAF air flow rate in g/s.
    MafAirFlowRateGramsPerSecond(f32),
}

/// Errors returned by OBD-II request/response helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObdError {
    /// Underlying CAN frame construction failed.
    Can(CanError),
    /// Frame did not use a standard ECU response identifier.
    NonResponseId,
    /// Frame payload was too short for the expected response.
    ShortPayload,
    /// Response service byte did not match Mode 01 response service.
    WrongService,
    /// Response PID did not match the requested PID.
    WrongPid,
}

impl From<CanError> for ObdError {
    fn from(error: CanError) -> Self {
        Self::Can(error)
    }
}

/// Returns true when the identifier is in the standard ECU response range.
pub fn is_ecu_response_id(id: CanId) -> bool {
    let value = id.value();
    (ECU_RESPONSE_ID_START..=ECU_RESPONSE_ID_END).contains(&value)
}

/// Encodes a Mode 01 PID request as a single 8-byte CAN frame.
pub fn encode_mode01_request(pid: Mode01Pid) -> Result<CanFrame, ObdError> {
    let id = CanId::new(FUNCTIONAL_REQUEST_CAN_ID)?;
    let data = [0x02, MODE_01_REQUEST_SERVICE, pid.value(), 0, 0, 0, 0, 0];

    CanFrame::new(id, data, 8).map_err(ObdError::from)
}

/// Parses a single-frame Mode 01 response for the expected PID.
pub fn parse_mode01_response(
    frame: &CanFrame,
    expected_pid: Mode01Pid,
) -> Result<Mode01Value, ObdError> {
    if !is_ecu_response_id(frame.id()) {
        return Err(ObdError::NonResponseId);
    }

    let payload = frame.payload();
    if payload.len() < 3 {
        return Err(ObdError::ShortPayload);
    }

    let response_len = payload[0] as usize;
    if response_len < 2 || response_len + 1 > payload.len() {
        return Err(ObdError::ShortPayload);
    }

    if payload[1] != MODE_01_RESPONSE_SERVICE {
        return Err(ObdError::WrongService);
    }

    if payload[2] != expected_pid.value() {
        return Err(ObdError::WrongPid);
    }

    match expected_pid {
        Mode01Pid::SupportedPids01To20 => {
            require_service_payload_len(response_len, payload.len(), 6)?;
            Ok(Mode01Value::SupportedPids01To20([
                payload[3], payload[4], payload[5], payload[6],
            ]))
        }
        Mode01Pid::VehicleSpeed => {
            require_service_payload_len(response_len, payload.len(), 3)?;
            Ok(Mode01Value::VehicleSpeedKmh(payload[3]))
        }
        Mode01Pid::MafAirFlowRate => {
            require_service_payload_len(response_len, payload.len(), 4)?;
            let raw = u16::from_be_bytes([payload[3], payload[4]]);
            Ok(Mode01Value::MafAirFlowRateGramsPerSecond(
                raw as f32 / 100.0,
            ))
        }
    }
}

fn require_service_payload_len(
    response_len: usize,
    payload_len: usize,
    required_service_bytes: usize,
) -> Result<(), ObdError> {
    let required_total_payload_len = 1 + required_service_bytes;

    if response_len < required_service_bytes || payload_len < required_total_payload_len {
        Err(ObdError::ShortPayload)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_frame(id: u16, data: [u8; 8], len: u8) -> CanFrame {
        CanFrame::new(CanId::new(id).unwrap(), data, len).unwrap()
    }

    #[test]
    fn encodes_mode01_request() {
        let frame = encode_mode01_request(Mode01Pid::VehicleSpeed).unwrap();

        assert_eq!(frame.id().value(), FUNCTIONAL_REQUEST_CAN_ID);
        assert_eq!(frame.len(), 8);
        assert_eq!(frame.data(), [0x02, 0x01, 0x0d, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn accepts_ecu_response_ids() {
        assert!(is_ecu_response_id(CanId::new(0x7e8).unwrap()));
        assert!(is_ecu_response_id(CanId::new(0x7ef).unwrap()));
    }

    #[test]
    fn rejects_non_response_ids() {
        let request_id = CanId::new(FUNCTIONAL_REQUEST_CAN_ID).unwrap();
        assert!(!is_ecu_response_id(request_id));

        let frame = response_frame(0x7df, [0x03, 0x41, 0x0d, 88, 0, 0, 0, 0], 8);
        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Err(ObdError::NonResponseId)
        );
    }

    #[test]
    fn parses_vehicle_speed() {
        let frame = response_frame(0x7e8, [0x03, 0x41, 0x0d, 88, 0, 0, 0, 0], 8);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Ok(Mode01Value::VehicleSpeedKmh(88))
        );
    }

    #[test]
    fn parses_maf_air_flow_rate() {
        let frame = response_frame(0x7e8, [0x04, 0x41, 0x10, 0x01, 0x90, 0, 0, 0], 8);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::MafAirFlowRate),
            Ok(Mode01Value::MafAirFlowRateGramsPerSecond(4.0))
        );
    }

    #[test]
    fn rejects_wrong_service() {
        let frame = response_frame(0x7e8, [0x03, 0x40, 0x0d, 88, 0, 0, 0, 0], 8);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Err(ObdError::WrongService)
        );
    }

    #[test]
    fn rejects_wrong_pid() {
        let frame = response_frame(0x7e8, [0x03, 0x41, 0x10, 88, 0, 0, 0, 0], 8);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Err(ObdError::WrongPid)
        );
    }

    #[test]
    fn rejects_short_payload_from_frame_len() {
        let frame = response_frame(0x7e8, [0x03, 0x41, 0x0d, 88, 0, 0, 0, 0], 3);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Err(ObdError::ShortPayload)
        );
    }

    #[test]
    fn rejects_short_payload_from_response_len() {
        let frame = response_frame(0x7e8, [0x02, 0x41, 0x0d, 88, 0, 0, 0, 0], 8);

        assert_eq!(
            parse_mode01_response(&frame, Mode01Pid::VehicleSpeed),
            Err(ObdError::ShortPayload)
        );
    }
}
