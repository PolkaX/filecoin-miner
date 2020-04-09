// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod http;
mod ws;

pub use self::http::HttpClient;
pub use self::ws::WsClient;

use async_trait::async_trait;
use jsonrpsee::client::Subscription;
use serde_json::Value;

use crate::errors::Result;

#[async_trait]
pub trait RpcClient {
    ///
    async fn request<M, Ret>(&self, method: M, params: Vec<Value>) -> Result<Ret>
    where
        M: AsRef<str> + Send,
        Ret: serde::de::DeserializeOwned;

    async fn subscribe<M, Notification>(
        &self,
        subscribe_method: M,
        params: Vec<Value>,
    ) -> Result<Subscription<Notification>>
    where
        M: AsRef<str> + Send,
        Notification: serde::de::DeserializeOwned;
}

#[test]
fn test_api() {
    async_std::task::block_on(async {
        use crate::interface::CommonApi;

        let client = WsClient::new("127.0.0.1:1234".parse().unwrap(), "/rpc/v0").await;
        let id = client.id().await.unwrap();
        println!("id: {:?}", id);
        let version = client.version().await.unwrap();
        println!("version: {:?}", version);

        /*
        use crate::interface::{ChainApi, HeadChange};
        let mut chain_notify_sub: Subscription<HeadChange> = client.chain_notify().await.unwrap();
        let head_change = chain_notify_sub.next().await;
        println!("chain_notify: {:?}", head_change);
        */
    });
    /*
    async_std::task::block_on(async {
        use crate::interface::CommonApi;

        let client = HttpClient::new("http://127.0.0.1:1234/rpc/v0");
        let id = client.id().await.unwrap();
        println!("id: {:?}", id);
        let version = client.version().await.unwrap();
        println!("version: {:?}", version);

        /*
        use crate::interface::{ChainApi, HeadChange};
        let mut chain_notify_sub: Subscription<HeadChange> = client.chain_notify().await.unwrap();
        let head_change = chain_notify_sub.next().await;
        println!("chain_notify: {:?}", head_change);
        */
    });
    */
}
