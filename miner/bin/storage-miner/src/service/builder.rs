use std::borrow::Cow;
use std::pin::Pin;
use std::sync::Arc;

use futures::{channel::mpsc, Future};

use repo::FsLockedRepo;

use super::{storageminer, Service};
use crate::error::*;
use utils::consts::{METADATA_SPACE, SECTORBUILDER_SPACE};

pub struct ServiceBuilder {
    repo: FsLockedRepo,
}

impl ServiceBuilder {
    pub fn new(repo: FsLockedRepo) -> Self {
        Self { repo }
    }

    pub fn build(self) -> Result<Service> {
        let ServiceBuilder {
            // TODO attrs
            repo,
        } = self;

        let (signal, exit) = exit_future::signal();

        // message bus
        // A side-channel for essential tasks to communicate shutdown.
        let (essential_failed_tx, essential_failed_rx) = mpsc::unbounded();
        // List of asynchronous tasks to spawn. We collect them, then spawn them all at once.
        let (to_spawn_tx, to_spawn_rx) =
            mpsc::unbounded::<(Pin<Box<dyn Future<Output = ()> + Send>>, Cow<'static, str>)>();

        // load config from file
        let config = repo.config::<node_config::StorageMiner>()?;

        // create metadata datastore
        let metadata = repo.datastore(METADATA_SPACE)?;

        let miner_addr = storageminer::load_miner_addr(&metadata)?;

        // sectorbuilder init
        let sector = {
            // todo, get size from api.StateMinerSectorSize
            let ssize = 32 << 30;
            let sectorbuilder_config = config
                .sector_builder
                .clone()
                .into_sectorbuilder_config(ssize, miner_addr);

            let sectorbuilder_ds = repo.datastore(SECTORBUILDER_SPACE)?;
            let s = sectorbuilder::SectorBuilder::new(&sectorbuilder_config, sectorbuilder_ds);
            Arc::new(s)
        };

        // TODO init part from attrs
        // todo send channel sender into other part
        Ok(Service {
            repo,
            sectorbuilder: sector,
            exit,
            signal: Some(signal),
            essential_failed_tx,
            essential_failed_rx,
            to_spawn_tx,
            to_spawn_rx,
        })
    }
}
