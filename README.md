# Rust Command Line Interface

This is a template repository meant to be used as a starting point for developing a new Rust-based CLI. This handles most of the key bootstrapping elements that I require when writing a new CLI.

## How to use

1. Create a new repository templated from this repo
1. Change the package name in the `Cargo.toml` file
1. Update `src/main` to use the new crate name
1. Update `.github/workflows/release.yml` to use the new binary name

## Features

### CLI

1. Logging configuration with the `fern` crate
1. Initial base CLI struct with count-based verbosity
1. `manfile` generation on build based on `clap` CLI command

### CI/CD

1. PR / Testing pipeline with:
   - Security scanning
   - Linting with Clippy
   - Cargo tests (debug)
1. A binary release pipeline for main branch (controlled through environment):
   - Target can be set through environment variable in pipeline
   - Major & Minor version tracked through `VERSION` file
   - Patch version tracked through pipeline. Actual binary/crate version set at release time.
   - `manfile` published alongside binary
   - Manually-triggered release pipeline (can be configured to auto-deploy from main)
   - Publishing to `crates.io` via commented-out section
