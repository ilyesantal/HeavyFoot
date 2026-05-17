# Hardware Pinout Planning

This document identifies required ESP32-WROOM-32U signal groups for the OBD-II fuel cost monitor. Final GPIO assignment is intentionally deferred until schematic capture and board layout.

## Required Signal Groups

| Signal group | Purpose | Notes |
|---|---|---|
| CAN TX/RX | ESP32 TWAI connection to SN65HVD232QD | Route as short digital signals between ESP32 and transceiver. Confirm chosen GPIOs support TWAI use in the selected firmware stack. |
| I2C SDA/SCL | OLED display bus | Add pull-ups sized for bus voltage, speed, trace length, and OLED module behavior. |
| Button 1 / Button 2 | User input | Prefer GPIOs with stable boot behavior. Use internal or external pull-ups and account for debounce in firmware. |
| Status LED | Basic device status | Budget LED current. Avoid boot strapping pins unless the LED circuit cannot alter boot state. |
| UART TX/RX | Boot, reset, logs, and programming | Preserve access to ESP32 programming UART. Provide header or pads if no USB-UART bridge is populated. |
| EN / reset | ESP32 reset and programming support | Include accessible reset control. Auto-programming circuit may require EN and boot control. |
| BOOT / GPIO0 | Programming mode control | Must respect boot strapping requirements. Provide button or programming circuit as needed. |
| CAN standby/silent | Optional transceiver mode control | SN65HVD232QD variant and selected circuit determine whether this signal exists. Tie to a defined state if unused. |
| Power enable/reset | Optional regulator or load switch control | Include only if power sequencing, sleep current, or rail control requires it. |

## ESP32 Boot Strapping Pins

Do not assign external circuits to ESP32 boot strapping pins without checking their required reset-time levels. Pull-ups, pull-downs, LEDs, buttons, transceiver control pins, and OLED circuitry can all disturb boot mode if attached incorrectly.

KiCAD pin assignment must explicitly review at least:

- GPIO0 boot/programming behavior
- EN reset behavior
- strapping pins used by flash voltage, boot mode, or debug behavior
- pins with input-only limitations
- pins used internally by module flash

## External Antenna Note

ESP32-WROOM-32U uses an external antenna connector. Keep copper, ground cutouts, enclosure walls, wiring, and the OBD cable away from the antenna region according to the module and antenna layout guidance. Final antenna placement should be reviewed with the enclosure concept, not only the schematic.

## Test Point Recommendations

Provide test access for:

- ESP32 3.3 V rail and local ground
- CAN TX and CAN RX between ESP32 and transceiver
- CAN-H and CAN-L after connector-side protection
- I2C SDA and SCL
- Button GPIOs
- Status LED GPIO
- UART TX, UART RX, EN, and BOOT/GPIO0
- Optional CAN standby/silent control
- Optional power enable/reset signals

Test points should be reachable after assembly and should not compromise CAN routing or antenna placement.

## Open Questions for KiCAD Pin Assignment

- Which ESP32 GPIOs are valid for TWAI TX/RX in the chosen firmware stack?
- Is a USB-UART bridge included, or are programming pads/header sufficient?
- Will auto-reset/auto-boot circuitry be included for flashing?
- Are the OLED pull-ups on the PCB or already present on the display module?
- Are buttons active-low with pull-ups, or active-high with pull-downs?
- Is a transceiver standby or silent mode signal available and useful for this design?
- Does the power subsystem need MCU-controlled enable, reset, or measurement signals?
- Which pins must remain unused to protect boot behavior and module flash operation?
- Are all debug and production-test signals accessible in the intended enclosure?
