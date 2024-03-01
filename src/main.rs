//! # action-test-rs
//! "action testing do not use"

use crate::cli::CliArgs;
use crate::error::Error;
use clap::Parser;
use log::{error, info, LevelFilter};

mod cli;
mod error;

const LOG_TARGET: &str = "pass-it-on-cli";

fn main() {
    let args = CliArgs::parse();

    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Off)
        .env()
        .with_module_level(LOG_TARGET, args.verbosity)
        .with_colors(true)
        .init()
        .unwrap();

    if let Err(error) = run(args) {
        error!(target: LOG_TARGET, "{}", error)
    } else {
        info!(target: LOG_TARGET, "Done")
    }
}

fn run(args: CliArgs) -> Result<(), Error> {
    info!(target: LOG_TARGET, "Hello world");
    info!(target: LOG_TARGET, "{:?}", args);

    Ok(())
}
