// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_trait::async_trait;
use jsonrpsee::client::Subscription;
use serde::{Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_bigint::BigInt;
use plum_block::BlockHeader;
use plum_message::{unsigned_message_json, MessageReceipt, SignedMessage, UnsignedMessage};
use plum_tipset::{tipset_json, tipset_key_json, Tipset, TipsetKey};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait ChainApi: RpcClient {
    /// ChainNotify returns channel with chain head updates
    /// First message is guaranteed to be of len == 1, and type == 'current'
    async fn chain_notify(&self) -> Result<Subscription<HeadChange>> {
        self.subscribe("ChainNotify", vec![]).await
    }

    ///
    async fn chain_head(&self) -> Result<Tipset> {
        let tipset: crate::helpers::Tipset = self.request("ChainHead", vec![]).await?;
        Ok(tipset.0)
    }

    ///
    async fn chain_get_randomness(&self, key: &TipsetKey, round: i64) -> Result<Vec<u8>> {
        self.request(
            "ChainGetRandomness",
            vec![
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
                crate::helpers::serialize(&round),
            ],
        )
        .await
    }

    ///
    async fn chain_get_block(&self, cid: &Cid) -> Result<BlockHeader> {
        let block_header: crate::helpers::BlockHeader = self
            .request(
                "ChainGetBlock",
                vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
            )
            .await?;
        Ok(block_header.0)
    }

    ///
    async fn chain_get_tipset(&self, key: &TipsetKey) -> Result<Tipset> {
        let tipset: crate::helpers::Tipset = self
            .request(
                "ChainGetTipSet",
                vec![crate::helpers::serialize_with(
                    tipset_key_json::serialize,
                    key,
                )],
            )
            .await?;
        Ok(tipset.0)
    }

    ///
    async fn chain_get_block_messages(&self, cid: &Cid) -> Result<BlockMessages> {
        let block_msgs: BlockMessagesHelper = self
            .request(
                "ChainGetBlockMessages",
                vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
            )
            .await?;
        Ok(block_msgs.into())
    }

    ///
    async fn chain_get_parent_receipts(&self, cid: &Cid) -> Result<MessageReceipt> {
        let msg_receipt: crate::helpers::MessageReceipt = self
            .request(
                "ChainGetParentReceipts",
                vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
            )
            .await?;
        Ok(msg_receipt.0)
    }

    ///
    async fn chain_get_parent_messages(&self, cid: &Cid) -> Result<Vec<ParentMessage>> {
        self.request(
            "ChainGetParentMessages",
            vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }

    ///
    async fn chain_get_tipset_by_height(&self, height: u64, key: &TipsetKey) -> Result<Tipset> {
        let tipset: crate::helpers::Tipset = self
            .request(
                "ChainGetTipSetByHeight",
                vec![
                    crate::helpers::serialize(&height),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(tipset.0)
    }

    ///
    async fn chain_read_obj(&self, cid: &Cid) -> Result<Vec<u8>> {
        self.request(
            "ChainReadObj",
            vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }

    ///
    async fn chain_has_obj(&self, cid: &Cid) -> Result<bool> {
        self.request(
            "ChainHasObj",
            vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }

    ///
    async fn chain_set_head(&self, key: &TipsetKey) -> Result<()> {
        self.request(
            "ChainSetHead",
            vec![crate::helpers::serialize_with(
                tipset_key_json::serialize,
                key,
            )],
        )
        .await
    }

    ///
    async fn chain_get_genesis(&self) -> Result<Tipset> {
        let tipset: crate::helpers::Tipset = self.request("ChainGetGenesis", vec![]).await?;
        Ok(tipset.0)
    }

    ///
    async fn chain_tipset_weight(&self, key: &TipsetKey) -> Result<BigInt> {
        let bigint: crate::helpers::BigInt = self
            .request(
                "ChainTipSetWeight",
                vec![crate::helpers::serialize_with(
                    tipset_key_json::serialize,
                    key,
                )],
            )
            .await?;
        Ok(bigint.0)
    }

    /*
    ///
    async fn chain_get_node(&self, path: &str) -> Result<Box<dyn Node>>;
    */
    ///
    async fn chain_get_message(&self, cid: &Cid) -> Result<UnsignedMessage> {
        let unsigned_msg: crate::helpers::UnsignedMessage = self
            .request(
                "ChainGetMessage",
                vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
            )
            .await?;
        Ok(unsigned_msg.0)
    }
    /*
    ///
    async fn chain_get_path(&self, from: &TipsetKey, to: &TipsetKey) -> Result<store.HeadChange>;
    ///
    async fn chain_export(&self, key: &TipsetKey) -> Result<Receiver<Vec<u8>>>;
    */
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HeadChangeType {
    Revert,
    Apply,
    Current,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HeadChange {
    #[serde(rename = "Type")]
    pub ty: HeadChangeType,
    #[serde(rename = "Val")]
    #[serde(with = "tipset_json")]
    pub val: Tipset,
}

#[derive(Clone, Debug)]
pub struct BlockMessages {
    pub bls_messages: Vec<UnsignedMessage>,
    pub secpk_messages: Vec<SignedMessage>,

    pub cids: Vec<Cid>,
}

impl From<BlockMessagesHelper> for BlockMessages {
    fn from(helper: BlockMessagesHelper) -> Self {
        Self {
            bls_messages: helper.bls_messages.into_iter().map(|bls| bls.0).collect(),
            secpk_messages: helper
                .secpk_messages
                .into_iter()
                .map(|secpk| secpk.0)
                .collect(),
            cids: helper.cids.into_iter().map(|cid| cid.0).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BlockMessagesHelper {
    bls_messages: Vec<crate::helpers::UnsignedMessage>,
    secpk_messages: Vec<crate::helpers::SignedMessage>,
    cids: Vec<crate::helpers::Cid>,
}

// Only For chain_get_parent_messages
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParentMessage {
    #[serde(with = "cid_json")]
    pub cid: Cid,
    #[serde(with = "unsigned_message_json")]
    pub message: UnsignedMessage,
}
