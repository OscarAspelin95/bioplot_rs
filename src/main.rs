mod args;
mod data;
mod errors;

use crate::{args::Args, errors::AppError};
use clap::Parser;

fn main() -> Result<(), AppError> {
    let args = Args::parse();
    data::dispatch(args)?;

    Ok(())
}
