# Changelog

All notable changes to this project are documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- A Cargo workspace with two packages: the `pool` daemon binary in `apps/pool` and the `pool-config` library in `crates/pool-config`. Rust 1.100.0 is pinned by `rust-toolchain.toml`, and `unsafe` code is forbidden in both packages (see spec 0001).
- `pool check-config --config <PATH> [--format human|json]`, an offline command that accepts or rejects a configuration file before you start POOL. It opens no socket, starts no thread and reads at most 1 MiB plus one byte of the file.
- The first configuration contract: `config_version = 1` plus two reserved tables, `result_cache` and `automatic_promotion`. Leaving them out, or setting `enabled = false`, is valid. Asking for either one is refused with `config.unsupported_capability`, because neither capability exists in this build (SEC-10).
- A fixed set of finding codes for rejected files, such as `config.syntax`, `config.schema`, `config.missing_key`, `config.unsupported_version`, `config.too_large` and `config.not_utf8`, with a line and column where the parser gives one. The output is built only from constant text, schema key names and positions, so nothing you put in the file, a secret included, can appear in it (SEC-11).
- A JSON report (`--format json`) that is one line with a fixed field order, for scripts.
- An exit status convention for POOL commands: 0 for success, 1 when the input is rejected, 2 for a usage error, 3 when the command could not finish, for example when the file is missing or cannot be read.
- `pool --help` and `pool --version`.
- Fifteen configuration fixtures and 134 automated tests that cover every acceptance criterion of the first build on the workstation. On Linux 136 tests run, because two of them need `/dev/full`.
- Workspace build notes, acceptance criteria `0001 AC-1` to `AC-13` and a five step build plan in spec 0001, a verification record (`docs/specs/0001-initial-architecture/verify.md`) and an independent review (`docs/reviews/2026-11-14-scope-001-stack-and-workspace.md`). The Linux container build (`0001 AC-2`) passed for `aarch64-unknown-linux-gnu`, the one declared Linux target, and the verification record holds the image digest and the binary checksum.
- Three shared conventions in the engineering standards: configuration keys, exit status with command output, and log event fields with redaction.
