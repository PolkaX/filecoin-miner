#[derive(Debug, thiserror::Error)]
pub enum FileSystemError {
    #[error("sector not found")]
    NotFound,
    #[error("sector already exists")]
    Exists,
    #[error("no suitable path for sector found")]
    NoSuitablePath,
}
