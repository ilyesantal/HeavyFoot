# Project Definition

## Goal

Build an ESP32-WROOM-32U device that reads OBD-II data directly over CAN, computes fuel consumption and fuel cost, and displays key trip metrics on an I2C OLED.

## Non-Goals

- No ELM327 or AT-command adapter support
- No firmware implementation in the initial repository scaffold
- No cloud dependency for core operation
- No vehicle write operations or ECU configuration changes

## Functional Requirements

- Query standard OBD-II PIDs over ISO 15765-4 CAN.
- Track live and trip fuel metrics from available vehicle data.
- Allow fuel price and basic display interaction through two buttons.
- Present readable status, live values, and error states on the OLED.
- Support host-side tests for protocol parsing and calculation logic.

## Engineering Constraints

- Automotive 12 V input must include reverse-polarity, transient, and overcurrent protection.
- CAN interface must use the ESP32 TWAI peripheral with SN65HVD232QD or compatible 3.3 V CAN transceiver.
- Firmware must separate hardware access from domain logic using Rust traits.
- Crates should be structured so protocol and calculation code can run on a host without ESP32 hardware.

## Initial Milestones

1. Define hardware architecture and protection requirements.
2. Define firmware crate boundaries and abstraction traits.
3. Implement host-testable OBD-II request/response parsing.
4. Bring up ESP32 CAN, display, and buttons.
5. Integrate trip state, fuel-cost calculation, and UI.
