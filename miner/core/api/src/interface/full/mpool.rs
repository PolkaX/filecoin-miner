// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_trait::async_trait;
// use serde::{de, ser, Deserialize, Serialize};

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

/*
#[derive(Clone, Debug)]
pub struct MpoolUpdate {
    pub ty: MpoolChange,
    pub message: Vec<SignedMessage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MpoolUpdateHelper {
    #[serde(rename = "Type")]
    ty: MpoolChange,
    message: Vec<crate::helpers::SignedMessage>,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MpoolChange {
    MpoolAdd = 0,
    MpoolRemove = 1,
}

impl ser::Serialize for MpoolChange {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for MpoolChange {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match u8::deserialize(deserializer)? {
            0 => MpoolChange::MpoolAdd,
            1 => MpoolChange::MpoolRemove,
            i => return Err(de::Error::custom(format!("unexpect integer {}", i))),
        })
    }
}
*/
