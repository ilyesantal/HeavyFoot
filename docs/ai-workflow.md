# AI Workflow

## Purpose

AI assistance may be used for architecture drafting, Rust module design, test generation, documentation, and review. Generated changes must remain small enough to inspect.

## Ground Rules

- Do not implement firmware until the architecture and crate boundaries are agreed.
- Prefer direct CAN/OBD-II designs; do not introduce ELM327 assumptions.
- Keep hardware notes tied to the selected parts: ESP32-WROOM-32U and SN65HVD232QD.
- Keep documents concise and technical.
- Record material architecture choices in `docs/decisions.md`.

## Development Loop

1. State the intended change and affected files.
2. Make a narrow edit.
3. Review the diff.
4. Run available formatting or tests once code exists.
5. Update documentation when decisions or interfaces change.

## Review Checklist

- Does the change preserve host-testable Rust crate boundaries?
- Are hardware-specific details isolated from core logic?
- Are CAN, power, and vehicle-safety assumptions explicit?
- Is the documentation free of marketing language?
- Are future implementation tasks separated from current decisions?
