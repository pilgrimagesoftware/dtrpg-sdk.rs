## ADDED Requirements

### Requirement: Publish job gated to master
The CI workflow SHALL run a publish job only when the triggering push is to the `master` branch, and SHALL NOT run crates.io publishing for `develop` pushes or `workflow_dispatch` invocations.

#### Scenario: Push to develop does not publish
- **WHEN** a commit is pushed to the `develop` branch and the build job succeeds
- **THEN** the publish job does not execute `cargo publish` or create a GitHub Release

#### Scenario: Push to master publishes
- **WHEN** a commit is pushed to the `master` branch and the build job succeeds
- **THEN** the publish job executes after the build job completes

### Requirement: GitHub Release created for each published version
The workflow SHALL create a GitHub Release for the version tag produced by the automatic version bump, with the packaged `.crate` file and generated documentation archive attached as release assets.

#### Scenario: Release created with assets
- **WHEN** the publish job runs for a new tag on `master`
- **THEN** a GitHub Release is created for that tag with the `.crate` file and docs archive attached as downloadable assets

### Requirement: Crate published to crates.io
The workflow SHALL run `cargo publish` using a `CRATES_API_KEY` secret to publish the crate to crates.io as part of the `master` release flow.

#### Scenario: Successful publish
- **WHEN** the publish job runs `cargo publish` with a valid `CRATES_API_KEY` secret and the version does not already exist on crates.io
- **THEN** the crate is published to crates.io under the new version

#### Scenario: Duplicate version fails the job
- **WHEN** `cargo publish` is run for a version that already exists on crates.io
- **THEN** the job fails with a non-zero exit code and no GitHub Release/publish state is silently reported as success

### Requirement: Publish ordering avoids inconsistent release state
The workflow SHALL create the GitHub Release and attach assets before running `cargo publish`, so that a release-step failure blocks the crates.io publish rather than leaving crates.io ahead of the GitHub Release.

#### Scenario: Release step fails before publish
- **WHEN** GitHub Release creation or asset upload fails
- **THEN** `cargo publish` does not run for that job execution
