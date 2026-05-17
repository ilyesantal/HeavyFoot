# Power Subsystem

## Input Source

Power is supplied from the OBD-II connector:

- Battery voltage: pin 16
- Ground: pins 4 and 5

The input must be treated as an unswitched automotive battery rail. It may see cranking dips, alternator ripple, reverse battery events, inductive transients, and load-dump conditions.

## Power Path

```text
OBD-II pin 16
  -> fuse or resettable fuse
  -> reverse polarity protection
  -> TVS diode / transient clamp
  -> input filtering
  -> buck regulator
  -> 3.3 V rail
  -> optional 3.3 V LDO or post-regulator

OBD-II pins 4/5
  -> system ground
```

## Protection Blocks

### Fuse or Resettable Fuse

- Limits fault current from the vehicle battery feed.
- Place close to the OBD-II power input.
- Rating must cover normal ESP32 transmit-current peaks and display load while opening or limiting under downstream faults.

### Reverse Polarity Protection

- Prevents damage if battery polarity is reversed or the connector wiring is incorrect.
- Candidate approaches include series diode, Schottky diode, or P-channel / ideal-diode MOSFET arrangement.
- Selection must account for voltage drop, thermal dissipation, and startup behavior.

### TVS Diode

- Clamps high-energy automotive transients before the regulator input.
- Select for the expected vehicle electrical environment and regulator absolute maximum input voltage.
- Coordinate with fuse behavior and input capacitance so surge energy has a defined path.

### Input Filtering

- Reduces conducted noise into the buck regulator and limits emissions back toward the vehicle harness.
- Expected elements may include bulk capacitance, high-frequency ceramic capacitance, ferrite bead, or LC filtering.
- Filter damping and regulator stability must be checked together.

## Buck Regulator Stage

The buck regulator converts protected vehicle battery voltage to the main low-voltage rail. It should be selected for:

- Automotive input range, including cranking and charging conditions
- Transient tolerance after the protection stage
- Output current margin for ESP32 RF peaks and peripherals
- Efficiency at active and idle loads
- Thermal performance in the enclosure
- Low-noise layout requirements from the regulator datasheet

The expected primary output is 3.3 V unless a higher intermediate rail is later justified.

## Optional 3.3 V LDO / Post-Regulator

An optional post-regulator may be added after the buck when testing shows a need for cleaner local power. Possible uses:

- Separate low-noise rail for ESP32, OLED, or CAN transceiver
- Reduced ripple from buck switching
- Isolation between display current steps and MCU/CAN operation

The dropout voltage, thermal dissipation, quiescent current, and enable sequencing must be checked before adding this stage.

## Expected Load Classes

- ESP32-WROOM-32U: dominant dynamic load; current peaks during Wi-Fi/Bluetooth activity, flash access, and CPU bursts.
- SN65HVD232QD: low-to-moderate 3.3 V load; current depends on CAN bus state and transceiver mode.
- OLED display: variable load; depends on panel type, brightness, and displayed pixels.
- LEDs/buttons: small loads; LEDs require explicit current budgeting, buttons usually negligible except pull-up current.

## Test Points

Provide test access for:

- OBD-II raw battery input after connector
- Protected input after fuse and reverse-polarity stage
- Regulator input after TVS/filtering
- Buck output rail
- Optional post-regulator output rail
- System ground near power input
- System ground near ESP32
- CAN transceiver 3.3 V supply

## Open Component-Selection Questions

- Required input voltage range and transient standard for the target installation.
- Fuse type, hold current, trip/open behavior, and serviceability.
- Reverse-protection topology and acceptable voltage drop.
- TVS working voltage, clamp voltage, surge rating, and package.
- Buck regulator input rating, switching frequency, output current, and thermal margin.
- Whether the optional 3.3 V post-regulator is needed after measurement.
- Required OLED current budget at intended brightness.
- Whether sleep current matters when connected to constant battery voltage.

## Validation Checklist

- Verify correct operation across expected battery voltage range.
- Verify startup during cranking-like input dips.
- Verify no damage under reverse-polarity input.
- Verify transient clamp behavior against regulator input limits.
- Verify buck output ripple under ESP32 peak load and OLED activity.
- Verify thermal rise of protection and regulator components in enclosure.
- Verify CAN transceiver supply remains within specification during bus activity.
- Verify no excessive current draw when vehicle is off.
- Verify test points are accessible on prototype hardware.
