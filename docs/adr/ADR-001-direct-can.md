## ADR-001: Use direct CAN instead of ELM327

Decision: Use ESP32 TWAI + external CAN transceiver.

Reason:
- Better learning value
- Full control over OBD-II protocol
- Cleaner firmware architecture
- No dependency on ELM327 clone behavior

Consequence:
- More firmware work
- Need CAN/OBD mocks for host testing
- Hardware must include proper CAN transceiver and protection
