// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_trait::async_trait;
use jsonrpsee::{
    client::{Client, Subscription},
    common::Params,
    raw::RawClient,
    transport::http::HttpTransportClient,
};
use serde_json::Value;

use crate::client::RpcClient;
use crate::errors::Result;

///
pub struct HttpClient {
    client: Client,
    token: Option<String>,
}

impl HttpClient {
    ///
    pub fn new(url: impl AsRef<str>) -> Self {
        let http_transport = HttpTransportClient::new(url.as_ref());
        let raw_client = RawClient::new(http_transport);
        let client = Client::new(raw_client);
        Self {
            client,
            token: None,
        }
    }

    ///
    pub fn new_with_token(url: impl AsRef<str>, token: impl Into<String>) -> Self {
        let http_transport = HttpTransportClient::new(url.as_ref());
        let raw_client = RawClient::new(http_transport);
        let client = Client::new(raw_client);
        Self {
            client,
            token: Some(token.into()),
        }
    }
}

#[async_trait]
impl RpcClient for HttpClient {
    async fn request<M, Ret>(&self, method: M, params: Vec<Value>) -> Result<Ret>
    where
        M: AsRef<str> + Send,
        Ret: serde::de::DeserializeOwned,
    {
        Ok(self
            .client
            .request(
                format!("Filecoin.{}", method.as_ref()),
                Params::Array(params),
                // token: permission (admin/sign/write/read)
                self.token.clone(),
            )
            .await?)
    }

    async fn subscribe<M, Notification>(
        &self,
        _subscribe_method: M,
        _params: Vec<Value>,
    ) -> Result<Subscription<Notification>>
    where
        M: AsRef<str> + Send,
        Notification: serde::de::DeserializeOwned,
    {
        unimplemented!()
    }
}

mod impls {
    use super::HttpClient;
    use crate::interface::*;

    impl CommonApi for HttpClient {}
    impl FullNodeApi for HttpClient {}
    impl StorageMinerApi for HttpClient {}

    impl ChainApi for HttpClient {}
    impl ClientApi for HttpClient {}
    impl MarketApi for HttpClient {}
    impl MpoolApi for HttpClient {}
    impl PaychApi for HttpClient {}
    impl StateApi for HttpClient {}
    impl SyncApi for HttpClient {}
    impl WalletApi for HttpClient {}
}
