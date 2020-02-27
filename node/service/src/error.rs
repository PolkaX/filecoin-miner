pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Io error
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Input error
    #[error("input error: {0}")]
    Input(String),
    /// Other type error.
    #[error("other err: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl std::convert::From<String> for Error {
    fn from(s: String) -> Error {
        Error::Input(s)
    }
}
