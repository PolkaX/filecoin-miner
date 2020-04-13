// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_std::task::block_on;
use async_trait::async_trait;
// use serde::{Deserialize, Serialize};
// use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::Cid;
use plum_address::{address_json, Address};
use plum_message::{signed_message_json, unsigned_message_json, SignedMessage, UnsignedMessage};
use plum_tipset::{tipset_key_json, TipsetKey};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait MpoolApi: RpcClient {
    ///
    async fn mpool_pending(&self, key: &TipsetKey) -> Result<Vec<SignedMessage>> {
        let signed_msgs: Vec<crate::helpers::SignedMessage> = self
            .request(
                "MpoolPending",
                vec![crate::helpers::serialize_with(
                    tipset_key_json::serialize,
                    key,
                )],
            )
            .await?;
        Ok(signed_msgs
            .into_iter()
            .map(|signed_msg| signed_msg.0)
            .collect())
    }
    ///
    async fn mpool_push(&self, signed_msg: &SignedMessage) -> Result<Cid> {
        let cid: crate::helpers::Cid = self
            .request(
                "MpoolPush",
                vec![crate::helpers::serialize_with(
                    signed_message_json::serialize,
                    signed_msg,
                )],
            )
            .await?;
        Ok(cid.0)
    }
    ///
    async fn mpool_push_message(&self, msg: &UnsignedMessage) -> Result<SignedMessage> {
        let signed_msg: crate::helpers::SignedMessage = self
            .request(
                "MpoolPushMessage",
                vec![crate::helpers::serialize_with(
                    unsigned_message_json::serialize,
                    msg,
                )],
            )
            .await?;
        Ok(signed_msg.0)
    }
    ///
    async fn mpool_get_nonce(&self, addr: &Address) -> Result<u64> {
        self.request(
            "MpoolGetNonce",
            vec![crate::helpers::serialize_with(
                address_json::serialize,
                addr,
            )],
        )
        .await
    }
    /*
    ///
    async fn mpool_sub(&self) -> Result<Receiver<MpoolUpdate>>;
    */
}

pub trait SyncMpoolApi: MpoolApi {
    ///
    fn mpool_pending_sync(&self, key: &TipsetKey) -> Result<Vec<SignedMessage>> {
        block_on(async { MpoolApi::mpool_pending(self, key).await })
    }
    ///
    fn mpool_push_sync(&self, signed_msg: &SignedMessage) -> Result<Cid> {
        block_on(async { MpoolApi::mpool_push(self, signed_msg).await })
    }
    ///
    fn mpool_push_message_sync(&self, msg: &UnsignedMessage) -> Result<SignedMessage> {
        block_on(async { MpoolApi::mpool_push_message(self, msg).await })
    }
    ///
    fn mpool_get_nonce_sync(&self, addr: &Address) -> Result<u64> {
        block_on(async { MpoolApi::mpool_get_nonce(self, addr).await })
    }
    /*
    ///
    fn mpool_sub_sync(&self) -> Result<Receiver<MpoolUpdate>>;
    */
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MpoolUpdate {
    pub r#type: MpoolChange,
    #[serde(with = "signed_message_json::vec")]
    pub message: Vec<SignedMessage>,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum MpoolChange {
    MpoolAdd = 0,
    MpoolRemove = 1,
}
*/
