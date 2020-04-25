use plum_address::Address;
use plum_crypto::CryptoError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("empty ProofInput")]
    EmptyProofInput,
    #[error("miner {0} is possibly slashed")]
    MaybeSlashed(Address),
    #[error("`{0}` has no miner power")]
    NoMiningPower(Address),
    #[error("api error{0}")]
    ApiError(#[from] api::ApiError),
    #[error("tipset error {0}")]
    TipsetError(#[from] plum_tipset::TipsetError),
    #[error("crypto error {0}")]
    CryptoError(#[from] CryptoError),
    #[error("anyhow error {0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}
