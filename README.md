# AmilcarCore-RS

**Local Native Processing Engine · Rust**

A modular, local-first Rust toolkit for inspecting, transforming, validating and processing files and structured data. Designed as a reusable library and a command-line application.

> Status: early development — Stage 2 (filesystem foundation). APIs may change before a stable release.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `amilcar-core` | Public API, engine, configuration and shared errors |
| `amilcar-fs` | Filesystem inspection, buffered I/O, search and comparison |
| `amilcar-data` | Text and structured-data processing |
| `amilcar-cli` | Terminal interface using the library APIs |

## Installation (Termux / Linux)

See the [visual installation guide](docs/install.html) (HTML with diagrams and copyable commands) or the [complete installation manual](docs/INSTALLATION.md). The installer is available at `scripts/install.sh`; it builds the current CLI locally and installs it into `$HOME/.local/bin` by default. Rust 1.75+ and Cargo are required.

```sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
sh scripts/install.sh
amilcar info
```

To uninstall the CLI binary, run `sh scripts/uninstall.sh` from the repository.

## Getting started

Requires the stable Rust toolchain.

```sh
git clone https://github.com/AmilcarGit/AmilcarCore-RS.git
cd AmilcarCore-RS
cargo check --workspace
cargo test --workspace
cargo run -p amilcar-cli -- --help
```

## Filesystem CLI

```sh
amilcar file inspect ./document.txt
amilcar file diff ./original.bin ./copy.bin
amilcar file find ./documents --extension json
amilcar file find ./documents --name config
```

The recursive finder does not follow symbolic links and has a configurable entry limit. File comparison uses buffered reads and reports the first differing byte offset. File writes are explicit through the library API: `WriteMode::CreateNew` refuses an existing destination; `WriteMode::Overwrite` must be requested to replace it.

## Principles

- Local execution; no remote service or external API required by core workflows.
- Linux, Windows, macOS and Android/Termux where toolchain and target support permit.
- Explicit filesystem operations; no implicit overwrites.
- Buffered processing and configurable limits for large inputs.
- Avoid `unsafe` unless technically necessary and documented.
- Focused dependencies with clear responsibilities.

Planned capabilities include richer text and JSON/CSV processing, complete CLI command groups, benchmarks and cross-platform verification. These are roadmap goals, not claims that every feature is implemented today.

## License

Licensed under either the MIT License or Apache License, Version 2.0, at your option. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
