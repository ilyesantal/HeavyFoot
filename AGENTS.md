# AGENTS.md

## Project Philosophy

This project prioritizes:
- clean architecture
- learning value
- explicit abstractions
- low overhead
- host-testable logic
- long-term maintainability

Do not optimize for rapid feature growth.

## Rust Guidelines

- Prefer simple and explicit code.
- Avoid unnecessary generics and macro-heavy abstractions.
- Avoid heap allocation unless justified.
- Shared crates should remain `no_std` compatible where practical.
- Prefer stack allocation and fixed-capacity buffers.
- Avoid async unless a clear benefit exists.

## Architecture Rules

- Hardware-independent logic must remain outside the ESP32 firmware crate.
- Protocol parsing must be testable on the host.
- Do not mix HAL calls with domain logic.
- Avoid putting business logic in `main.rs`.

## Dependencies

- Minimize dependencies.
- Prefer well-known embedded crates.
- Do not introduce large frameworks without justification.

## Testing

- Add unit tests for parsing and calculations.
- Prefer deterministic tests.
- Host tests should not require ESP32 hardware.

## Style

- Prefer small modules with clear responsibilities.
- Prefer composition over complex inheritance-style patterns.
- Keep APIs easy to navigate.
- Add comments explaining *why*, not *what*.

## When implementing code

Before implementing:
1. Explain the design briefly.
2. Identify tradeoffs.
3. Keep the first implementation simple.
