// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

///
pub type Result<T> = std::result::Result<T, ApiError>;

///
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Rpc request error.
    #[error("Rpc request: {0}")]
    RpcRequest(#[from] jsonrpsee::client::RequestError),
}
