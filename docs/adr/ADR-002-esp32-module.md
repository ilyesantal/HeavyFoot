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
