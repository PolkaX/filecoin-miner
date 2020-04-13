// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod helpers;

pub use self::helpers::*;

use async_std::task::block_on;
use async_trait::async_trait;
use libp2p_core::PeerId;

use crate::client::RpcClient;
use crate::errors::Result;

/// The Common API interface
#[async_trait]
pub trait CommonApi: RpcClient {
    async fn auth_verify(&self, token: &str) -> Result<Vec<Permission>> {
        self.request("AuthVerify", vec![crate::helpers::serialize(&token)])
            .await
    }

    async fn auth_new(&self, permissions: &[Permission]) -> Result<String> {
        self.request("AuthNew", vec![crate::helpers::serialize(&permissions)])
            .await
    }

    async fn net_connectedness(&self, peer_id: &PeerId) -> Result<Connectedness> {
        self.request(
            "NetConnectedness",
            vec![crate::helpers::serialize_with(
                crate::helpers::peer_id::serialize,
                peer_id,
            )],
        )
        .await
    }

    async fn net_peers(&self) -> Result<Vec<PeerAddrInfo>> {
        self.request("NetPeers", vec![]).await
    }

    async fn net_connect(&self, addr_info: &PeerAddrInfo) -> Result<()> {
        self.request("NetConnect", vec![crate::helpers::serialize(addr_info)])
            .await
    }

    async fn net_addrs_listen(&self) -> Result<PeerAddrInfo> {
        self.request("NetAddrsListen", vec![]).await
    }

    async fn net_disconnect(&self, peer_id: &PeerId) -> Result<()> {
        self.request(
            "NetDisconnect",
            vec![crate::helpers::serialize_with(
                crate::helpers::peer_id::serialize,
                peer_id,
            )],
        )
        .await
    }

    async fn net_find_peer(&self, peer_id: &PeerId) -> Result<PeerAddrInfo> {
        self.request(
            "NetFindPeer",
            vec![crate::helpers::serialize_with(
                crate::helpers::peer_id::serialize,
                peer_id,
            )],
        )
        .await
    }

    /// returns peer id of libp2p node backing this API.
    async fn id(&self) -> Result<PeerId> {
        let peer_id: crate::helpers::PeerId = self.request("ID", vec![]).await?;
        Ok(peer_id.0)
    }

    /// provides information about API provider.
    async fn version(&self) -> Result<Version> {
        self.request("Version", vec![]).await
    }

    async fn log_list(&self) -> Result<Vec<String>> {
        self.request("LogList", vec![]).await
    }

    async fn log_set_level(&self, subsystem: &str, level: &str) -> Result<()> {
        self.request(
            "LogSetLevel",
            vec![
                crate::helpers::serialize(&subsystem),
                crate::helpers::serialize(&level),
            ],
        )
        .await
    }
}

/// The SyncCommon API interface
pub trait SyncCommonApi: CommonApi {
    fn auth_verify_sync(&self, token: &str) -> Result<Vec<Permission>> {
        block_on(async { CommonApi::auth_verify(self, token).await })
    }

    fn auth_new_sync(&self, permissions: &[Permission]) -> Result<String> {
        block_on(async { CommonApi::auth_new(self, permissions).await })
    }

    fn net_connectedness_sync(&self, peer_id: &PeerId) -> Result<Connectedness> {
        block_on(async { CommonApi::net_connectedness(self, peer_id).await })
    }

    fn net_peers_sync(&self) -> Result<Vec<PeerAddrInfo>> {
        block_on(async { CommonApi::net_peers(self).await })
    }

    fn net_connect_sync(&self, addr_info: &PeerAddrInfo) -> Result<()> {
        block_on(async { CommonApi::net_connect(self, addr_info).await })
    }

    fn net_addrs_listen_sync(&self) -> Result<PeerAddrInfo> {
        block_on(async { CommonApi::net_addrs_listen(self).await })
    }

    fn net_disconnect_sync(&self, peer_id: &PeerId) -> Result<()> {
        block_on(async { CommonApi::net_disconnect(self, peer_id).await })
    }

    fn net_find_peer_sync(&self, peer_id: &PeerId) -> Result<PeerAddrInfo> {
        block_on(async { CommonApi::net_find_peer(self, peer_id).await })
    }

    /// returns peer id of libp2p node backing this API.
    fn id_sync(&self) -> Result<PeerId> {
        block_on(async { CommonApi::id(self).await })
    }

    /// provides information about API provider.
    fn version_sync(&self) -> Result<Version> {
        block_on(async { CommonApi::version(self).await })
    }

    fn log_list_sync(&self) -> Result<Vec<String>> {
        block_on(async { CommonApi::log_list(self).await })
    }

    fn log_set_level_sync(&self, subsystem: &str, level: &str) -> Result<()> {
        block_on(async { CommonApi::log_set_level(self, subsystem, level).await })
    }
}
