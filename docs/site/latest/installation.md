---
title: Installation
version: latest
slug: installation
---

# Installation

Add the crate:

```bash
cargo add dtrpg-sdk
```

## Building from source

The repository uses the `dtrpg-api` repository as a git submodule (`API/`). `build.rs` reads
`API/openapi.yaml` at compile time and generates Rust OpenAPI metadata, exposed through
`dtrpg_sdk::openapi`. Clone with submodules, or initialize them after cloning:

```bash
git clone --recursive https://github.com/pilgrimagesoftware/dtrpg-sdk.rs.git

# or, if already cloned:
git submodule update --init --recursive
```
