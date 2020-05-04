pub type Result<T> = std::result::Result<T, StoresError>;
#[derive(thiserror::Error, Debug)]
pub enum StoresError {
    #[error("tmp")]
    Tmp,
    #[error("url parse error:{0}")]
    URLErr(#[from] url::ParseError),
}
