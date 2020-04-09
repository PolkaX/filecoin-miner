// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

pub mod helpers;
pub use self::helpers::*;

use std::collections::HashMap;

use async_trait::async_trait;

use plum_address::{address_json, Address};

use crate::client::RpcClient;
use crate::errors::Result;

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
