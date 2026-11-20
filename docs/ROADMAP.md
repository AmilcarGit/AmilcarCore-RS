# Roadmap

This roadmap separates implementation from device-level validation. A checked implementation stage does not imply that every operating system or physical device has been manually tested.

## Completed implementation stages

- [x] Stage 1 — workspace foundation, crate boundaries, engine types, configuration, shared errors and initial CI.
- [x] Stage 2 — filesystem foundation and safe operations: buffered reads, explicit writes, metadata inspection, bounded recursive search, byte comparison, copy/move/rename, directory operations, SHA-256 and duplicate detection.
- [x] Stage 3 — local text, JSON and CSV processing: UTF-8 text statistics, newline normalization, JSON validation/pretty/compact formatting, CSV validation/inspection/formatting and column selection.
- [x] Stage 4 — CLI command groups for file, text, JSON and CSV, with machine-readable \`--json\` output and safe output-file defaults.
- [x] Stage 5 — reproducible benchmark executables for filesystem hashing/comparison and text/JSON/CSV processing, without a benchmark-framework dependency.
- [x] Stage 6 — CI quality gates, Linux/Windows/macOS tests, Android ARM64 target check, installation instructions, architecture guide and release checklist.

## Remaining before stable 1.0

- [ ] Manual device validation — install and exercise the build on real Termux/Android ARM64 devices and supported Linux distributions.
- [ ] Stable 1.0 release — finalize API compatibility policy, versioning, release artifacts and signed release notes.

## Validation commands

\`\`\`sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo bench --workspace
\`\`\`

Benchmarks report measurements and do not enforce fixed throughput thresholds. The Android CI job is a target-specific compile check; it does not replace testing on a physical Android device.
