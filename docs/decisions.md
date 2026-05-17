# Architecture Decisions

## ADR-001: Use direct CAN instead of ELM327

Decision: Use ESP32 TWAI + external CAN transceiver.

Reason:
- Better learning value
- Full control over OBD-II protocol
- Cleaner firmware architecture
- No dependency on ELM327 clone behavior

Consequence:
- More firmware work
- Need CAN/OBD mocks for host testing
- Hardware must include proper CAN transceiver and protection

## ADR-002: Use ESP32-WROOM-32U

Decision: Target ESP32-WROOM-32U as the initial MCU module.

Reason:
- Integrated Wi-Fi/Bluetooth available for future diagnostics or configuration
- External antenna option improves enclosure flexibility
- TWAI peripheral supports direct CAN controller operation
- Rust ecosystem support is available through ESP-IDF and embedded tooling

Consequence:
- Antenna placement and enclosure design matter
- GPIO selection must account for ESP32 boot strapping pins
- Firmware must account for ESP32 TWAI behavior and driver constraints

## ADR-003: Keep firmware logic host-testable

Decision: Separate protocol, calculation, and application state from hardware drivers.

Reason:
- OBD-II parsing and fuel calculations can be tested without a vehicle
- Mock hardware makes UI and polling behavior deterministic
- Hardware bring-up can proceed without blocking core logic tests

Consequence:
- Hardware abstraction traits are required early
- ESP32-specific code should remain in the firmware binary crate
- Shared crates must avoid unnecessary `std` or HAL dependencies

## ADR-004: Use protected 12 V input with buck conversion

Decision: Power the device from the vehicle 12 V rail through protection and a buck regulator, with an optional 3.3 V post-regulator.

Reason:
- Vehicle power is noisy and transient-prone
- ESP32 current draw makes linear-only regulation inefficient
- A post-regulator can reduce rail noise if testing shows display, RF, or sensor sensitivity

Consequence:
- Power design must be validated for thermals and transients
- Protection components are part of the baseline design, not optional accessories
