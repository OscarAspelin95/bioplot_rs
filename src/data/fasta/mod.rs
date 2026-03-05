mod multi;
mod single;

use crate::errors::AppError;
use std::path::PathBuf;

pub fn fasta_dispatch(files: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    match &files[..] {
        [] => {
            return Err(AppError::UnknownError(
                "provide at least one FASTA file".to_string(),
            ));
        }
        [single_fasta] => single::parse(single_fasta.to_owned(), outfile)?,
        [_, ..] => multi::parse(files, outfile)?,
    }
    Ok(())
}
