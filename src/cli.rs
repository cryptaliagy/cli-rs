//! # Command Line Definition
//!
//! This module of the crate includes the definition of the command line args that should
//! be accepted. It uses [`clap`](https://docs.rs/clap/latest/clap/) and the `derive`
//! feature to achieve this.
//!
//! The [`env`](https://docs.rs/clap/latest/clap/_features/index.html) feature of `clap`
//! is also enabled, so arguments can instead pull from the environment. See the `clap`
//! documentation for more information.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// An example CLI that includes man page generation, configured logging
/// that shows files if in debug, and verbosity filter.
pub struct Cli {
    /// Increases logging verbosity, up to max of 3
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}
