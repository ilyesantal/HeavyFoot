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
