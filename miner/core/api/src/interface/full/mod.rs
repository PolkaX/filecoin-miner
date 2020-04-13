// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

mod chain;
mod client;
mod market;
mod mpool;
mod paych;
mod state;
mod sync;
mod wallet;

pub use self::chain::*;
pub use self::client::*;
pub use self::market::*;
pub use self::mpool::*;
pub use self::paych::*;
pub use self::state::*;
pub use self::sync::*;
pub use self::wallet::*;

use async_trait::async_trait;

///
#[async_trait]
pub trait FullNodeApi:
    SyncApi + WalletApi + StateApi + MpoolApi + MarketApi + ChainApi + PaychApi + ClientApi
{
}

pub trait SyncFullNodeApi:
    SyncSyncApi
    + SyncWalletApi
    + SyncStateApi
    + SyncMpoolApi
    + SyncMarketApi
    + SyncChainApi
    + SyncPaychApi
    + SyncClientApi
{
}

// The priority of implementation: (1 => 2 => 3 => 4)
// 1. Common, Sync, Wallet
// 2. State, Mpool, Market, Chain
// 3, Paych, StorageMiner
// 4. Client
