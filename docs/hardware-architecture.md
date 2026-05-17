# Hardware Architecture

## Block Diagram

```text
OBD-II connector
  |-- CAN-H/CAN-L --> protection/filtering --> SN65HVD232QD --> ESP32 TWAI
  |-- 12 V input ----> protection -----------> buck regulator --> 3.3 V rail
                                                   |
                                                   +--> optional 3.3 V LDO/post-regulator

ESP32-WROOM-32U
  |-- I2C --> OLED display
  |-- GPIO --> button 1
  |-- GPIO --> button 2
```

## CAN Interface

- Use ESP32 TWAI peripheral for direct CAN frames.
- Use SN65HVD232QD as the 3.3 V CAN transceiver.
- Provide CAN-H/CAN-L ESD protection close to the connector.
- Consider common-mode choke and split termination only after measuring target installation needs.
- Do not include ELM327 hardware or AT-command protocol dependencies.

## Power Input

The device is powered from OBD-II pin 16 with ground from pins 4 and 5. The power path uses protected automotive input filtering followed by a buck regulator and optional 3.3 V post-regulation.

Detailed protection, rail, load, and validation requirements are maintained in `docs/power-subsystem.md`.

## ESP32 Module

- Target module: ESP32-WROOM-32U.
- External antenna connector requires enclosure and antenna placement review.
- GPIO assignment must preserve boot strapping requirements.
- TWAI pins, I2C pins, and button pins should be documented before PCB layout.

## Display and Buttons

- OLED uses I2C to minimize pin count.
- Buttons connect to GPIOs with hardware or internal pull-ups.
- Button inputs should include debounce handling in firmware; add simple RC filtering only if testing shows a need.
