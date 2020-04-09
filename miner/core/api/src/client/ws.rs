// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::net::SocketAddr;

use async_trait::async_trait;
use jsonrpsee::{
    client::{Client, Subscription},
    common::Params,
    raw::RawClient,
    transport::ws::{Mode, WsTransportClient},
};
use serde_json::Value;

use crate::client::RpcClient;
use crate::errors::Result;

///
pub struct WsClient {
    client: Client,
    token: Option<String>,
}

impl WsClient {
    ///
    pub async fn new(socket_addr: SocketAddr, url: impl AsRef<str>) -> Self {
        let ws_transport = WsTransportClient::builder(
            socket_addr,
            socket_addr.to_string(),
            socket_addr.ip().to_string(),
            Mode::Plain,
        )
        .with_url(url.as_ref())
        .build()
        .await
        .expect("invalid ws config");

        let raw_client = RawClient::new(ws_transport);
        let client = Client::new(raw_client);
        Self {
            client,
            token: None,
        }
    }

    ///
    pub async fn new_with_token(
        socket_addr: SocketAddr,
        url: impl AsRef<str>,
        token: impl Into<String>,
    ) -> Self {
        let ws_transport = WsTransportClient::builder(
            socket_addr,
            socket_addr.to_string(),
            socket_addr.ip().to_string(),
            Mode::Plain,
        )
        .with_url(url.as_ref())
        .build()
        .await
        .expect("invalid ws config");

        let raw_client = RawClient::new(ws_transport);
        let client = Client::new(raw_client);
        Self {
            client,
            token: Some(token.into()),
        }
    }
}

#[async_trait]
impl RpcClient for WsClient {
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
        subscribe_method: M,
        params: Vec<Value>,
    ) -> Result<Subscription<Notification>>
    where
        M: AsRef<str> + Send,
        Notification: serde::de::DeserializeOwned,
    {
        Ok(self
            .client
            .subscribe(
                format!("Filecoin.{}", subscribe_method.as_ref()),
                Params::Array(params),
                format!("Filecoin.Un{}", subscribe_method.as_ref()),
                self.token.clone(),
            )
            .await?)
    }
}

mod impls {
    use super::WsClient;
    use crate::interface::*;

    impl CommonApi for WsClient {}
    impl FullNodeApi for WsClient {}
    impl StorageMinerApi for WsClient {}

    impl ChainApi for WsClient {}
    impl ClientApi for WsClient {}
    impl MarketApi for WsClient {}
    impl MpoolApi for WsClient {}
    impl PaychApi for WsClient {}
    impl StateApi for WsClient {}
    impl SyncApi for WsClient {}
    impl WalletApi for WsClient {}
}
