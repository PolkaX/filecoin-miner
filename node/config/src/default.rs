use std::time::Duration;

use super::{Libp2p, Role, SectorBuilder, API};
use crate::utils::get_role;

impl Default for API {
    fn default() -> Self {
        API {
            listen_address: default_api_listen_addr(),
            timeout: Duration::from_secs(30),
        }
    }
}

fn default_api_listen_addr() -> String {
    match get_role() {
        Role::FullNode => "/ip4/127.0.0.1/tcp/1234/http".to_owned(),
        Role::StorageMiner => "/ip4/127.0.0.1/tcp/2345/http".to_owned(),
    }
}

impl Default for Libp2p {
    fn default() -> Self {
        Libp2p {
            listen_addresses: vec!["/ip4/0.0.0.0/tcp/0".to_owned(), "/ip6/::/tcp/0".to_owned()],
            bootstrap_peers: vec![],
            protected_peers: vec![],
            conn_mgr_low: 150,
            conn_mgr_high: 180,
            conn_mgr_grace: Duration::from_secs(20),
        }
    }
}

impl Default for SectorBuilder {
    fn default() -> Self {
        SectorBuilder {
            path: "".to_string(),
            storage: vec![],
            worker_count: 5,
            disable_local_pre_commit: false,
            disable_local_commit: false,
        }
    }
}
