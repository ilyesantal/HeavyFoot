# Protocol Notes

## Current supported protocol scope

- OBD-II Mode 01
- ISO 15765-4 over classic CAN
- Standard 11-bit functional request ID: 0x7DF
- ECU response IDs: 0x7E8..=0x7EF
- Single-frame responses only for now

## Supported PIDs

| PID | Name | Formula |
|---|---|---|
| 0x00 | Supported PIDs 01-20 | bitfield |
| 0x0D | Vehicle speed | A km/h |
| 0x10 | MAF air flow rate | ((A * 256) + B) / 100 g/s |

## Out of scope for now

- ISO-TP multi-frame responses
- Mode 03 diagnostic trouble codes
- vehicle-specific PIDs
- ECU write operations
