mod args;
mod data;
mod errors;

use crate::{args::Args, errors::AppError};
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() -> Result<(), AppError> {
    SimpleLogger::new()
        .init()
        .expect("failed to initialize logger.");

    let args = Args::parse();
    data::dispatch(args)?;

    Ok(())
}
