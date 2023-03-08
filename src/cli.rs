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
