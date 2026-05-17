# Project Definition

## Goal

Build an ESP32-WROOM-32U device that reads OBD-II data directly over CAN, computes fuel consumption and fuel cost, and displays key trip metrics on an I2C OLED.

## Non-Goals

- No ELM327 or AT-command adapter support
- No ESP32-specific firmware implementation yet
- No schematic or PCB design files yet
- No cloud dependency for core operation
- No vehicle write operations or ECU configuration changes

## Functional Requirements

- Query standard OBD-II PIDs over ISO 15765-4 CAN.
- Track live fuel rate, live fuel consumption, cost per hour, and cost per 100 km from available vehicle data.
- Allow fuel price and basic display interaction through two buttons.
- Present readable status, live values, and error states on the OLED.
- Support host-side tests for protocol parsing and calculation logic.

## Engineering Constraints

- Automotive 12 V input must include reverse-polarity, transient, and overcurrent protection.
- CAN interface must use the ESP32 TWAI peripheral with SN65HVD232QD or compatible 3.3 V CAN transceiver.
- Firmware must separate hardware access from domain logic using Rust traits.
- Crates should be structured so protocol and calculation code can run on a host without ESP32 hardware.
- Shared core crates should remain `no_std` compatible and avoid heap allocation unless justified.

## Current Implementation Status

- Rust workspace exists with `can-core`, `obd-core`, `fuel-model`, `device-core`, and `simulator`.
- `can-core` supports standard 11-bit CAN identifiers and fixed-size classic CAN frames.
- `obd-core` supports OBD-II Mode 01 single-frame request/response handling for PID `0x00`, vehicle speed `0x0D`, and MAF `0x10`.
- `fuel-model` provides validated `f32` newtypes and helpers for MAF-derived fuel rate, L/100 km, EUR/h, EUR/100 km, and trip cost.
- `device-core` contains device state, display model derivation, a two-PID polling scheduler, and a single-step runtime over an abstract CAN bus.
- `simulator` can feed samples or fake ECU response frames into the runtime model and includes a simple CLI.

## Initial Milestones

1. Define hardware architecture and protection requirements. Done.
2. Define firmware crate boundaries and abstraction traits. Done for CAN/runtime core.
3. Implement host-testable OBD-II request/response parsing. Initial Mode 01 single-frame support done.
4. Implement host-side fuel and display calculations. Initial live metrics done.
5. Bring up ESP32 CAN, display, and buttons. Not started.
6. Integrate embedded firmware crate and hardware drivers. Not started.
