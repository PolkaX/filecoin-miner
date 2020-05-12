// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_address::Address;
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait MarketApi: RpcClient {
    ///
    async fn market_ensure_available(
        &self,
        addr1: &Address,
        addr2: &Address,
        amt: &BigInt,
    ) -> Result<()> {
        self.request(
            "MarketEnsureAvailable",
            vec![
                helper::serialize(addr1),
                helper::serialize(addr2),
                helper::serialize_with(bigint_json::serialize, amt),
            ],
        )
        .await
    }
}
