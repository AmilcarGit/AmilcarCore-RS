# AmilcarCore-RS

**Local Native Processing Engine · Rust**

A modular, local-first Rust toolkit for inspecting, transforming, validating and processing files and structured data. Designed as a reusable library and a command-line application.

> Status: early development — Stage 1 (foundation). APIs may change before a stable release.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `amilcar-core` | Public API, engine, configuration and shared errors |
| `amilcar-fs` | Filesystem inspection and operations |
| `amilcar-data` | Text and structured-data processing |
| `amilcar-cli` | Terminal interface using the library APIs |

## Getting started

Requires the stable Rust toolchain.

```sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
cargo check --workspace
cargo test --workspace
cargo run -p amilcar-cli -- --help
```

## Principles

- Local execution; no remote service or external API required by core workflows.
- Linux, Windows, macOS and Android/Termux where toolchain and target support permit.
- Explicit filesystem operations; no implicit overwrites.
- Buffered processing and configurable limits for large inputs.
- Avoid `unsafe` unless technically necessary and documented.
- Focused dependencies with clear responsibilities.

Planned capabilities include filesystem search/comparison, text and JSON/CSV processing, CLI command groups, benchmarks and cross-platform verification. These are roadmap goals, not claims that every feature is implemented today.

## License

Licensed under either the MIT License or Apache License, Version 2.0, at your option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
