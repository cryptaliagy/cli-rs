use clap::Parser;

use cli_rs::cli::Cli;
use cli_rs::log::setup_logging;

use log::debug;

fn main() {
    let cli = Cli::parse();

    setup_logging(&cli);

    debug!("CLI config: {:?}", &cli);
}
