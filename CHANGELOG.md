# Changelog

## Unreleased
- Initial Rust workspace, crate boundaries, shared engine types and CI foundation.
- Filesystem stage 3: bounded file copying with explicit overwrite behavior, path move/rename, directory creation, safe file/empty-directory removal, streaming SHA-256, and duplicate detection.
- CLI commands for copy, move, rename, mkdir, remove, rmdir, hash, and duplicates.
- Added unit tests for copy policies, path operations, safe removal, SHA-256 and duplicate grouping.
