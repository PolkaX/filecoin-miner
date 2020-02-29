pub type Result<T> = std::result::Result<T, MinerError>;

#[derive(Debug, thiserror::Error)]
pub enum MinerError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("datastore error: {0}")]
    Datastore(#[from] datastore::DSError),

    #[error("address error: {0}")]
    Address(#[from] plum_address::AddressError),
}
