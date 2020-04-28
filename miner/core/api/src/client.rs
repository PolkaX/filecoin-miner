// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub use jsonrpc_client::{HttpTransport, NotificationStream, SubscriptionId, WebSocketTransport};
use jsonrpc_client::{Params, PubsubTransport, Transport, Value};

use crate::errors::Result;

#[async_trait::async_trait]
pub trait RpcClient: Send + Sync + 'static {
    ///
    async fn request<M, T>(&self, method: M, params: Vec<Value>) -> Result<T>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned;

    async fn subscribe<M, T>(
        &self,
        subscribe_method: M,
        params: Vec<Value>,
    ) -> Result<(SubscriptionId, NotificationStream<T>)>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned;

    fn unsubscribe(&self, subscription_id: SubscriptionId);
}

#[async_trait::async_trait]
impl RpcClient for HttpTransport {
    async fn request<M, T>(&self, method: M, params: Vec<Value>) -> Result<T>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned,
    {
        Ok(self
            .send(
                format!("Filecoin.{}", method.as_ref()),
                Params::Array(params),
            )
            .await?)
    }

    async fn subscribe<M, T>(
        &self,
        _subscribe_method: M,
        _params: Vec<Value>,
    ) -> Result<(SubscriptionId, NotificationStream<T>)>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned,
    {
        unimplemented!("HTTP doesn't support `pub-sub` mode")
    }

    fn unsubscribe(&self, _subscription_id: SubscriptionId) {
        unimplemented!("HTTP doesn't support `pub-sub` mode")
    }
}

#[async_trait::async_trait]
impl RpcClient for WebSocketTransport {
    async fn request<M, T>(&self, method: M, params: Vec<Value>) -> Result<T>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned,
    {
        Ok(self
            .send(
                format!("Filecoin.{}", method.as_ref()),
                Params::Array(params),
            )
            .await?)
    }

    async fn subscribe<M, T>(
        &self,
        subscribe_method: M,
        params: Vec<Value>,
    ) -> Result<(SubscriptionId, NotificationStream<T>)>
    where
        M: AsRef<str> + Send,
        T: serde::de::DeserializeOwned,
    {
        let subscription_id: SubscriptionId = self
            .send(
                format!("Filecoin.{}", subscribe_method.as_ref()),
                Params::Array(params),
            )
            .await?;
        Ok((
            subscription_id,
            PubsubTransport::subscribe(self, subscription_id),
        ))
    }

    fn unsubscribe(&self, subscription_id: SubscriptionId) {
        PubsubTransport::unsubscribe(self, subscription_id)
    }
}

mod impls {
    use super::{HttpTransport, WebSocketTransport};
    use crate::interface::*;

    // HTTP
    // async version
    impl CommonApi for HttpTransport {}
    impl FullNodeApi for HttpTransport {}
    impl StorageMinerApi for HttpTransport {}

    impl ChainApi for HttpTransport {}
    impl ClientApi for HttpTransport {}
    impl MarketApi for HttpTransport {}
    impl MinerApi for HttpTransport {}
    impl MpoolApi for HttpTransport {}
    impl PaychApi for HttpTransport {}
    impl StateApi for HttpTransport {}
    impl SyncApi for HttpTransport {}
    impl WalletApi for HttpTransport {}

    // WebSocket
    // async version
    impl CommonApi for WebSocketTransport {}
    impl FullNodeApi for WebSocketTransport {}
    impl StorageMinerApi for WebSocketTransport {}

    impl ChainApi for WebSocketTransport {}
    impl ClientApi for WebSocketTransport {}
    impl MarketApi for WebSocketTransport {}
    impl MinerApi for WebSocketTransport {}
    impl MpoolApi for WebSocketTransport {}
    impl PaychApi for WebSocketTransport {}
    impl StateApi for WebSocketTransport {}
    impl SyncApi for WebSocketTransport {}
    impl WalletApi for WebSocketTransport {}
}

#[tokio::test]
async fn test_async_api() {
    use crate::interface::CommonApi;

    let client = WebSocketTransport::new("ws://127.0.0.1:1234/rpc/v0");
    let version = client.version().await.unwrap();
    println!("version: {:?}", version);

    /*
    use tokio::stream::StreamExt;
    use crate::interface::ChainApi;
    let (_sub_id, mut chain_notify_stream) = client.chain_notify().await.unwrap();
    while let Some(head_changes) = chain_notify_stream.next().await {
        println!("chain_notify: {:?}", head_changes);
    }
    */
}

#[tokio::test]
async fn test_multi_task() {
    use crate::interface::CommonApi;
    use std::time::Duration;

    let client = HttpTransport::new("http://127.0.0.1:1234/rpc/v0");
    let client2 = client.clone();
    tokio::task::spawn(async move {
        tokio::time::delay_for(Duration::from_secs(5)).await;
        let id = client2.id().await.unwrap();
        println!("api - id: {:?}", id);
    });

    let version = client.version().await.unwrap();
    println!("api - version: {:?}", version);
    tokio::time::delay_for(Duration::from_secs(10)).await;
}
