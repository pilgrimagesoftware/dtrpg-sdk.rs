# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.0.29 (2026-07-07)

### Chore

 - <csr-id-a826d5886828f907a8dad447d0adf89be9d34f71/> update version to 0.0.29 and adjust CI workflow for semantic release

### Bug Fixes

 - <csr-id-0561c0d1bf2937e3d8b9085ed97ed1266fb1d12b/> exclude API/.github from crate packaging
   The API submodule's .github/copilot-instructions.md is a symlink to a
   non-existent AGENTS.md within that directory. cargo package follows
   VCS file listings into the submodule and fails to archive the dangling
   symlink, breaking the release workflow's Package step once the
   submodule pointer advances past commit 58bd246.
 - <csr-id-32f1518023b455236cfc53a473623ff942c95fb5/> remove GitHub authentication step and use secret token for tagging
 - <csr-id-29b1c363e741503fe86e989287514c676a6d1394/> update subproject commit reference in API
 - <csr-id-548271bcb4ddafdbad7a08ca187a0b920f6fdfbe/> add GitHub authentication step for tagging on develop branch
 - <csr-id-e351dfc942379d97abde79cd535e4226cf668834/> name

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Name ([`e351dfc`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e351dfc942379d97abde79cd535e4226cf668834))
    - Update version to 0.0.29 and adjust CI workflow for semantic release ([`a826d58`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a826d5886828f907a8dad447d0adf89be9d34f71))
</details>

## v0.0.28 (2026-07-06)

### New Features

 - <csr-id-56f200e7c040791a1b9d24a136bfd9b1dfb31a49/> add product list item add/delete endpoints
   Bumps the API submodule to pick up the collections/items CRUD contract
   and rounds out ProductListItem support with the two remaining
   operations: POST to add a product to a list, DELETE to remove one.

### Bug Fixes

 - <csr-id-3607af0764bd32f619c1f660543e793bfd1a9bf0/> verify end-to-end workflow for crate release and address pre-existing bugs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #19 from pilgrimagesoftware/feature/product-list-item-endpoints ([`fe95411`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/fe954115ad2d79b204a92e7a95982bc5833b4e4c))
    - Add product list item add/delete endpoints ([`56f200e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/56f200e7c040791a1b9d24a136bfd9b1dfb31a49))
    - Verify end-to-end workflow for crate release and address pre-existing bugs ([`3607af0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3607af0764bd32f619c1f660543e793bfd1a9bf0))
</details>

## v0.0.27 (2026-07-05)

### Chore

 - <csr-id-c3c784a1542689f2a1e9b8a9a9ca222003731fb8/> update CI workflows for build and PR validation with matrix strategy
 - <csr-id-668a17a559dd7e66914ffcb17f2d1bf1dbf8cef2/> update dtrpg-sdk version to 0.0.26
 - <csr-id-62a365615f9b443bf647392cd09e79cdcea570e9/> update dtrpg-sdk version to 0.0.25 and modify tests to use generated passwords
 - <csr-id-17ba64fbb461eb68c201628f4f53ec141a9529e0/> update Cargo.toml on develop with version: 0.0.26 [skip ci]

### Bug Fixes

 - <csr-id-9e2c6ac5debe7734f560e1c398d438a850c76015/> remove invalid crates.io category slug
   crates.io rejected the publish with "The following category slugs are
   not currently supported on crates.io: rust" (400 Bad Request).
   api-bindings alone accurately describes the crate.
 - <csr-id-6a49edf3e77aa1608331f10bc20b63b32d97a3f3/> write docs archive outside the checkout
   cargo publish refuses to run with uncommitted changes present.
   docs-<version>.tar.gz was written straight into the repo root, which
   isn't gitignored, so it showed up as a dirty file and failed the
   publish step. Write it to $RUNNER_TEMP instead.
 - <csr-id-96d75df302dc7e62dfc4309ef38d87c91989d22f/> skip version bump/push on protected master branch
   master's ruleset requires PRs, signed commits, and passing CodeQL, so
   CI's direct push of the Cargo.toml bump commit is rejected. Only bump
   and tag on develop, which already has push access; master just reads
   whatever version merged in from develop via a plain Cargo.toml grep.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Remove invalid crates.io category slug ([`9e2c6ac`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/9e2c6ac5debe7734f560e1c398d438a850c76015))
    - Write docs archive outside the checkout ([`6a49edf`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6a49edf3e77aa1608331f10bc20b63b32d97a3f3))
    - Skip version bump/push on protected master branch ([`96d75df`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/96d75df302dc7e62dfc4309ef38d87c91989d22f))
    - Update CI workflows for build and PR validation with matrix strategy ([`c3c784a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c3c784a1542689f2a1e9b8a9a9ca222003731fb8))
    - Update dtrpg-sdk version to 0.0.26 ([`668a17a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/668a17a559dd7e66914ffcb17f2d1bf1dbf8cef2))
    - Update dtrpg-sdk version to 0.0.25 and modify tests to use generated passwords ([`62a3656`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/62a365615f9b443bf647392cd09e79cdcea570e9))
    - Update Cargo.toml on develop with version: 0.0.26 [skip ci] ([`17ba64f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/17ba64fbb461eb68c201628f4f53ec141a9529e0))
</details>

## v0.0.26 (2026-07-05)

### Chore

 - <csr-id-1f04d86de012aeec36170eeaeee1665e79eb1191/> update verification step to confirm publish job is skipped
 - <csr-id-36a38f37c0629036dd5c9183bceb0e394a6d8ba7/> update Cargo.toml on develop with version: 0.0.25 [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Potential fix for pull request finding 'CodeQL / Hard-coded cryptographic value' ([`a9292c4`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a9292c40f54f74a7954e976e65ae90e2e61f11dc))
    - Update verification step to confirm publish job is skipped ([`1f04d86`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/1f04d86de012aeec36170eeaeee1665e79eb1191))
    - Update Cargo.toml on develop with version: 0.0.25 [skip ci] ([`36a38f3`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/36a38f37c0629036dd5c9183bceb0e394a6d8ba7))
</details>

## v0.0.25 (2026-07-05)

### Chore

 - <csr-id-5ab68c075b71d8981d517ae674e08ef9fd857f90/> update Cargo.toml on develop with version: 0.0.24 [skip ci]

### Refactor

 - <csr-id-1ec02af988f726ea32236a8bbc458abb5026ad60/> split build/release/publish into separate jobs
   anothrNick/github-tag-action is a Docker container action, which
   GitHub Actions only runs on Linux runners; the macOS matrix leg could
   never execute it, and running it on both legs would race on the same
   tag/commit regardless. Move version bump, doc generation, and
   packaging into a single release job (needs: build, ubuntu-latest) that
   runs once. publish and deploy-docs now depend on release instead of
   the matrix build job, and read the tag from its job output directly.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Split build/release/publish into separate jobs ([`1ec02af`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/1ec02af988f726ea32236a8bbc458abb5026ad60))
    - Update Cargo.toml on develop with version: 0.0.24 [skip ci] ([`5ab68c0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/5ab68c075b71d8981d517ae674e08ef9fd857f90))
</details>

## v0.0.24 (2026-07-05)

### Chore

 - <csr-id-b40aa41da1e31d6dd508d381831ea7f72d1a1beb/> update Cargo.toml on develop with version: 0.0.23 [skip ci]

### Bug Fixes

 - <csr-id-ef23fe512f6d191b08b67bd10308bf473cf785cc/> declare cross-compile targets in rust-toolchain.toml
   The project pins rustc via rust-toolchain.toml (1.95.0), which always
   takes precedence over whatever dtolnay/rust-toolchain@stable installs.
   Adding the matrix targets there (not to the unused stable toolchain)
   is what actually makes them auto-install for the pinned toolchain.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Declare cross-compile targets in rust-toolchain.toml ([`ef23fe5`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/ef23fe512f6d191b08b67bd10308bf473cf785cc))
    - Update Cargo.toml on develop with version: 0.0.23 [skip ci] ([`b40aa41`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/b40aa41da1e31d6dd508d381831ea7f72d1a1beb))
</details>

## v0.0.23 (2026-07-05)

### Chore

 - <csr-id-968312409737eda40245008b1bdbd89f3bc03542/> update dtrpg-sdk version to 0.0.21 in Cargo.lock

### Documentation

 - <csr-id-6689a66073255318fdd800a7635fb0583d234b63/> update project title in README.md to "DriveThruRPG SDK (Rust)"

### Bug Fixes

 - <csr-id-a3a5ec0bca6de36aa3caf04e649dcdce36bba8fa/> install the matrix target toolchain explicitly
   macos-latest runners are Apple Silicon (aarch64-apple-darwin), so the
   x86_64-apple-darwin target used by the macOS matrix leg was never
   installed, failing every build with "can't find crate for core".

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update dtrpg-sdk version to 0.0.21 in Cargo.lock ([`9683124`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/968312409737eda40245008b1bdbd89f3bc03542))
    - Install the matrix target toolchain explicitly ([`a3a5ec0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a3a5ec0bca6de36aa3caf04e649dcdce36bba8fa))
    - Update project title in README.md to "DriveThruRPG SDK (Rust)" ([`6689a66`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6689a66073255318fdd800a7635fb0583d234b63))
</details>

## v0.0.22 (2026-07-05)

### Chore

 - <csr-id-90321c474134f3ee304d9b2823bedabc84e6d4cd/> update Cargo.toml on develop with version: 0.0.21 [skip ci]

### Bug Fixes

 - <csr-id-63b2fab339352d383f9ab387ef2f8c787f948393/> stop cross-compiling the macOS matrix leg via Docker
   The macOS leg builds/tests/packages natively for x86_64-apple-darwin
   already, but use-cross: true forced every step through cross's Docker
   toolchain, which fails immediately on the macOS runner. Only the
   Linux/musl leg needs cross.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Stop cross-compiling the macOS matrix leg via Docker ([`63b2fab`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/63b2fab339352d383f9ab387ef2f8c787f948393))
    - Update Cargo.toml on develop with version: 0.0.21 [skip ci] ([`90321c4`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/90321c474134f3ee304d9b2823bedabc84e6d4cd))
</details>

## v0.0.21 (2026-07-05)

### Chore

 - <csr-id-f38bebc042b0bac86e4bcf50e3131a33183a8004/> update version to 0.0.20 in Cargo.lock and Cargo.toml; add RELEASE.md for release process documentation
 - <csr-id-0c6fd32abb61b5b2ae362c217b82e2c4e3189641/> update Cargo.toml on develop with version: 0.0.20 [skip ci]

### Documentation

 - <csr-id-af905ec77f599e26eb0bc449b328ef66104ae260/> update README.md with enhanced installation, quick start, and release process sections

### New Features

 - <csr-id-87134c8f190b34deb6418546bf29d2fb58f7bfc4/> enhance GitHub Actions workflow for crate publishing and documentation release
 - <csr-id-1d4e7d06e26634b47f235bd27243b5c616d728af/> add automated crate release workflow with GitHub Release and crates.io publishing

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version to 0.0.20 in Cargo.lock and Cargo.toml; add RELEASE.md for release process documentation ([`f38bebc`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f38bebc042b0bac86e4bcf50e3131a33183a8004))
    - Update README.md with enhanced installation, quick start, and release process sections ([`af905ec`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/af905ec77f599e26eb0bc449b328ef66104ae260))
    - Enhance GitHub Actions workflow for crate publishing and documentation release ([`87134c8`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/87134c8f190b34deb6418546bf29d2fb58f7bfc4))
    - Add automated crate release workflow with GitHub Release and crates.io publishing ([`1d4e7d0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/1d4e7d06e26634b47f235bd27243b5c616d728af))
    - Update Cargo.toml on develop with version: 0.0.20 [skip ci] ([`0c6fd32`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/0c6fd32abb61b5b2ae362c217b82e2c4e3189641))
</details>

## v0.0.20 (2026-07-05)

### Chore

 - <csr-id-e442b84698e13ff90f684082f9325c6a61b255d7/> checkpoint
 - <csr-id-fabed0a7c34f56427bccbe41747b0efba81ba704/> checkpoint
 - <csr-id-0428035341d4e2d40cf5caafe9dc2e060601ab07/> update ignore file
 - <csr-id-0774283845c16f911c9fb2344eb04566eace577d/> update Cargo.toml on develop with version: 0.0.19 [skip ci]

### Documentation

 - <csr-id-f32830b8d547f975b38ce84f7adfc160f0337041/> add a comment

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Checkpoint ([`e442b84`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e442b84698e13ff90f684082f9325c6a61b255d7))
    - Checkpoint ([`fabed0a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/fabed0a7c34f56427bccbe41747b0efba81ba704))
    - Merge pull request #13 from pilgrimagesoftware/feature/replace-api-key-entry-with-credential-login ([`5a3e8fa`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/5a3e8fa5f19262ad889b99506d29e0ea2fafa7ec))
    - Add a comment ([`f32830b`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f32830b8d547f975b38ce84f7adfc160f0337041))
    - Update ignore file ([`0428035`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/0428035341d4e2d40cf5caafe9dc2e060601ab07))
    - Merge branch 'develop' into feature/replace-api-key-entry-with-credential-login ([`e921994`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e92199457dae878f49a3ccb04207fe60ef6c3120))
    - Update Cargo.toml on develop with version: 0.0.19 [skip ci] ([`0774283`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/0774283845c16f911c9fb2344eb04566eace577d))
</details>

## v0.0.19 (2026-07-04)

### Chore

 - <csr-id-e2e51e670be92caca58b2be24a9513a77ed521e2/> update lock file
 - <csr-id-79aef39215ca896d4cef0e28e935d0bba2c9485d/> bump actions/checkout from 6 to 7 in /.github/workflows
   Bumps [actions/checkout](https://github.com/actions/checkout) from 6 to 7.
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v6...v7)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-version: '7'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-fb8c1b2aa5cc6dcc190d76f5bf7dc0d77f671633/> clean up dependabot file
 - <csr-id-ba590fb00fa0aa5ddaab82558d8a27be10e101cd/> update PR workflow to use toolchain action
 - <csr-id-227de07cafc761cbb3dbcf2d092e550ccb76e142/> update Cargo.toml on develop with version: 0.0.18 [skip ci]

### Documentation

 - <csr-id-f1e9d20d8ee723b6ca8b2c84cfcc7c5ebd3d14fc/> spec
 - <csr-id-34ccbbb366d5307fdf4e796994409d3df3faa315/> add credential-login-exchange openspec change
   Child change of the dtrpg umbrella proposal
   replace-api-key-entry-with-credential-login, scoping the SDK-level
   website credential exchange (validate_login_credentials.php,
   create_account_app.php) behind login_with_credentials.

### New Features

 - <csr-id-e3d2c6c2025a61b21cd7b15412973996fff4603f/> credential login exchange

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 1 day passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #12 from pilgrimagesoftware/feature/replace-api-key-entry-with-credential-login ([`65a4a71`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/65a4a71084748e0a1077e9f515ec7f3166fa2253))
    - Credential login exchange ([`e3d2c6c`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e3d2c6c2025a61b21cd7b15412973996fff4603f))
    - Spec ([`f1e9d20`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f1e9d20d8ee723b6ca8b2c84cfcc7c5ebd3d14fc))
    - Add credential-login-exchange openspec change ([`34ccbbb`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/34ccbbb366d5307fdf4e796994409d3df3faa315))
    - Merge pull request #8 from pilgrimagesoftware/dependabot/cargo/serde_json-1.0.150 ([`3833953`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3833953171a2aee6a80c70132640a4424a747e4e))
    - Merge pull request #9 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/kiyoon/changelog-action-2.0.2 ([`bf28562`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/bf28562b4c017122a73e5744e2a5716db9f62e88))
    - Merge pull request #10 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/YunaBraska/git-info-action-3.0.3 ([`9bc3756`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/9bc375622f241557d32168a96b91bc11b51e9316))
    - Merge pull request #11 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/actions/checkout-7 ([`e2e17ab`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e2e17ab9514545fe51c0ed0e1474f5d72a6ef927))
    - Update lock file ([`e2e51e6`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e2e51e670be92caca58b2be24a9513a77ed521e2))
    - Bump actions/checkout from 6 to 7 in /.github/workflows ([`79aef39`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/79aef39215ca896d4cef0e28e935d0bba2c9485d))
    - Clean up dependabot file ([`fb8c1b2`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/fb8c1b2aa5cc6dcc190d76f5bf7dc0d77f671633))
    - Update PR workflow to use toolchain action ([`ba590fb`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/ba590fb00fa0aa5ddaab82558d8a27be10e101cd))
    - Update Cargo.toml on develop with version: 0.0.18 [skip ci] ([`227de07`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/227de07cafc761cbb3dbcf2d092e550ccb76e142))
</details>

## v0.0.18 (2026-07-02)

### Chore

 - <csr-id-410462078e5e5b739770dc611da02f0f6b8aea6c/> bump dtrpg-sdk version to 0.0.16
   Reformat imports for consistency.
 - <csr-id-e87c4a8a1e7741028bdebe628526acac7311f680/> update Cargo.toml on develop with version: 0.0.17 [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump dtrpg-sdk version to 0.0.16 ([`4104620`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/410462078e5e5b739770dc611da02f0f6b8aea6c))
    - Update Cargo.toml on develop with version: 0.0.17 [skip ci] ([`e87c4a8`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e87c4a8a1e7741028bdebe628526acac7311f680))
</details>

## v0.0.17 (2026-07-02)

### Chore

 - <csr-id-c0cd7bdfd5b4c01fbb58118d267b5ed4d317b171/> update Cargo.toml on develop with version: 0.0.16 [skip ci]

### New Features

 - <csr-id-6044dda9023675e026a42fc96759ab8607f8950a/> Add JSON:API relationship support and sideloaded resources
   Add support for JSON:API `relationships` and `included` array handling
   in ordered product responses. Introduce `OrderProductRelationships`,
   `RelationshipRef`, `RelationshipData`, and `IncludedItem` types with
   helper methods for decoding sideloaded Publisher and Product resources.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add JSON:API relationship support and sideloaded resources ([`6044dda`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6044dda9023675e026a42fc96759ab8607f8950a))
    - Update Cargo.toml on develop with version: 0.0.16 [skip ci] ([`c0cd7bd`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c0cd7bdfd5b4c01fbb58118d267b5ed4d317b171))
</details>

## v0.0.16 (2026-07-02)

### Chore

 - <csr-id-f52deee69f9692cbfda7b15d1691d2fef6de757d/> update Cargo.toml on develop with version: 0.0.15 [skip ci]

### New Features

 - <csr-id-44e4c563eb24cc05ea6f8f02e1e79f71d65cf8ed/> Add embedded product, publisher, and order metadata types
   Add three new structs to support embedded metadata on ordered products:
   - OrderProductPublisher: Publisher name, ID, and slug
   - OrderProductInfo: Product catalog metadata including cover images
   - OrderProductDescription: Product descriptive text
   - OrderProductOrder: Order summary metadata
   
   These are added as optional fields to OrderProductAttributes and
   exported from the library module.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add embedded product, publisher, and order metadata types ([`44e4c56`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/44e4c563eb24cc05ea6f8f02e1e79f71d65cf8ed))
    - Update Cargo.toml on develop with version: 0.0.15 [skip ci] ([`f52deee`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f52deee69f9692cbfda7b15d1691d2fef6de757d))
</details>

## v0.0.15 (2026-07-01)

### Chore

 - <csr-id-4a597e7e81a31dd3736106717900c8ecd035f054/> update version

### Refactor

 - <csr-id-e35c9c536a64fdc83b8a8399fd88b7e8a3011b7c/> format code for consistency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Format code for consistency ([`e35c9c5`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/e35c9c536a64fdc83b8a8399fd88b7e8a3011b7c))
    - Update version ([`4a597e7`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/4a597e7e81a31dd3736106717900c8ecd035f054))
</details>

## v0.0.14 (2026-06-30)

### Chore

 - <csr-id-ad06fcf2d54fd8896c419b43dd5305f618932382/> add delete
 - <csr-id-4568426d652d8adb5efcacef7795dabb7af545db/> checkpoint
 - <csr-id-3d11c1715c184c7fa7014fdc8a4b96c3e8f84ddb/> update Cargo.toml on develop with version: 0.0.13 [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add delete ([`ad06fcf`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/ad06fcf2d54fd8896c419b43dd5305f618932382))
    - Checkpoint ([`4568426`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/4568426d652d8adb5efcacef7795dabb7af545db))
    - Update Cargo.toml on develop with version: 0.0.13 [skip ci] ([`3d11c17`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3d11c1715c184c7fa7014fdc8a4b96c3e8f84ddb))
</details>

## v0.0.13 (2026-06-30)

### Chore

 - <csr-id-9a0c5d8fe2e9af519d95ab51571918e50aae4516/> more updates
 - <csr-id-88046f4e17a406a8cb873c95e49cf62d80e65e0c/> archived specs
 - <csr-id-32a8ee7e5ff6193a84d73b21d38e073372377716/> update Cargo.toml on develop with version: 0.0.12 [skip ci]

### Bug Fixes

 - <csr-id-5466cefc60eb558a05dd8e37f15c3f098ccc99ba/> library
 - <csr-id-2fd33b0f8bc36939635d862343bf5bc9a8e850c6/> auth header

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - More updates ([`9a0c5d8`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/9a0c5d8fe2e9af519d95ab51571918e50aae4516))
    - Archived specs ([`88046f4`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/88046f4e17a406a8cb873c95e49cf62d80e65e0c))
    - Library ([`5466cef`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/5466cefc60eb558a05dd8e37f15c3f098ccc99ba))
    - Auth header ([`2fd33b0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/2fd33b0f8bc36939635d862343bf5bc9a8e850c6))
    - Update Cargo.toml on develop with version: 0.0.12 [skip ci] ([`32a8ee7`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/32a8ee7e5ff6193a84d73b21d38e073372377716))
</details>

## v0.0.12 (2026-06-26)

### Chore

 - <csr-id-06df969e0c52f9214f641b6df60ef9fc464c1339/> auth flow
 - <csr-id-4ceaf0c6511fedef50bef523d0809b7ca22a822d/> update submodule
 - <csr-id-153f33e607e3af30fb561957cdefdbed9d9e2af7/> link CLAUDE.md to AGENTS.md
 - <csr-id-b1ce7d5509f092180f428568895076aed10a7231/> update API submodule
 - <csr-id-7f3906ca0712f034dbe0b888b559f7d464574bcf/> update .gitignore
 - <csr-id-33ec47a5b0fc3ab527ebac2951165e5a3a772e7f/> bump YunaBraska/git-info-action in /.github/workflows
   Bumps [YunaBraska/git-info-action](https://github.com/yunabraska/git-info-action) from 3.0.2 to 3.0.3.
   - [Release notes](https://github.com/yunabraska/git-info-action/releases)
   - [Commits](https://github.com/yunabraska/git-info-action/compare/3.0.2...3.0.3)
   
   ---
   updated-dependencies:
   - dependency-name: YunaBraska/git-info-action
     dependency-version: 3.0.3
     dependency-type: direct:production
     update-type: version-update:semver-patch
   ...
 - <csr-id-9f0b342ca8be7b229519991d2b89df0fe2505cbe/> bump kiyoon/changelog-action in /.github/workflows
   Bumps [kiyoon/changelog-action](https://github.com/kiyoon/changelog-action) from 2.0.1 to 2.0.2.
   - [Release notes](https://github.com/kiyoon/changelog-action/releases)
   - [Changelog](https://github.com/kiyoon/changelog-action/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/kiyoon/changelog-action/compare/v2.0.1...v2.0.2)
   
   ---
   updated-dependencies:
   - dependency-name: kiyoon/changelog-action
     dependency-version: 2.0.2
     dependency-type: direct:production
     update-type: version-update:semver-patch
   ...
 - <csr-id-988bd0678ff38ae9d50a3713094a9b5a0c073876/> bump serde_json from 1.0.149 to 1.0.150
   Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.149 to 1.0.150.
   - [Release notes](https://github.com/serde-rs/json/releases)
   - [Commits](https://github.com/serde-rs/json/compare/v1.0.149...v1.0.150)
   
   ---
   updated-dependencies:
   - dependency-name: serde_json
     dependency-version: 1.0.150
     dependency-type: direct:production
     update-type: version-update:semver-patch
   ...
 - <csr-id-f1e28c88dd93ea8efa09aa3a788b414b60471bb0/> bump actions/upload-artifact in /.github/workflows
   Bumps [actions/upload-artifact](https://github.com/actions/upload-artifact) from 4 to 7.
   - [Release notes](https://github.com/actions/upload-artifact/releases)
   - [Commits](https://github.com/actions/upload-artifact/compare/v4...v7)
   
   ---
   updated-dependencies:
   - dependency-name: actions/upload-artifact
     dependency-version: '7'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-2cc98309d0de67951189c304de3bc43aa03ba386/> bump YunaBraska/git-info-action in /.github/workflows
   Bumps [YunaBraska/git-info-action](https://github.com/yunabraska/git-info-action) from 3.0.1 to 3.0.2.
   - [Release notes](https://github.com/yunabraska/git-info-action/releases)
   - [Commits](https://github.com/yunabraska/git-info-action/compare/3.0.1...3.0.2)
   
   ---
   updated-dependencies:
   - dependency-name: YunaBraska/git-info-action
     dependency-version: 3.0.2
     dependency-type: direct:production
     update-type: version-update:semver-patch
   ...
 - <csr-id-390955168d43efcac9743f399d1daf566c6b0dd4/> bump actions/checkout from 4 to 6 in /.github/workflows
   Bumps [actions/checkout](https://github.com/actions/checkout) from 4 to 6.
   - [Release notes](https://github.com/actions/checkout/releases)
   - [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
   - [Commits](https://github.com/actions/checkout/compare/v4...v6)
   
   ---
   updated-dependencies:
   - dependency-name: actions/checkout
     dependency-version: '6'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-40dc16e38edd75785d19b0ca5a9c9dd8229f6c27/> bump actions/upload-pages-artifact in /.github/workflows
   Bumps [actions/upload-pages-artifact](https://github.com/actions/upload-pages-artifact) from 3 to 5.
   - [Release notes](https://github.com/actions/upload-pages-artifact/releases)
   - [Commits](https://github.com/actions/upload-pages-artifact/compare/v3...v5)
   
   ---
   updated-dependencies:
   - dependency-name: actions/upload-pages-artifact
     dependency-version: '5'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-ee9c7bb2595bc28ccd7d38a40042654d643eefc9/> bump actions/deploy-pages from 4 to 5 in /.github/workflows
   Bumps [actions/deploy-pages](https://github.com/actions/deploy-pages) from 4 to 5.
   - [Release notes](https://github.com/actions/deploy-pages/releases)
   - [Commits](https://github.com/actions/deploy-pages/compare/v4...v5)
   
   ---
   updated-dependencies:
   - dependency-name: actions/deploy-pages
     dependency-version: '5'
     dependency-type: direct:production
     update-type: version-update:semver-major
   ...
 - <csr-id-8574d790c5f828dea1a754b123855118543ac2f4/> formatting
 - <csr-id-8041c3eccbb2b524e359eeb11214ba053a3bd877/> add GHA to dependabot
 - <csr-id-f79e4581620577cf6c446efab4288727cef4956a/> fix reqwest
 - <csr-id-83cd3b384bbb93116dcb5121689ba06f1a599460/> replace Rust version file with toolchain spec
 - <csr-id-923924ae9a0bcd402cadba4fdadb3c7284e89c31/> bump reqwest from 0.12.28 to 0.13.3
   Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.12.28 to 0.13.3.
   - [Release notes](https://github.com/seanmonstar/reqwest/releases)
   - [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
   - [Commits](https://github.com/seanmonstar/reqwest/compare/v0.12.28...v0.13.3)
   
   ---
   updated-dependencies:
   - dependency-name: reqwest
     dependency-version: 0.13.3
     dependency-type: direct:production
     update-type: version-update:semver-minor
   ...
 - <csr-id-c9f1890fde8c3dcd4951bc1cada0a4117d669db9/> update lock file
 - <csr-id-c862eb79ef7a4e0cbdc92c8f72e0547f5ea4d3a2/> update submodule
 - <csr-id-4bf9cf7a19321493aa1a440ea5fead4b7cc6a151/> update README
 - <csr-id-6ec122ef3be00426b28e4b05d3745a954f80af0b/> update Cargo.toml on develop with version: 0.0.11 [skip ci]

### New Features

 - <csr-id-baa9a8524b58fd7d483c77047a32632281ee65d9/> add the code
 - <csr-id-a5eb7baf9af338069947abf8f121c695a3da9961/> add client spec

### Bug Fixes

 - <csr-id-a8a4eb1cca536b50b4e08d56bfc0a9dea93ae060/> check out submodules

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release.
 - 57 days passed between releases.
 - 25 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Auth flow ([`06df969`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/06df969e0c52f9214f641b6df60ef9fc464c1339))
    - Update submodule ([`4ceaf0c`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/4ceaf0c6511fedef50bef523d0809b7ca22a822d))
    - Link CLAUDE.md to AGENTS.md ([`153f33e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/153f33e607e3af30fb561957cdefdbed9d9e2af7))
    - Update API submodule ([`b1ce7d5`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/b1ce7d5509f092180f428568895076aed10a7231))
    - Update .gitignore ([`7f3906c`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/7f3906ca0712f034dbe0b888b559f7d464574bcf))
    - Bump YunaBraska/git-info-action in /.github/workflows ([`33ec47a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/33ec47a5b0fc3ab527ebac2951165e5a3a772e7f))
    - Bump kiyoon/changelog-action in /.github/workflows ([`9f0b342`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/9f0b342ca8be7b229519991d2b89df0fe2505cbe))
    - Merge pull request #7 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/actions/upload-artifact-7 ([`d105ce3`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/d105ce304b72011c1fd198f5c7367fa847d5b72b))
    - Merge pull request #6 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/YunaBraska/git-info-action-3.0.2 ([`d95a169`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/d95a169ed536767b266de5adaea95eed3de96cbe))
    - Merge pull request #4 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/actions/upload-pages-artifact-5 ([`0bd18e1`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/0bd18e1e9a698e29aa081bc67cca0f7b37fa97f8))
    - Merge pull request #3 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/actions/deploy-pages-5 ([`d0d8fd6`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/d0d8fd68486375b3b46dd66eda64fd9aef16cc76))
    - Merge pull request #5 from pilgrimagesoftware/dependabot/github_actions/dot-github/workflows/actions/checkout-6 ([`474d3af`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/474d3af018d73bbaf9f2b5f2ee9b0952a7834de8))
    - Bump serde_json from 1.0.149 to 1.0.150 ([`988bd06`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/988bd0678ff38ae9d50a3713094a9b5a0c073876))
    - Bump actions/upload-artifact in /.github/workflows ([`f1e28c8`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f1e28c88dd93ea8efa09aa3a788b414b60471bb0))
    - Bump YunaBraska/git-info-action in /.github/workflows ([`2cc9830`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/2cc98309d0de67951189c304de3bc43aa03ba386))
    - Bump actions/checkout from 4 to 6 in /.github/workflows ([`3909551`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/390955168d43efcac9743f399d1daf566c6b0dd4))
    - Bump actions/upload-pages-artifact in /.github/workflows ([`40dc16e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/40dc16e38edd75785d19b0ca5a9c9dd8229f6c27))
    - Bump actions/deploy-pages from 4 to 5 in /.github/workflows ([`ee9c7bb`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/ee9c7bb2595bc28ccd7d38a40042654d643eefc9))
    - Formatting ([`8574d79`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/8574d790c5f828dea1a754b123855118543ac2f4))
    - Add GHA to dependabot ([`8041c3e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/8041c3eccbb2b524e359eeb11214ba053a3bd877))
    - Check out submodules ([`a8a4eb1`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a8a4eb1cca536b50b4e08d56bfc0a9dea93ae060))
    - Fix reqwest ([`f79e458`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f79e4581620577cf6c446efab4288727cef4956a))
    - Merge pull request #2 from pilgrimagesoftware/dependabot/cargo/reqwest-0.13.3 ([`782501c`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/782501cdf0d096b5915f781efb03d7b79c919dd6))
    - Replace Rust version file with toolchain spec ([`83cd3b3`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/83cd3b384bbb93116dcb5121689ba06f1a599460))
    - Bump reqwest from 0.12.28 to 0.13.3 ([`923924a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/923924ae9a0bcd402cadba4fdadb3c7284e89c31))
    - Add the code ([`baa9a85`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/baa9a8524b58fd7d483c77047a32632281ee65d9))
    - Add client spec ([`a5eb7ba`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a5eb7baf9af338069947abf8f121c695a3da9961))
    - Update lock file ([`c9f1890`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c9f1890fde8c3dcd4951bc1cada0a4117d669db9))
    - Update submodule ([`c862eb7`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c862eb79ef7a4e0cbdc92c8f72e0547f5ea4d3a2))
    - Update README ([`4bf9cf7`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/4bf9cf7a19321493aa1a440ea5fead4b7cc6a151))
    - Update Cargo.toml on develop with version: 0.0.11 [skip ci] ([`6ec122e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6ec122ef3be00426b28e4b05d3745a954f80af0b))
</details>

## v0.0.11 (2026-04-30)

### Chore

 - <csr-id-17300d8d4cd9edf8ac52d8bfa225a2a202efbcce/> update code, spec, and tests
 - <csr-id-79464b9dc738c38fdc15166bef311d5ba5eb95ab/> update Cargo.toml on develop with version: 0.0.10 [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 7 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update code, spec, and tests ([`17300d8`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/17300d8d4cd9edf8ac52d8bfa225a2a202efbcce))
    - Update Cargo.toml on develop with version: 0.0.10 [skip ci] ([`79464b9`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/79464b9dc738c38fdc15166bef311d5ba5eb95ab))
</details>

## v0.0.10 (2026-04-23)

### Chore

 - <csr-id-0b816fd2e08d2ee418e823fd7040baa16d173a6b/> comment out release-plz action in build workflow
 - <csr-id-b99154ff8a2015bbdc22602cd18f84ac0c93dc71/> update Cargo.toml on develop with version: 0.0.9 [skip ci]

### New Features

 - <csr-id-41896ff79e83b4119e9ce75bd25ab232419159ce/> rename GitHub Actions workflow for build and release process

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Comment out release-plz action in build workflow ([`0b816fd`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/0b816fd2e08d2ee418e823fd7040baa16d173a6b))
    - Rename GitHub Actions workflow for build and release process ([`41896ff`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/41896ff79e83b4119e9ce75bd25ab232419159ce))
    - Update Cargo.toml on develop with version: 0.0.9 [skip ci] ([`b99154f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/b99154ff8a2015bbdc22602cd18f84ac0c93dc71))
</details>

## v0.0.9 (2026-04-23)

### Chore

 - <csr-id-4ffd1f6f3ef3c90b69258e04d14a1f3a6570cf58/> update Cargo.toml on develop with version: 0.0.8 [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update Cargo.toml on develop with version: 0.0.8 [skip ci] ([`4ffd1f6`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/4ffd1f6f3ef3c90b69258e04d14a1f3a6570cf58))
</details>

## v0.0.8 (2026-04-22)

### Chore

 - <csr-id-6df5e1c7b0b68c88fdcf0d4197c91d0883b43998/> update commit message format for Cargo.toml version update in CI workflow

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update commit message format for Cargo.toml version update in CI workflow ([`6df5e1c`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6df5e1c7b0b68c88fdcf0d4197c91d0883b43998))
    - Merge pull request #1 from pilgrimagesoftware/fix/workflow-perms-2 ([`c16acf3`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c16acf320bf9e6c479c3cbc1c33df79deada60d8))
    - Potential fix for code scanning alert no. 1: Workflow does not contain permissions ([`202259f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/202259f5dddb5d4114dfa83f73f64ab731a8759d))
    - Update Cargo.toml on develop with version: 0.0.7 [skip ci] ([`f68418f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/f68418ff07596685a1476a265fafabeb1416f792))
</details>

## v0.0.7 (2026-04-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update Cargo.toml on develop with version: 0.0.6 [skip ci] ([`561569a`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/561569a36d622eae566cf5c23ae345a59a7d17b4))
</details>

## v0.0.6 (2026-04-22)

### Bug Fixes

 - <csr-id-436fe187564b48494ec524d77e20723d68773910/> update CI workflow to fetch all tags and improve checkout step

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update CI workflow to fetch all tags and improve checkout step ([`436fe18`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/436fe187564b48494ec524d77e20723d68773910))
</details>

## v0.0.5 (2026-04-22)

### New Features

 - <csr-id-75875813581ac3c3fc8ba64aad73002343fd2fb3/> add documentation generation and deployment steps to CI workflow
 - <csr-id-3869aeede5fc593d6433ecc637697cfb314a9e4e/> enhance documentation for SDK components including authentication, configuration, and error handling

### Bug Fixes

 - <csr-id-65fba5dcb393def66572b16416a36b3d89055e9d/> reorder package step in CI workflow to allow dirty builds

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add documentation generation and deployment steps to CI workflow ([`7587581`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/75875813581ac3c3fc8ba64aad73002343fd2fb3))
    - Enhance documentation for SDK components including authentication, configuration, and error handling ([`3869aee`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3869aeede5fc593d6433ecc637697cfb314a9e4e))
    - Reorder package step in CI workflow to allow dirty builds ([`65fba5d`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/65fba5dcb393def66572b16416a36b3d89055e9d))
</details>

## v0.0.4 (2026-04-22)

### Bug Fixes

 - <csr-id-a528102a77a8f58289bfffa685e646c11d862a15/> add permissions section for CI workflow

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add permissions section for CI workflow ([`a528102`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a528102a77a8f58289bfffa685e646c11d862a15))
</details>

## v0.0.3 (2026-04-22)

### New Features

 - <csr-id-176084fc1f7496333c81d1dc804fb9a387106f8b/> add .release-plz.toml configuration for workspace settings

### Bug Fixes

 - <csr-id-d6b3f5e47cb863d152927980b6f323b332b91c4a/> allow dirty package builds in CI workflow
 - <csr-id-1107391d67c3ab0996a24cc782fbff548ee0d74e/> update release-plz action version to 0.5.17 in CI workflow

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow dirty package builds in CI workflow ([`d6b3f5e`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/d6b3f5e47cb863d152927980b6f323b332b91c4a))
    - Update release-plz action version to 0.5.17 in CI workflow ([`1107391`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/1107391d67c3ab0996a24cc782fbff548ee0d74e))
    - Add .release-plz.toml configuration for workspace settings ([`176084f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/176084fc1f7496333c81d1dc804fb9a387106f8b))
    - Update Cargo.toml on develop with version: 0.0.2 [skip ci] ([`3b544f9`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3b544f98e9c3fc6f379d83fd5916b0f5daf7ed36))
</details>

## v0.0.2 (2026-04-22)

### New Features

 - <csr-id-63e462fbbef832b729bf6cd510a4e75c97145cda/> add CODE_OF_CONDUCT, CONTRIBUTING, RELEASE, and SECURITY documents
 - <csr-id-3240c0f8cfffa867384eff30e4fefc612a48bab9/> update package metadata in Cargo.toml for dtrpg-sdk

### Bug Fixes

 - <csr-id-3112deab3db06a761ab83b7e984ffd1aa69456b9/> update package name and version in Cargo.lock for dtrpg-sdk

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update package name and version in Cargo.lock for dtrpg-sdk ([`3112dea`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3112deab3db06a761ab83b7e984ffd1aa69456b9))
    - Add CODE_OF_CONDUCT, CONTRIBUTING, RELEASE, and SECURITY documents ([`63e462f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/63e462fbbef832b729bf6cd510a4e75c97145cda))
    - Update package metadata in Cargo.toml for dtrpg-sdk ([`3240c0f`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/3240c0f8cfffa867384eff30e4fefc612a48bab9))
    - Update Cargo.toml on develop with version: 0.0.1 [skip ci] ([`1bb0f38`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/1bb0f38f81b58e8d02773a7fafafcd1474f4970f))
</details>

## v0.0.1 (2026-04-22)

### Chore

 - <csr-id-05156a046392b7e9727c76751254354e6713bdd7/> add .rust-version file for Rust SDK version management
 - <csr-id-52935c38eb5c1cbf4f127098296a473b8edb19d1/> rename license file

### New Features

 - <csr-id-692c3dc3e2b957c0292ca4638f4349190d645fca/> enhance CI workflow with cross-compilation and automated version bumping
 - <csr-id-50d8975f4c4b1844c6670ea94a04791dacbef6e6/> add workflow for bumping version with SemVer options
 - <csr-id-53751baf2a9951520159367a3a7eacabfb798658/> add issue templates for bug reports and feature requests
 - <csr-id-a19beec332c8392f4c9e46fb6db753b7594db044/> rename CI and PR workflow files
 - <csr-id-5d7f0f719d9294d1512b4c740b549044b9bb7769/> add Dependabot configuration for automated cargo updates
 - <csr-id-982ead6345d1194b9b7f8097f0855585a5de232e/> add debug workflow for GitHub Actions to manage versioning and changelog generation
 - <csr-id-04f5b97cd2c5a49707ebd43b782a3111c077cbf6/> add CI and PR validation workflows for Rust SDK
 - <csr-id-78057190d42857e3a56c9f828ee6efa5b596cdc6/> implement authentication and session management in Rust SDK
 - <csr-id-6387c90e4b80feaaaf3afa0d878398651e14932a/> add Rust SDK specifications for authentication and session management

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Enhance CI workflow with cross-compilation and automated version bumping ([`692c3dc`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/692c3dc3e2b957c0292ca4638f4349190d645fca))
    - Add workflow for bumping version with SemVer options ([`50d8975`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/50d8975f4c4b1844c6670ea94a04791dacbef6e6))
    - Add issue templates for bug reports and feature requests ([`53751ba`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/53751baf2a9951520159367a3a7eacabfb798658))
    - Rename CI and PR workflow files ([`a19beec`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a19beec332c8392f4c9e46fb6db753b7594db044))
    - Add Dependabot configuration for automated cargo updates ([`5d7f0f7`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/5d7f0f719d9294d1512b4c740b549044b9bb7769))
    - Add debug workflow for GitHub Actions to manage versioning and changelog generation ([`982ead6`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/982ead6345d1194b9b7f8097f0855585a5de232e))
    - Add .rust-version file for Rust SDK version management ([`05156a0`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/05156a046392b7e9727c76751254354e6713bdd7))
    - Add CI and PR validation workflows for Rust SDK ([`04f5b97`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/04f5b97cd2c5a49707ebd43b782a3111c077cbf6))
    - Implement authentication and session management in Rust SDK ([`7805719`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/78057190d42857e3a56c9f828ee6efa5b596cdc6))
    - Add Rust SDK specifications for authentication and session management ([`6387c90`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/6387c90e4b80feaaaf3afa0d878398651e14932a))
    - Rename license file ([`52935c3`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/52935c38eb5c1cbf4f127098296a473b8edb19d1))
    - Rename project from dtrpg-client to dtrpg-sdk ([`76aed03`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/76aed03b96a729a6c85d0dfd5129c0dae8d4b41a))
    - Rename and update README for client library ([`c4d89e1`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/c4d89e169b9c294e0ffc35e51be3c41002fcd544))
    - Initial commit ([`a893baf`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/commit/a893baf06f35bbfc56b699fc11871c23ee8ee7be))
</details>

