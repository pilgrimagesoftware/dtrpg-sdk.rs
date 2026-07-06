# Release Process

Pushes to `master`/`develop` that touch `src/**/*.rs`, `Cargo.toml`, or `.rust-version`
trigger `.github/workflows/build.yaml`, which builds and tests the crate, bumps the patch
version, and packages the crate and docs as workflow artifacts.

On `master` only, a `publish` job then:

- Creates a GitHub Release for the new version, with the packaged `.crate` file and a docs
  archive attached.
- Publishes the crate to [crates.io](https://crates.io/crates/dtrpg-sdk) via `cargo publish`.

A manual major/minor bump is available via the `bump-version` workflow
(`workflow_dispatch`).
