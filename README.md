# HeavyFoot

ESP32-based OBD-II fuel cost monitor.

The device connects directly to a vehicle CAN bus through an automotive CAN transceiver and queries OBD-II PIDs without using an ELM327 adapter. It estimates fuel usage and cost from vehicle data, then presents current trip values on a small I2C OLED display.

## Scope

This repository currently defines project structure, hardware direction, firmware architecture, and development workflow. Firmware implementation is intentionally not included yet.

## Target Hardware

- MCU: ESP32-WROOM-32U
- CAN transceiver: SN65HVD232QD
- Vehicle input: protected automotive 12 V supply
- Power conversion: buck regulator with optional 3.3 V LDO or post-regulator
- Display: I2C OLED
- Controls: two user buttons

## Firmware Direction

- Rust firmware
- Direct CAN and OBD-II implementation
- Host-testable crates for protocol, calculations, and state logic
- Hardware abstraction traits for CAN, display, buttons, storage, and time

## Repository Layout

- `PROJECT.md`: requirements, constraints, and milestones
- `docs/hardware-architecture.md`: hardware block design
- `docs/firmware-architecture.md`: firmware module boundaries
- `docs/decisions.md`: architecture decisions
- `docs/ai-workflow.md`: AI-assisted development workflow
- `crates/`: future Rust crates

## Development Notes

This project was developed with the assistance of AI tools including ChatGPT and Codex.
All generated code is reviewed, modified, and integrated manually.
