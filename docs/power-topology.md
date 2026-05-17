# Power Topology

Detailed power requirements and validation criteria are maintained in `docs/power-subsystem.md`.

## Current Topology Assumption

```text
OBD-II pin 16
  -> input protection
  -> input filtering
  -> buck regulator
  -> 3.3 V rail
  -> optional 3.3 V post-regulator
```

OBD-II pins 4 and 5 provide the device ground reference.

## Notes

- The main required rail is 3.3 V for ESP32-WROOM-32U, SN65HVD232QD, OLED, buttons, and status LED.
- A higher intermediate rail is not selected yet.
- A 3.3 V post-regulator remains optional and should be justified by measurement, noise, or thermal requirements.
- Component values and part numbers are intentionally open until schematic design.

## Open Topology Questions

- Should the buck regulator generate 3.3 V directly?
- Is an intermediate rail needed for efficiency, thermal margin, or future peripherals?
- Is low sleep current a first-prototype requirement?
- Does any peripheral require a rail other than 3.3 V?

## Candidate Component Families

### Buck Regulator

Primary candidate:

- TI LM5164-Q1
  - Automotive-qualified synchronous buck
  - Wide input-voltage margin
  - Suitable for robust 12 V vehicle input designs
  - Candidate output: 5 V intermediate rail

Alternative candidates:

- TI LM63610-Q1
- TI LM53601-Q1
- Infineon S6BP202A buck-boost, if deep-cranking operation becomes a priority

### 3.3 V Post-Regulator

Candidate:

- Infineon TLS208D1EJV33
  - Reused from previous EDFC design
  - Used after 5 V buck stage
  - Not used directly from vehicle battery because of LDO heat dissipation

### Input Protection

Planned blocks:

- Fuse or resettable fuse after OBD pin 16
- Reverse polarity protection using MOSFET-based ideal-diode style circuit
- High-power automotive TVS diode to ground
- Input bulk capacitor and ceramic decoupling
- Optional ferrite/EMI filtering before buck regulator

### TVS Candidate Family

Candidate family:

- Littelfuse SM8S or SLD8S automotive/high-power TVS family

Exact standoff voltage and package must be selected together with the buck regulator absolute maximum input rating and desired load-dump/transient behavior.

## Current Recommendation

Use a 5 V automotive buck regulator followed by the TLS208D1EJV33 3.3 V LDO/post-regulator.

Preferred robust buck candidate:

- TI LM5164-Q1

Reason:

- Large input-voltage margin
- 1 A output class is enough for the planned 5 V intermediate rail
- More robust against vehicle transients than a minimal 36 V-only buck design

Tradeoff:

- Potentially larger/more expensive than simpler 36 V automotive buck regulators
