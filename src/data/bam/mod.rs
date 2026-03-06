use crate::errors::AppError;
use std::path::PathBuf;

pub mod single;

pub fn bam_dispatch(files: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    match &files[..] {
        [] => {
            return Err(AppError::UnknownError(
                "provide at least one BAM file".to_string(),
            ));
        }
        [single_bam] => single::parse(single_bam.to_owned(), outfile)?,
        [_, ..] => unreachable!(""),
    }
    Ok(())
}
