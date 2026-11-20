# Changelog

## Unreleased

- Added UTF-8 text statistics and CRLF/CR newline normalization.
- Added JSON validation, pretty formatting and compact serialization.
- Added CSV validation, inspection, formatting and exact-header column selection.
- Expanded the \`amilcar\` CLI with \`text\`, \`json\` and \`csv\` command groups.
- Added global \`--json\` output for successful command results.
- Added CLI integration tests for machine-readable output, text statistics, CSV selection and destination safety.
- Added standalone filesystem and data-processing benchmark programs without a benchmark framework dependency.
- Expanded CI to run formatting, all-target checks, tests and Clippy, plus Linux/Windows/macOS and Android ARM64 target checks.
- Added architecture, roadmap and release preparation documentation.
- Retained local-only processing and create-new output behavior unless \`--overwrite\` is explicitly supplied.

- Standardized streaming I/O on 64 KiB buffers for file copy, hashing and comparison.
- Hardened bounded text reads against files growing after the initial metadata check.
- Refused symbolic-link targets in explicit overwrite writes and added a Unix regression test.
- Optimized duplicate detection by filtering unique file sizes before hashing.
- Added benchmark execution to the CI quality job.
