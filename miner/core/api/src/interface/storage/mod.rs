// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub mod helpers;
pub use self::helpers::*;

use std::collections::HashMap;

use async_std::task::block_on;
use async_trait::async_trait;

use plum_address::{address_json, Address};

use crate::client::RpcClient;
use crate::errors::Result;

/// The StorageMiner API Interface
#[async_trait]
pub trait StorageMinerApi: RpcClient {
    ///
    async fn actor_address(&self) -> Result<Address> {
        let addr: crate::helpers::Address = self.request("ActorAddress", vec![]).await?;
        Ok(addr.0)
    }
    ///
    async fn actor_sector_size(&self, addr: &Address) -> Result<u64> {
        self.request(
            "ActorSectorSize",
            vec![crate::helpers::serialize_with(
                address_json::serialize,
                addr,
            )],
        )
        .await
    }

    // Temp api for testing
    ///
    async fn pledge_sector(&self) -> Result<()> {
        self.request("PledgeSector", vec![]).await
    }

    /*
    /// Get the status of a given sector by ID
    async fn sector_status(&self, sector_id: u64) -> Result<SectorInfo>;
    */
    /// List all staged sectors
    async fn sectors_list(&self) -> Result<Vec<u64>> {
        self.request("SectorsList", vec![]).await
    }
    ///
    async fn sectors_refs(&self) -> Result<HashMap<String, Vec<SealedRef>>> {
        self.request("SectorsRefs", vec![]).await
    }
    ///
    async fn sectors_update(&self, sector_id: u64, state: SectorState) -> Result<()> {
        self.request(
            "SectorsUpdate",
            vec![
                crate::helpers::serialize(&sector_id),
                crate::helpers::serialize(&state),
            ],
        )
        .await
    }

    /*
    ///
    async fn worker_stats(&self) -> Result<sectorbuilder::WorkerStats>;
    /// WorkerQueue registers a remote worker
    async fn worker_queue(&self, cfg: sectorbuilder::WorkerCfg) -> Result<<-chan sectorbuilder::WorkerTask>;
    ///
    async fn worker_done(&self, task: u64, res: sectorbuilder::SealRes) -> Result<()>;
    */
}

/// The SyncStorageMiner API Interface
pub trait SyncStorageMinerApi: StorageMinerApi {
    ///
    fn actor_address_sync(&self) -> Result<Address> {
        block_on(async { StorageMinerApi::actor_address(self).await })
    }
    ///
    fn actor_sector_size_sync(&self, addr: &Address) -> Result<u64> {
        block_on(async { StorageMinerApi::actor_sector_size(self, addr).await })
    }

    // Temp api for testing
    ///
    fn pledge_sector_sync(&self) -> Result<()> {
        block_on(async { StorageMinerApi::pledge_sector(self).await })
    }

    /*
    /// Get the status of a given sector by ID
    fn sector_status_sync(&self, sector_id: u64) -> Result<SectorInfo>;
    */
    /// List all staged sectors
    fn sectors_list_sync(&self) -> Result<Vec<u64>> {
        block_on(async { StorageMinerApi::sectors_list(self).await })
    }
    ///
    fn sectors_refs_sync(&self) -> Result<HashMap<String, Vec<SealedRef>>> {
        block_on(async { StorageMinerApi::sectors_refs(self).await })
    }
    ///
    fn sectors_update_sync(&self, sector_id: u64, state: SectorState) -> Result<()> {
        block_on(async { StorageMinerApi::sectors_update(self, sector_id, state).await })
    }

    /*
    ///
    fn worker_stats_sync(&self) -> Result<sectorbuilder::WorkerStats>;
    /// WorkerQueue registers a remote worker
    fn worker_queue_sync(&self, cfg: sectorbuilder::WorkerCfg) -> Result<<-chan sectorbuilder::WorkerTask>;
    ///
    fn worker_done_sync(&self, task: u64, res: sectorbuilder::SealRes) -> Result<()>;
    */
}

/*
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorkerStats {
    local_free: i32,
    local_reserved: i32,
    local_total: i32,
    // todo: post in progress
    remotes_total: i32,
    remotes_free: i32,

    add_piece_wait: i32,
    pre_commit_wait: i32,
    commit_wait: i32,
    unseal_wait: i32,
}
*/
