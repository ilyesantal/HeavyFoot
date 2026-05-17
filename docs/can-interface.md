# CAN Interface

This document describes the hardware CAN interface for the ESP32-WROOM-32U OBD-II fuel cost monitor. It does not define a schematic.

## OBD-II Connector

Use the standard OBD-II CAN pins:

- Pin 6: CAN-H
- Pin 14: CAN-L

The design should treat the vehicle CAN bus as an existing terminated bus. The device is a node connected through the OBD-II connector, not the main bus terminator.

## CAN Transceiver

The SN65HVD232QD provides the physical layer interface between the ESP32 and the vehicle CAN bus:

- CAN-H/CAN-L side connects to the protected OBD-II bus lines.
- TXD/RXD side connects to ESP32 TWAI TX/RX GPIOs.
- Supply is 3.3 V.
- Ground must share the device ground reference from the OBD-II connector.

The ESP32 TWAI peripheral supplies CAN controller behavior. The SN65HVD232QD is only the bus transceiver; it does not implement OBD-II, filtering, retries, or higher-level protocol handling.

## Protection and Filtering

- Add CAN-H/CAN-L ESD protection close to the OBD-II connector.
- Consider an optional common-mode choke if EMC testing, harness length, or installation noise requires it.
- Keep protection and filtering compatible with CAN signal integrity at the selected bus speed.

## Termination

Do not populate a default 120 ohm termination resistor across CAN-H/CAN-L for normal vehicle OBD-II use. The vehicle bus should already be terminated.

An optional DNP resistor footprint or jumper-selectable 120 ohm termination may be included for bench testing with an isolated CAN setup. It must be clearly marked so it is not accidentally enabled in a vehicle.

## Test Points

Provide accessible test points for:

- CAN-H
- CAN-L
- TXD between ESP32 and SN65HVD232QD
- RXD between SN65HVD232QD and ESP32
- CAN transceiver 3V3 supply
- Local GND near the transceiver

Test points should support oscilloscope probing without disturbing CAN routing or connector-side protection.

## Layout Notes

- Keep CAN-H/CAN-L routing short from the OBD-II connector to protection and transceiver.
- Place ESD protection near the connector before long internal routing.
- Route CAN-H/CAN-L as a close pair with consistent spacing.
- Avoid stubs, sharp discontinuities, and unnecessary vias on CAN-H/CAN-L.
- Keep noisy switching regulator nodes away from CAN-H/CAN-L and TXD/RXD.
- Place transceiver decoupling close to the SN65HVD232QD supply pins.
