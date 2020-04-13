// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_std::task::block_on;
use async_trait::async_trait;
use serde::{de, ser, Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_block::{block_msg_json, BlockMsg};
use plum_tipset::{tipset_json, Tipset};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait SyncApi: RpcClient {
    ///
    async fn sync_state(&self) -> Result<SyncState> {
        self.request("SyncState", vec![]).await
    }
    ///
    async fn sync_submit_block(&self, block: &BlockMsg) -> Result<()> {
        self.request(
            "SyncSubmitBlock",
            vec![crate::helpers::serialize_with(
                block_msg_json::serialize,
                block,
            )],
        )
        .await
    }
    /*
    ///
    async fn sync_incoming_blocks(&self) -> Result<Receiver<BlockHeader>>;
    */
    ///
    async fn sync_mark_bad(&self, bad_cid: &Cid) -> Result<()> {
        self.request(
            "SyncMarkBad",
            vec![crate::helpers::serialize_with(cid_json::serialize, bad_cid)],
        )
        .await
    }
    ///
    async fn sync_check_bad(&self, bad_cid: &Cid) -> Result<String> {
        self.request(
            "SyncCheckBad",
            vec![crate::helpers::serialize_with(cid_json::serialize, bad_cid)],
        )
        .await
    }
}

pub trait SyncSyncApi: SyncApi {
    ///
    fn sync_state_sync(&self) -> Result<SyncState> {
        block_on(async { SyncApi::sync_state(self).await })
    }
    ///
    fn sync_submit_block_sync(&self, block: &BlockMsg) -> Result<()> {
        block_on(async { SyncApi::sync_submit_block(self, block).await })
    }
    /*
    ///
    fn sync_incoming_blocks_sync(&self) -> Result<Receiver<BlockHeader>>;
    */
    ///
    fn sync_mark_bad_sync(&self, bad_cid: &Cid) -> Result<()> {
        block_on(async { SyncApi::sync_mark_bad(self, bad_cid).await })
    }
    ///
    fn sync_check_bad_sync(&self, bad_cid: &Cid) -> Result<String> {
        block_on(async { SyncApi::sync_check_bad(self, bad_cid).await })
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
#[derive(Copy, Clone, Debug)]
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

impl ser::Serialize for SyncStateStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for SyncStateStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match u8::deserialize(deserializer)? {
            0 => SyncStateStage::StageIdle,
            1 => SyncStateStage::StageHeaders,
            2 => SyncStateStage::StagePersistHeaders,
            3 => SyncStateStage::StageMessages,
            4 => SyncStateStage::StageSyncComplete,
            5 => SyncStateStage::StageSyncErrored,
            i => return Err(de::Error::custom(format!("unexpect integer {}", i))),
        })
    }
}
