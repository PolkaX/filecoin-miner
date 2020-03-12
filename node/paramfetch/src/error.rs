use reqwest;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParamsError {
    #[error("io error:{0}")]
    Io(io::Error),
    #[error("the data for key `{0}` is not available")]
    Object(String),
    #[error("serde_json: {0}")]
    Json(serde_json::Error),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("reqwest error:{0}")]
    Reqwest(reqwest::Error),
    #[error("other err: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("unknown data store error")]
    Unknown,
}

impl From<serde_json::Error> for ParamsError {
    fn from(err: serde_json::Error) -> ParamsError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => ParamsError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => ParamsError::Json(err),
        }
    }
}

impl From<std::io::Error> for ParamsError {
    fn from(err: std::io::Error) -> ParamsError {
        ParamsError::Io(err.into())
    }
}

impl From<reqwest::Error> for ParamsError {
    fn from(err: reqwest::Error) -> ParamsError {
        ParamsError::Reqwest(err.into())
    }
}
