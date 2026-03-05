mod args;
mod data;
mod dispatch;
mod errors;

use crate::{args::Args, dispatch::dispatch, errors::AppError};
use clap::Parser;
use simple_logger::SimpleLogger;

fn main() -> Result<(), AppError> {
    SimpleLogger::new().init().expect("");

    let args = Args::parse();
    let _ = dispatch(args)?;

    Ok(())
}
