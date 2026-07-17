
## 1.0.1 - 2026-07-11

### Fixed
- Decode the included sideload array on order product detail responses



## 1.0.0 - 2026-07-10

### Documentation
- Propose prepare-download-file-index


### Fixed
- Require an index parameter for prepare_download **[BREAKING]**



## 0.1.3 - 2026-07-09

### Added
- Better docs and examples, code reorg


### Changed
- Reorganize the code


### Documentation
- Change spec


### Fixed
- Bump version
- Remove extra output line in changelog template
- Restore API submodule pointer to develop tip
- Surface real API error messages instead of decode failures
- Decode the JSON:API envelope on product list create responses



## Unreleased

### Fixed
- Restore API submodule pointer to develop tip


## 0.1.2 - 2026-07-09

### Added
- Better docs and examples, code reorg


### Changed
- Reorganize the code


### Documentation
- Change spec


### Fixed
- Bump version
- Remove extra output line in changelog template
- Restore API submodule pointer to develop tip
- Surface real API error messages instead of decode failures
- Decode the JSON:API envelope on product list create responses



## Unreleased

### Fixed
- Restore API submodule pointer to develop tip


## 0.1.0 - 2026-07-08

### Added
- Better docs and examples, code reorg


### Changed
- Reorganize the code


### Documentation
- Change spec


### Fixed
- Bump version
- Remove extra output line in changelog template
- Restore API submodule pointer to develop tip
- Surface real API error messages instead of decode failures
- Decode the JSON:API envelope on product list create responses


## 0.1.0 - 2026-07-08

### Added
- Better docs and examples, code reorg


### Changed
- Reorganize the code


### Fixed
- Bump version
- Remove extra output line in changelog template
- Prepare release error when it fails
- Workflow authentication
- Authenticate with crates.io
- Authenticate with crates.io
- Restore API submodule pointer to develop tip




### Added
- Add Rust SDK specifications for authentication and session management
- Implement authentication and session management in Rust SDK
- Add CI and PR validation workflows for Rust SDK
- Add debug workflow for GitHub Actions to manage versioning and changelog generation
- Add Dependabot configuration for automated cargo updates
- Rename CI and PR workflow files
- Add workflow for bumping version with SemVer options
- Enhance CI workflow with cross-compilation and automated version bumping
- Update package metadata in Cargo.toml for dtrpg-sdk
- Add CODE_OF_CONDUCT, CONTRIBUTING, RELEASE, and SECURITY documents
- Add .release-plz.toml configuration for workspace settings
- Enhance documentation for SDK components including authentication, configuration, and error handling
- Add documentation generation and deployment steps to CI workflow
- Rename GitHub Actions workflow for build and release process
- Add client spec
- Add the code
- Add embedded product, publisher, and order metadata types
- Add JSON:API relationship support and sideloaded resources
- Credential login exchange
- Add automated crate release workflow with GitHub Release and crates.io publishing
- Enhance GitHub Actions workflow for crate publishing and documentation release
- Add product list item add/delete endpoints
- Add release job to CI workflow for automated cargo releases
- Add CI and release workflows with changelog configuration
- Better docs and examples, code reorg


### Changed
- Format code for consistency
- Update release workflow to use tags and simplify steps
- Reorganize the code


### Documentation
- Add credential-login-exchange openspec change
- Spec
- Add a comment
- Update README.md with enhanced installation, quick start, and release process sections
- Update project title in README.md to "DriveThruRPG SDK (Rust)"
- Update vulnerability reporting policy


### Fixed
- Update package name and version in Cargo.lock for dtrpg-sdk
- Update release-plz action version to 0.5.17 in CI workflow
- Allow dirty package builds in CI workflow
- Add permissions section for CI workflow
- Reorder package step in CI workflow to allow dirty builds
- Update CI workflow to fetch all tags and improve checkout step
- Auth header
- Library
- Remove invalid crates.io category slug
- Verify end-to-end workflow for crate release and address pre-existing bugs
- Update subproject commit reference in API
- Remove GitHub authentication step and use secret token for tagging
- Name
- Run cargo install quietly in CI workflow
- Update GIT_AUTHOR_IDENT to use a fixed author name for cargo release
- Move GIT_AUTHOR_IDENT to env section for consistency in CI workflow
- Remove GIT_AUTHOR_IDENT from env section and set author details in release step
- Add continuation to commit prefix in cargo release command
- Update Cargo release step to use generated GitHub token for authentication
- Update GitHub App token credentials in build workflow
- Add token generation step for GitHub App in release workflow
- Set GITHUB_TOKEN for Cargo release step
- Update cargo-edit installation action to use version 2
- Update prepare-release workflow to use correct action versions and improve documentation
- Update prepare-release workflow to use correct token syntax and improve documentation
- Bump version
- Remove extra output line in changelog template


## 0.0.0 - 2026-01-19

