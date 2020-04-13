// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_std::task::block_on;
use async_trait::async_trait;

use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait MarketApi: RpcClient {
    ///
    async fn market_ensure_available(&self, addr: &Address, amt: &BigInt) -> Result<()> {
        self.request(
            "MarketEnsureAvailable",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(bigint_json::serialize, amt),
            ],
        )
        .await
    }
}

pub trait SyncMarketApi: MarketApi {
    ///
    fn market_ensure_available_sync(&self, addr: &Address, amt: &BigInt) -> Result<()> {
        block_on(async { MarketApi::market_ensure_available(self, addr, amt).await })
    }
}
