use crate::errors::AppError;
use std::path::PathBuf;

pub fn parse(_bam_path: PathBuf, _outfile: Option<PathBuf>) -> Result<(), AppError> {
    unreachable!("single BAM not yet implemented")
}
