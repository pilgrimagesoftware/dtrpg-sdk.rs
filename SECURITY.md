# Security Policy

## Supported Versions

This crate is pre-1.0. Only the latest version published on
[crates.io](https://crates.io/crates/dtrpg-sdk) receives security fixes. Please upgrade before reporting
an issue to confirm it still reproduces.

## Reporting a Vulnerability

Do not open a public issue for security vulnerabilities.

Report privately via GitHub's
[Security Advisories](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/security/advisories/new). This
keeps the report confidential until a fix is released.

Include, where possible:

- Affected version(s)
- A minimal reproduction or proof of concept
- Impact (e.g. what an attacker can do, what data or systems are exposed)

You should receive an initial response as soon as possible. If the report is confirmed, we'll work
with you on a fix and coordinate a disclosure timeline before any public advisory is published. Reporters
are credited in the advisory unless they ask to remain anonymous.

## Scope

This policy covers the `dtrpg-sdk` Rust crate and its published GitHub Actions workflows. Vulnerabilities
in dependencies should be reported upstream; if a dependency issue affects this crate directly (e.g. no
fix available, requires a workaround here), report it here as well.
