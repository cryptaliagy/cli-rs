//! # Setting up logging
//!
//! Coloured output is available by default with the `colors` feature, but can be disabled
//! when installing if using the `--no-default-features` flag.

use crate::cli::Cli;

#[cfg(feature = "colors")]
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn, LevelFilter};

/// Configures the logging according to the CLI configurations. This
/// enables coloured output (if using `colors` feature), and formats the
/// output to include the file and line (if running in `debug` profile).
pub fn setup_logging(cli: &Cli) {
    #[cfg(feature = "colors")]
    let colors = ColoredLevelConfig::new()
        .trace(Color::BrightMagenta)
        .debug(Color::Cyan)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let level = match cli.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let log_all = level == LevelFilter::Debug || level == LevelFilter::Trace;

    let mut log_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            #[cfg(feature = "colors")]
            let level = colors.color(record.level());

            #[cfg(not(feature = "colors"))]
            let level = record.level();

            if cfg!(debug_assertions) {
                out.finish(format_args!(
                    "[{}][{}][{}:{}][{}]\t{}",
                    chrono::Utc::now().format("%Y-%m-%d - %H:%M:%S"),
                    record.target(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    level,
                    message,
                ))
            } else {
                out.finish(format_args!(
                    "[{}][{}][{}]\t{}",
                    chrono::Utc::now().format("%Y-%m-%d - %H:%M:%S"),
                    record.target(),
                    level,
                    message,
                ))
            }
        })
        .level(level)
        .chain(std::io::stdout());

    if !log_all {
        log_config = log_config.filter(|metadata| metadata.target().starts_with("cli_rs"))
    }

    log_config.apply().unwrap();

    trace!("Finished setting up logging configuration!");

    if log_all {
        error!("Example error message!");
        warn!("Example warn message!");
        info!("Example info message!");
        debug!("Example debug message!");
        trace!("Example trace message!");
    }
}
