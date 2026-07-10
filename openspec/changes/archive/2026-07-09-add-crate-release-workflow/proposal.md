## Why

The `build.yaml` workflow already bumps the crate version, packages a `.crate` file, and builds docs on every push to `master`/`develop`, but it stops short of publishing anything. There is no GitHub Release and no `crates.io` publish step (the `release-plz` step is commented out), so every version bump today produces build artifacts nobody can install via `cargo add dtrpg-sdk`.

## What Changes

- Add a release job that runs after `build.yaml`'s tag bump, triggered only on `master` (or on the `push` tag created by `anothrNick/github-tag-action`).
- Create a GitHub Release for the newly pushed tag, attaching the packaged `.crate` file and generated docs archive as release assets.
- Publish the crate to `crates.io` using `cargo publish`, gated behind the `CRATES_API_KEY` secret (matching the existing commented-out reference), mapped to the `CARGO_REGISTRY_TOKEN` env var that `cargo publish` reads natively.
- Skip publish on `develop` pushes and `workflow_dispatch` runs unless explicitly targeting `master`, so pre-release/test builds never land on crates.io.
- Fail the workflow (not silently skip) if the crate version being published already exists on crates.io, to avoid confusing duplicate-version errors.

## Capabilities

### New Capabilities
- `crate-release-publishing`: Automated GitHub Release creation and crates.io publishing triggered by version tags on the SDK crate.

### Modified Capabilities
(none — `build.yaml` job wiring changes are implementation, not a spec-level requirements change to an existing capability)

## Impact

- `.github/workflows/build.yaml`: add a `publish` job (or new workflow file) depending on `build`.
- Requires a `CRATES_API_KEY` secret in the GitHub repo settings (crates.io API token).
- Uses existing `GITHUB_TOKEN` permissions (`contents: write`) already granted in `build.yaml` for release creation.
- No changes to SDK source code, only CI/CD configuration.
