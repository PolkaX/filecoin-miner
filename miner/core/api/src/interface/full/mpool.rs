// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::Cid;
use plum_address::{address_json, Address};
use plum_bigint::BigInt;
use plum_message::{signed_message_json, unsigned_message_json, SignedMessage, UnsignedMessage};
use plum_tipset::{tipset_key_json, TipsetKey};

use jsonrpc_client::{NotificationStream, SubscriptionId};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait MpoolApi: RpcClient {
    ///
    async fn mpool_pending(&self, key: &TipsetKey) -> Result<Vec<SignedMessage>> {
        let signed_msgs: Vec<helper::SignedMessage> = self
            .request(
                "MpoolPending",
                vec![helper::serialize_with(tipset_key_json::serialize, key)],
            )
            .await?;
        Ok(signed_msgs
            .into_iter()
            .map(|signed_msg| signed_msg.0)
            .collect())
    }
    ///
    async fn mpool_push(&self, signed_msg: &SignedMessage) -> Result<Cid> {
        let cid: helper::Cid = self
            .request(
                "MpoolPush",
                vec![helper::serialize_with(
                    signed_message_json::serialize,
                    signed_msg,
                )],
            )
            .await?;
        Ok(cid.0)
    }
    ///
    async fn mpool_push_message(&self, msg: &UnsignedMessage) -> Result<SignedMessage> {
        let signed_msg: helper::SignedMessage = self
            .request(
                "MpoolPushMessage",
                vec![helper::serialize_with(
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
            vec![helper::serialize_with(address_json::serialize, addr)],
        )
        .await
    }
    ///
    async fn mpool_sub(&self) -> Result<(SubscriptionId, NotificationStream<MpoolUpdate>)> {
        self.subscribe("MpoolSub", vec![]).await
    }
    ///
    async fn mpool_estimate_gas_price(
        &self,
        what1: u64,
        addr: &Address,
        what2: i64,
        key: &TipsetKey,
    ) -> Result<BigInt> {
        let price: helper::BigInt = self
            .request(
                "MpoolEstimateGasPrice",
                vec![
                    helper::serialize(&what1),
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize(&what2),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(price.0)
    }
}

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
