# HeavyFoot

ESP32-based OBD-II fuel cost monitor.

The device connects directly to a vehicle CAN bus through an automotive CAN transceiver and queries OBD-II PIDs without using an ELM327 adapter. It estimates fuel usage and cost from vehicle data, then presents current trip values on a small I2C OLED display.

![CI](https://github.com/ilyesantal/HeavyFoot/actions/workflows/rust.yml/badge.svg)

## Scope

This repository currently contains the host-testable core crates, a small simulator, and hardware/firmware architecture documentation. ESP32-specific firmware, PCB schematics, and hardware drivers are not implemented yet.

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
- Host-testable crates for CAN frames, OBD-II protocol parsing, fuel calculations, device state, and runtime orchestration
- `no_std`-compatible shared crates where practical
- Hardware abstraction traits for CAN first; display, buttons, storage, and time remain future work

## Current Rust Workspace

- `crates/can-core`: standard 11-bit CAN IDs, fixed-size CAN frames, CAN errors, and CAN bus trait.
- `crates/obd-core`: OBD-II Mode 01 single-frame request/response handling for supported PIDs, vehicle speed, and MAF.
- `crates/fuel-model`: fuel price, rate, consumption, distance, and cost newtypes plus calculation helpers.
- `crates/device-core`: device state, display model, PID polling scheduler, and a single-step runtime over an abstract CAN bus.
- `crates/simulator`: host-side simulation library and simple CLI using fake ECU response frames.

## Common Commands

```sh
cargo test --workspace
cargo run -p simulator
```

## Repository Layout

- `PROJECT.md`: requirements, constraints, and milestones
- `docs/hardware-architecture.md`: hardware block design
- `docs/firmware-architecture.md`: firmware module boundaries
- `docs/runtime-architecture.md`: current host-testable runtime flow
- `docs/protocol-notes.md`: current OBD-II protocol scope
- `docs/power-subsystem.md`: automotive power input architecture
- `docs/power-topology.md`: concise power topology assumptions and open rail questions
- `docs/decisions.md`: architecture decisions
- `docs/ai-workflow.md`: AI-assisted development workflow
- `crates/`: Rust workspace crates

## Development Notes

This project was developed with the assistance of AI tools including ChatGPT and Codex.
All generated code is reviewed, modified, and integrated manually.
