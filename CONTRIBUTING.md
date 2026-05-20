# Contributing code

## 📢 Conventional Commits 📢

To enhance our development workflow, enable automated changelog generation, and pave the way for Continuous Delivery,
the `dtrpg-sdk.rs` project has adopted the [Conventional Commits
standard](https://www.conventionalcommits.org/en/v1.0.0/) for all commit messages.

Going forward, all commits to this repository **MUST** adhere to the Conventional Commits standard. Commits not adhering
to this standard will cause the CI build to fail. PRs will not be merged if they include non-conventional commits.

A git pre-commit hook may be installed to validate your conventional commit messages before pushing them to GitHub by
running `bin/setup` in the project root.

Read more about this change in the [Commit Message Guidelines section of
CONTRIBUTING.md](CONTRIBUTING.md#commit-message-guidelines)
