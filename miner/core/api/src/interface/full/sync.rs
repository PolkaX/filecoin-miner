// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use jsonrpc_client::{NotificationStream, SubscriptionId};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_block::{block_msg_json, BlockHeader, BlockMsg};
use plum_tipset::{tipset_json, Tipset};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait SyncApi: RpcClient {
    ///
    async fn sync_state(&self) -> Result<SyncState> {
        self.request("SyncState", vec![]).await
    }
    ///
    async fn sync_submit_block(&self, block: &BlockMsg) -> Result<()> {
        self.request(
            "SyncSubmitBlock",
            vec![helper::serialize_with(block_msg_json::serialize, block)],
        )
        .await
    }
    ///
    async fn sync_incoming_blocks(
        &self,
    ) -> Result<(SubscriptionId, NotificationStream<BlockHeader>)> {
        self.subscribe("SyncIncomingBlocks", vec![]).await
    }
    ///
    async fn sync_mark_bad(&self, bad_cid: &Cid) -> Result<()> {
        self.request(
            "SyncMarkBad",
            vec![helper::serialize_with(cid_json::serialize, bad_cid)],
        )
        .await
    }
    ///
    async fn sync_check_bad(&self, bad_cid: &Cid) -> Result<String> {
        self.request(
            "SyncCheckBad",
            vec![helper::serialize_with(cid_json::serialize, bad_cid)],
        )
        .await
    }
}

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyncState {
    ///
    pub active_syncs: Vec<ActiveSync>,
}

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveSync {
    ///
    #[serde(with = "tipset_json")]
    pub base: Tipset,
    ///
    #[serde(with = "tipset_json")]
    pub target: Tipset,

    ///
    pub stage: SyncStateStage,
    ///
    pub height: u64,

    ///
    pub start: u64, // need to serialize to the format '2009-11-10T23:00:00Z'
    ///
    pub end: u64, // need to serialize to the format '2009-11-10T23:00:00Z'
    ///
    pub message: String,
}

///
#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum SyncStateStage {
    ///
    StageIdle = 0,
    ///
    StageHeaders = 1,
    ///
    StagePersistHeaders = 2,
    ///
    StageMessages = 3,
    ///
    StageSyncComplete = 4,
    ///
    StageSyncErrored = 5,
}
