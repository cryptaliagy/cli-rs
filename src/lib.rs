//! # Rust Command Line App
//!
//! This crate defines a binary application with sensible defaults for logging
//! and a basic verbosity option. It is meant to act as a starting point
//! for CLI applications to offer a batteries-mostly-included approach to CI/CD,
//! logging, and documentation.
//!
//! Please see the top level [README.md](https://github.com/taliamax/cli-rs) file
//! for instructions on how to set up this template repository for new projects
//!
//! ## Default Features
//! - **colors**: Causes the logging level of a log message to be coloured depending on its level
//! - **env**: Enables the `clap` feature `env` for pulling values from the environment
//! - **wrap_help**: Enables the `clap` feature `wrap_help` for wrapping help messages for terminal size
//!

pub mod cli;
pub mod log;
