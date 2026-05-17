## ADR-005: Shared crates should be no_std-compatible

Decision:
Core crates should avoid heap allocation and remain compatible with `no_std`.
Host-side tests may enable `std` conditionally.

Reason:
- Easier migration to embedded targets
- Lower overhead
- Better control over allocations and timing
- More portable architecture

Consequence:
- Avoid unnecessary allocations and async runtimes early
- Prefer fixed-capacity data structures
- Some libraries may not be usable
