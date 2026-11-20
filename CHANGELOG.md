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
