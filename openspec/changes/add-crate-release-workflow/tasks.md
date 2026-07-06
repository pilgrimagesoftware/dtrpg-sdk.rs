## 1. Prerequisites

- [x] 1.1 Confirm a `crates.io` API token exists for the `dtrpg-sdk` crate owner and add it to the GitHub repo as the `CRATES_API_KEY` secret. (Secret already exists under this name.)

## 2. Workflow changes

- [x] 2.1 Add a `publish` job to `.github/workflows/build.yaml` with `needs: build` and a condition on `github.ref == 'refs/heads/master'`.
- [x] 2.2 In the `publish` job, download the `crate-${{ github.sha }}` and `docs-${{ github.sha }}` artifacts produced by the `build` job.
- [x] 2.3 Package the docs directory into a `tar.gz` archive for release attachment.
- [x] 2.4 Add a step using `softprops/action-gh-release@v2` to create a GitHub Release, attaching the `.crate` file and docs archive. Deviation from plan: instead of reading the tag from `steps.bump_version.outputs.new_tag` (a matrix-job output, which is ambiguous across the `build` job's Linux/macOS legs), the `publish` job checks out the repo post-bump and reads the version directly from the committed `Cargo.toml`.
- [x] 2.5 Add a step running `cargo publish --verbose` with the `CARGO_REGISTRY_TOKEN` env var set from the `CRATES_API_KEY` secret, placed after the release-creation step.
- [x] 2.6 Remove the commented-out `release-plz` step now superseded by the explicit release/publish steps.

## 3. Verification

- [ ] 3.1 Push a commit to `develop` and confirm the `publish` job is skipped.
- [ ] 3.2 Push/merge to `master` and confirm: tag bump succeeds, GitHub Release is created with both assets attached, and `cargo publish` succeeds against crates.io.
- [ ] 3.3 Re-run the workflow against an already-published version (or simulate via `cargo publish --dry-run` locally) to confirm duplicate-version failures surface as a failed job rather than a silent skip.
