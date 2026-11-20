# Release checklist

AmilcarCore-RS remains in early development. Do not publish a stable release until all required checks below are complete.

## Before a release

- [ ] Review and update \`CHANGELOG.md\`.
- [ ] Confirm the intended semantic version in the workspace manifest.
- [ ] Run \`cargo fmt --all -- --check\`.
- [ ] Run \`cargo check --workspace --all-targets\`.
- [ ] Run \`cargo test --workspace\`.
- [ ] Run \`cargo clippy --workspace --all-targets -- -D warnings\`.
- [ ] Run \`cargo bench --workspace\` and record host/device details for any published measurements.
- [ ] Confirm every GitHub Actions job is green on the exact release commit.
- [ ] Install and exercise the CLI on a physical Termux/Android ARM64 device.
- [ ] Install and exercise the CLI on a supported Linux distribution.
- [ ] Review license files, dependency changes and security notes.
- [ ] Verify README, installation instructions and known limitations.

## Tag and publish

1. Commit release notes and the version change to \`main\`.
2. Wait for the full CI workflow to pass on that exact commit.
3. Create an annotated tag such as \`v0.1.0\` pointing to the verified commit.
4. Publish release notes with installation instructions and known limitations.
5. Attach only binaries built and tested for their stated target. Never label a host binary as an Android/Termux build.

## After publication

- Verify the release page and source archive.
- Test the documented installation command against the tag.
- Record installation issues and start the next changelog entry.
