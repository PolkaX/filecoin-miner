use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SealWorkerError {
    #[error("io error:{0}")]
    Io(io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("serde_json: {0}")]
    Json(serde_json::Error),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

impl From<serde_json::Error> for SealWorkerError {
    fn from(err: serde_json::Error) -> SealWorkerError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => SealWorkerError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => SealWorkerError::Json(err),
        }
    }
}

impl From<std::io::Error> for SealWorkerError {
    fn from(err: std::io::Error) -> SealWorkerError {
        SealWorkerError::Io(err.into())
    }
}
