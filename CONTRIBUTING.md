# Contributing

Discuss substantial API or architecture changes in an issue first.

Run these checks before submitting:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Keep crates focused, document public APIs, test behavior and avoid unnecessary dependencies.
