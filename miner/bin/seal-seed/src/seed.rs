// Copyright 2020 PolkaX

use ds_rocksdb::DatabaseConfig;
use plum_address::Address;
use plum_wallet::KeyInfo;
use repo::{FsRepo, RepoType};
use sectorbuilder::{fs as FS, Config, SectorBuilder, user_bytes_for_sector_size, interface::Interface};
use std::{fs, path::PathBuf};
use utils::consts;

struct StorageDealProposal {}
struct PreSeal {
    commr: [u8; 32],
    commd: [u8; 32],
    sector_size: u64,
    deal: StorageDealProposal, // to do: actors.StorageDealProposal
}

pub struct GenesisMiner {
    owner: Address,
    woker: Address,
    sector_size: u64,
    sectors: Vec<PreSeal>,
    key: KeyInfo,
}

pub fn pre_seal(
    maddr: Address,
    ssize: u64,
    sectors: i32,
    sbroot: String,
    preimage: &[u8],
) -> Option<GenesisMiner> {
    let sectorbuilder_config = Config {
        sector_size: ssize,
        miner: maddr,
        worker_threads: 0,
        fall_back_last_id: 0,
        no_commit: true,
        no_pre_commit: true,
        paths: FS::SimplePath(sbroot.clone()),
    };

    let config = DatabaseConfig::with_columns(
        consts::ALL_NAMESPACE
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    if let Some(repo) = FsRepo::init(PathBuf::from(sbroot), RepoType::FullNode, config).unwrap() {
        let sectorbuilder_ds = repo
            .lock()
            .unwrap()
            .datastore(consts::SECTORBUILDER_SPACE)
            .unwrap();
        let sb = SectorBuilder::new(&sectorbuilder_config, sectorbuilder_ds);
        let size = user_bytes_for_sector_size(ssize);
        let mut sealed_sectors = Vec::new();
        for i in 0..sectors {
            let sid = sb.acquire_sector_id().unwrap();

        }
    }
    None
}
