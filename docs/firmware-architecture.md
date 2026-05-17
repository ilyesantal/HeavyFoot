# Firmware Architecture

## Language and Runtime

Firmware will be written in Rust for ESP32. The design should keep protocol and calculation logic independent of ESP-IDF or HAL-specific APIs so it can be tested on a host.

## Crate Structure

Current workspace crates under `crates/`:

- `can-core`: standard 11-bit CAN IDs, classic CAN frames, CAN errors, and bus trait.
- `obd-core`: Mode 01 PID definitions, request frame encoding, single-frame response parsing, and `ObdClient`.
- `fuel-model`: validated measurement newtypes and fuel/cost calculations.
- `device-core`: application state, display model, polling scheduler, and single-step runtime over an abstract CAN bus.
- `simulator`: host-side fake ECU responses, sample simulation, and a simple CLI.

Planned embedded crate:

- `esp32-fw`: ESP32 binary crate wiring TWAI, I2C display, buttons, storage, timing, and board configuration.

## Hardware Abstraction

Domain crates should depend on traits, not concrete ESP32 drivers:

- `can_core::CanBus`: send and receive raw CAN frames for the current single-step runtime.
- `Display`: render structured screens or display commands. Future work.
- `Buttons`: report debounced button events. Future work.
- `Clock`: provide monotonic time for polling and trip accumulation. Future work.
- `SettingsStore`: persist fuel price and user settings. Future work.

The ESP32 firmware crate owns concrete implementations of these traits.

## OBD-II Flow

1. Configure CAN for ISO 15765-4, usually 500 kbit/s or 250 kbit/s depending on vehicle.
2. Send standard functional requests to OBD-II request ID `0x7DF`.
3. Receive ECU responses from `0x7E8` through `0x7EF`.
4. Parse Mode 01 single-frame responses for the supported PID set.
5. Add ISO-TP multi-frame support only for PIDs or diagnostics that require it.

Currently supported Mode 01 PIDs:

- `0x00`: supported PIDs 01-20
- `0x0D`: vehicle speed
- `0x10`: MAF air flow rate

## Runtime Model

The current host-testable runtime is:

```text
DeviceRuntime<B: CanBus>
  -> PollScheduler selects VehicleSpeed or MafAirFlowRate
  -> ObdClient builds request frame
  -> CanBus sends request and receives at most one response
  -> ObdClient parses response when present
  -> DeviceState updates values
  -> DisplayModel exposes render-ready metrics
```

The runtime intentionally has no timing, retries, filtering, async, timeout policy, display driver, or button handling yet.

## Test Strategy

- Unit-test PID encoding and response parsing on the host.
- Unit-test fuel and cost calculations with fixed time steps.
- Test application state and runtime behavior with mock CAN implementations.
- Keep hardware integration tests separate from host tests.

## Resource Philosophy

- Avoid heap allocation unless justified.
- Prefer fixed-capacity buffers and stack allocation.
- Shared crates should not assume an allocator exists.
- Dynamic dispatch should only be used when it improves clarity or testability.

## Current Runtime Model

The hardware-independent runtime is built around:

```text
DeviceRuntime<B: CanBus>
```
