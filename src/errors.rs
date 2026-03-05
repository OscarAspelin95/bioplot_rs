use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unknown error: {0}")]
    UnknownError(String),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    BioUtilsError(#[from] bio_utils_rs::errors::BioError),
}
