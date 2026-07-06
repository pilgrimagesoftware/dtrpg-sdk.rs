## Context

`build.yaml` currently runs on every push to `master`/`develop`, bumps the version with `anothrNick/github-tag-action`, commits the bumped `Cargo.toml`, packages the crate (`cargo package`), and uploads the `.crate` file and docs as workflow artifacts (retained 30 days, then deleted). There is no durable release output: no GitHub Release, no crates.io publish. A commented-out `release-plz` step is the only trace of prior intent to publish.

The `bump-version.yaml` workflow provides a manual `workflow_dispatch` path to bump major/minor and push a tag directly, independent of `build.yaml`'s automatic patch bump.

## Goals / Non-Goals

**Goals:**
- Publish every tagged release on `master` to crates.io automatically.
- Create a corresponding GitHub Release with the `.crate` file and docs archive attached, so released artifacts remain accessible past the 30-day workflow-artifact retention window.
- Keep `develop` pushes and manual `workflow_dispatch` runs from publishing to crates.io (avoid polluting the public registry with unreleased or test versions).
- Fail loudly if `cargo publish` rejects a version (e.g. already published), rather than treating it as a soft success.

**Non-Goals:**
- Not changing the version-bump strategy (`anothrNick/github-tag-action` stays as-is).
- Not implementing changelog generation as part of this change (the `debug.yaml` changelog experiment is out of scope).
- Not adding release automation for `dtrpg-sdk/swift` or other sibling SDKs.

## Decisions

- **New `publish` job in `build.yaml` vs. new workflow file**: Add a `publish` job to `build.yaml` that `needs: build` and conditions on `github.ref == 'refs/heads/master'`. Keeping it in the same workflow avoids re-triggering on tag push (which would otherwise double-build) and lets the job directly reuse the `build` job's packaged `.crate` artifact via `actions/download-artifact`.
  - Alternative considered: separate workflow triggered `on: push: tags`. Rejected — the tag is pushed by `build.yaml` itself using `GITHUB_TOKEN`, and by default that does not re-trigger other workflows (GitHub suppresses recursive triggers from `GITHUB_TOKEN`-authored pushes), so a separate tag-triggered workflow would silently never run.
- **Publish gate**: only run `publish` when `github.ref == 'refs/heads/master'` (not `develop`, not manual dispatch on other refs). Matches the repo's git-flow convention where `master` is the release branch.
- **Crates.io token**: set the `CARGO_REGISTRY_TOKEN` env var (the name `cargo publish` reads natively) from the existing `CRATES_API_KEY` GitHub secret, matching the name already referenced in the old commented-out `release-plz` step.
- **GitHub Release creation**: use `softprops/action-gh-release@v2` (widely used, supports multi-file `files:` glob for attaching both the `.crate` and a docs `tar.gz`), tied to the tag produced by the bump step (`${{ steps.bump_version.outputs.new_tag }}` from the existing `build` job, passed via job output).
- **Duplicate-version handling**: run `cargo publish` without any `--dry-run`/ignore-error flags so a duplicate-version rejection fails the job. Do not pre-check crates.io before publishing — `cargo publish`'s own error is sufficient and avoids an extra network round trip with its own failure modes.

## Risks / Trade-offs

- [Recursive publish loop if `GITHUB_TOKEN`-triggered pushes ever start re-triggering workflows] → Mitigation: job is gated on `github.ref == 'refs/heads/master'` AND `needs: build` succeeding in the same run; it does not listen for tag-push events at all.
- [`cargo publish` partially succeeds (uploads to crates.io) but the GitHub Release step fails afterward, leaving crates.io ahead of GitHub] → Mitigation: order steps so `cargo publish` runs last, after the GitHub Release and asset upload succeed; a release-step failure then blocks publish rather than leaving an inconsistent state the other way.
- [Missing or expired `CRATES_API_KEY` secret causes silent auth failure] → Mitigation: `cargo publish` fails the job on auth error by default (non-zero exit); no special handling needed beyond ensuring the secret is configured, which is called out in tasks.md.
- [Docs artifact naming drift between `build` job's `actions/upload-artifact` name and `publish` job's `download-artifact` name] → Mitigation: use the existing `crate-${{ github.sha }}` / `docs-${{ github.sha }}` artifact names already defined in `build.yaml`, referenced by the same expression in `publish`.

## Migration Plan

1. Add `CRATES_API_KEY` secret to the GitHub repo (manual, out-of-band).
2. Add the `publish` job to `build.yaml`.
3. Merge to `develop`, verify the job is skipped (ref check) on a `develop` push.
4. Merge to `master`, verify: tag bump → package → GitHub Release created with assets → `cargo publish` succeeds.
5. No rollback beyond reverting the workflow file; already-published crates.io versions cannot be unpublished (only `cargo yank`), so this is a forward-only change.

## Open Questions

- None outstanding — confirmed the repo's crates.io token secret is named `CRATES_API_KEY`; mapped to `CARGO_REGISTRY_TOKEN` in the `cargo publish` step's `env:` block.
