# Architecture

- `amilcar-fs` owns filesystem primitives and does not depend on the engine.
- `amilcar-data` owns parsing and transformation primitives.
- `amilcar-core` composes lower-level crates and exposes the public entry point.
- `amilcar-cli` is an adapter over the library API, not a second implementation.

Stage 1 establishes crate boundaries, shared configuration, error handling, metadata inspection and JSON validation/formatting.
