use crate::errors::AppError;
use std::path::PathBuf;

pub mod multi;
pub mod single;

pub fn bam_dispatch(files: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    match &files[..] {
        [] => return Err(AppError::UnknownError("provide at least one BAM file".to_string())),
        [_] => unreachable!(""),
        [_, ..] => multi::parse(files, outfile)?,
    }
    Ok(())
}
