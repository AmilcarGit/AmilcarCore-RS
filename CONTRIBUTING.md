# Contributing

Discuss substantial API or architecture changes in an issue first.

## Development checks

Run these checks before submitting:

\`\`\`sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
\`\`\`

Run the informational benchmark programs with:

\`\`\`sh
cargo bench --workspace
\`\`\`

## Contribution guidelines

- Keep crates focused and maintain the dependency direction documented in \`docs/ARCHITECTURE.md\`.
- Document public APIs and test normal, boundary and failure behavior.
- Preserve Rust 1.75 compatibility unless the declared MSRV is deliberately changed.
- Do not introduce network calls into local core operations.
- Do not overwrite user files unless the command exposes an explicit opt-in flag.
- Treat benchmark results as measurements, not guaranteed performance claims.
