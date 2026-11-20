# Architecture

AmilcarCore-RS is a local-first Rust workspace. The CLI is an adapter over reusable crates; it does not duplicate filesystem or data-processing logic.

## Crates

- \`amilcar-core\` exposes the engine entry point, configuration and shared error/result types.
- \`amilcar-fs\` implements bounded file reads, explicit writes, metadata inspection, recursive search, comparison, filesystem operations, streaming SHA-256 and duplicate grouping.
- \`amilcar-data\` implements UTF-8 text statistics/newline normalization, JSON validation/formatting and CSV validation/inspection/formatting/column selection.
- \`amilcar-cli\` exposes the \`amilcar\` executable and maps command arguments to library calls. Its optional \`--json\` flag emits machine-readable successful results.

## Data flow

\`\`\`mermaid
flowchart TD
    U[User or script] --> CLI[amilcar CLI]
    CLI --> CORE[amilcar-core]
    CLI --> FS[amilcar-fs]
    CLI --> DATA[amilcar-data]
    CORE --> FS
    DATA --> TEXT[Text]
    DATA --> JSON[JSON]
    DATA --> CSV[CSV]
\`\`\`

## Safety properties

- Text, JSON and CSV CLI reads are limited to 64 MiB and detect growth beyond the limit while reading.
- Output files are created without replacement unless the caller passes \`--overwrite\`.
- Recursive discovery does not follow symbolic links and has a configurable entry limit.
- File hashing and comparison use buffered/streaming I/O.
- CSV parsing requires consistent record widths by default.
- The documented core operations run locally and do not call remote APIs or background services.

## Performance and platform verification

The \`cargo bench --workspace\` programs measure representative filesystem and data-processing operations. Results depend on the host and are informational; CI does not enforce absolute throughput targets. CI checks Linux, Windows and macOS and performs a compile-time check for the Android ARM64 target. Physical Termux testing remains a separate release-readiness task.
