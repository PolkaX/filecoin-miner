// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::Cid;
use plum_address::Address;
use plum_bigint::BigInt;
use plum_message::{SignedMessage, UnsignedMessage};
use plum_tipset::TipsetKey;

use jsonrpc_client::{NotificationStream, SubscriptionId};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait MpoolApi: RpcClient {
    ///
    async fn mpool_pending(&self, key: &TipsetKey) -> Result<Vec<SignedMessage>> {
        self.request("MpoolPending", vec![helper::serialize(key)])
            .await
    }
    ///
    async fn mpool_push(&self, signed_msg: &SignedMessage) -> Result<Cid> {
        self.request("MpoolPush", vec![helper::serialize(signed_msg)])
            .await
    }
    ///
    async fn mpool_push_message(&self, msg: &UnsignedMessage) -> Result<SignedMessage> {
        self.request("MpoolPushMessage", vec![helper::serialize(msg)])
            .await
    }
    ///
    async fn mpool_get_nonce(&self, addr: &Address) -> Result<u64> {
        self.request("MpoolGetNonce", vec![helper::serialize(addr)])
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
                    helper::serialize(addr),
                    helper::serialize(&what2),
                    helper::serialize(key),
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
    pub message: Vec<SignedMessage>,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum MpoolChange {
    MpoolAdd = 0,
    MpoolRemove = 1,
}
