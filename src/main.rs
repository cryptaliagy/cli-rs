mod logging;

use clap::Parser;

use cli_rs::cli::Cli;
use logging::setup_logging;
use tracing::debug;

fn main() {
    let cli = Cli::parse();

    setup_logging(&cli);

    debug!("CLI config: {:?}", &cli);
}
