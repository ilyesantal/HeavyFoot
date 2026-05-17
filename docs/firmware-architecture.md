# Firmware Architecture

## Language and Runtime

Firmware will be written in Rust for ESP32. The design should keep protocol and calculation logic independent of ESP-IDF or HAL-specific APIs so it can be tested on a host.

## Crate Structure

Planned crates under `crates/`:

- `obd-core`: OBD-II PID definitions, CAN frame encoding, response parsing, and error types.
- `fuel-model`: fuel-rate, trip, and cost calculations.
- `device-core`: application state machine and UI model without hardware dependencies.
- `esp32-fw`: ESP32 binary crate wiring TWAI, I2C display, buttons, storage, and timing.

Names may change when implementation begins.

## Hardware Abstraction

Domain crates should depend on traits, not concrete ESP32 drivers:

- `CanBus`: send and receive raw CAN frames with timeout/error reporting.
- `Display`: render structured screens or display commands.
- `Buttons`: report debounced button events.
- `Clock`: provide monotonic time for polling and trip accumulation.
- `SettingsStore`: persist fuel price and user settings.

The ESP32 firmware crate owns concrete implementations of these traits.

## OBD-II Flow

1. Configure CAN for ISO 15765-4, usually 500 kbit/s or 250 kbit/s depending on vehicle.
2. Send standard functional requests to OBD-II request ID `0x7DF`.
3. Receive ECU responses from `0x7E8` through `0x7EF`.
4. Parse single-frame responses first.
5. Add ISO-TP multi-frame support only for PIDs or diagnostics that require it.

## Test Strategy

- Unit-test PID encoding and response parsing on the host.
- Unit-test fuel and cost calculations with fixed time steps.
- Test application state with mock CAN, display, button, clock, and settings implementations.
- Keep hardware integration tests separate from host tests.

## Resource Philosophy

- Avoid heap allocation unless justified.
- Prefer fixed-capacity buffers and stack allocation.
- Shared crates should not assume an allocator exists.
- Dynamic dispatch should only be used when it improves clarity or testability.
