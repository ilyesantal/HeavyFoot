## ADR-004: Use protected 12 V input with buck conversion

Decision: Power the device from the vehicle 12 V rail through protection and a buck regulator, with an optional 3.3 V post-regulator.

Reason:
- Vehicle power is noisy and transient-prone
- ESP32 current draw makes linear-only regulation inefficient
- A post-regulator can reduce rail noise if testing shows display, RF, or sensor sensitivity

Consequence:
- Power design must be validated for thermals and transients
- Protection components are part of the baseline design, not optional accessories
