//! # Setting up logging
//!
//! Logging levels are configured by the CLI depending on the "verbose" flag. Increasing
//! occurrences of this flag will increase the logging level. The default is "warn" and
//! the maximum is "trace". This module sets up the logging display configuration and
//! the output.

use cli_rs::cli::Cli;

use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// Configures the logging according to the CLI configurations.
pub fn setup_logging(cli: &Cli) {
    let level = match cli.verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };

    let log_all = level == Level::DEBUG || level == Level::TRACE;

    let subscriber = FmtSubscriber::builder()
        .pretty()
        .with_max_level(level)
        .with_target(cfg!(debug_assertions))
        .with_line_number(cfg!(debug_assertions))
        .with_file(cfg!(debug_assertions))
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Finished setting up logging configuration!");

    if log_all {
        error!("Example error message!");
        warn!("Example warn message!");
        info!("Example info message!");
        debug!("Example debug message!");
        trace!("Example trace message!");
    }
}
