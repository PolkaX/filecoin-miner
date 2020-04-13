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
pub trait RpcClient: Send + Sync + 'static {
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
fn test_async_api() {
    use crate::interface::CommonApi;

    async_std::task::block_on(async {
        let client = WsClient::new_async("127.0.0.1:1234".parse().unwrap(), "/rpc/v0").await;
        let id = client.id().await.unwrap();
        println!("async api - id: {:?}", id);
        let version = client.version().await.unwrap();
        println!("async api - version: {:?}", version);

        /*
        use crate::interface::{ChainApi, HeadChange};
        let mut chain_notify_sub: Subscription<HeadChange> = client.chain_notify().await.unwrap();
        let head_change = chain_notify_sub.next().await;
        println!("chain_notify: {:?}", head_change);
        */
    });
}

#[test]
fn test_sync_api() {
    use crate::interface::SyncCommonApi;

    let client = WsClient::new_sync("127.0.0.1:1234".parse().unwrap(), "/rpc/v0");
    let id = client.id_sync().unwrap();
    println!("sync api - id: {:?}", id);
    let version = client.version_sync().unwrap();
    println!("sync api - version: {:?}", version);
}

#[async_std::test]
async fn test_multi_task() {
    use crate::interface::CommonApi;
    use std::time::Duration;

    let client = WsClient::new_async("127.0.0.1:1234".parse().unwrap(), "/rpc/v0").await;
    let client2 = client.clone();
    async_std::task::spawn(async move {
        async_std::task::sleep(Duration::from_secs(5)).await;
        let id = client2.id().await.unwrap();
        println!("async api - id: {:?}", id);
    });

    let version = client.version().await.unwrap();
    println!("async api - version: {:?}", version);
    async_std::task::sleep(Duration::from_secs(10)).await;
}

#[test]
fn test_multi_thread() {
    use crate::interface::SyncCommonApi;
    use std::thread;
    use std::time::Duration;

    let client = WsClient::new_sync("127.0.0.1:1234".parse().unwrap(), "/rpc/v0");
    let client2 = client.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let id = client2.id_sync().unwrap();
        println!("sync api - id: {:?}", id);
    });

    let version = client.version_sync().unwrap();
    println!("sync api - version: {:?}", version);
    thread::sleep(Duration::from_secs(10));
}
