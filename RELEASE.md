# Release process

## Summary

1. Trigger the [**Prepare Release**](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/actions/workflows/prepare-release.yaml) workflow.
1. Merge PR to `master`.
1. Merge `master` into `develop`.

## Breakdown

Instead of manually bumping `Cargo.toml` and writing the changelog by hand on the release branch, a `prepare-release` workflow does it for you:

```sh
git checkout develop
git pull
```

Trigger the [**Prepare Release**](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/actions/workflows/prepare-release.yaml) workflow (`workflow_dispatch` in the Actions tab). It:

1. Runs `git-cliff --bump` against `develop` to determine the next version (e.g. `0.3.0`) from commits since the last tag.
2. Updates `Cargo.toml` to that version.
3. Prepends the generated changelog section to `CHANGELOG.md`.
4. Opens a PR from an auto-created `release/0.3.0` branch into `master`.

You review the PR (catch anything that shouldn't ship, fix as needed) — the [**CI**](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/actions/workflows/ci.yaml) workflow runs against it — merge into `master`, then merge the same changes back into `develop`. Merging into `master` triggers tagging — either as a manual step or via a follow-up workflow step that tags on merge:

```sh
git checkout master
git pull
git tag -a v0.3.0 -m "Release 0.3.0"
git push origin v0.3.0
```

The tag push triggers the [**Release**](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/actions/workflows/release.yaml) workflow, which builds, publishes to crates.io, generates the changelog scoped to that tag, and attaches it to the GitHub Release.

## Triggering the run

```sh
gh workflow run prepare-release.yaml
```

Or trigger it directly from the [Prepare Release workflow page](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/actions/workflows/prepare-release.yaml).
