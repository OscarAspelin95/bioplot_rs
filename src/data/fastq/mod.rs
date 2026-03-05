pub mod multi;
pub mod single;

use crate::errors::AppError;
use std::path::PathBuf;

pub fn fastq_dispatch(files: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    match &files[..] {
        [] => {
            return Err(AppError::UnknownError(
                "provide at least one file".to_string(),
            ));
        }
        [single_fastq] => single::parse(single_fastq.to_owned(), outfile)?,
        [_, ..] => multi::parse(files, outfile)?,
    }
    Ok(())
}
