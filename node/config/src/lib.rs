mod default;
mod utils;

use std::io;
use std::path::Path;
use std::time::Duration;

use node_utils::other_io_err;
use plum_address::Address;
use serde::{de, Deserialize, Serialize};

pub use utils::set_role;
use utils::{duration_de, duration_s, from_file, is_zero};

#[derive(Debug, Copy, Clone)]
pub enum Role {
    FullNode,
    StorageMiner,
}

pub trait LoadConfig
where
    Self: Sized + Default + Serialize + de::DeserializeOwned,
{
    fn from_file(path: &Path) -> io::Result<Self> {
        from_file(path)
    }

    fn from_bytes<B: AsRef<[u8]>>(b: &B) -> io::Result<Self> {
        toml::from_slice(b.as_ref()).map_err(other_io_err)
    }
}

// common is common config between full node and miner
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Common {
    #[serde(rename = "API")]
    pub api: API,
    pub libp2p: Libp2p,
}

// FullNode is a full node config
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FullNode {
    #[serde(flatten)]
    pub common: Common,
    pub metrics: Metrics,
}

impl LoadConfig for FullNode {}

// // common

// StorageMiner is a storage miner config
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StorageMiner {
    #[serde(flatten)]
    pub common: Common,
    pub sector_builder: SectorBuilder,
}

impl SectorBuilder {
    pub fn into_sectorbuilder_config(
        self,
        sector_size: usize,
        addr: Address,
    ) -> sectorbuilder::Config {
        sectorbuilder::Config {
            sector_size: sector_size as u64,
            miner: addr,
            worker_threads: self.worker_count as u8,
            fall_back_last_id: 0,
            no_commit: self.disable_local_commit,
            no_pre_commit: self.disable_local_pre_commit,
            paths: self.storage,
        }
    }
}

impl LoadConfig for StorageMiner {}

// api contains configs for api endpoint
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct API {
    listen_address: String,
    #[serde(serialize_with = "duration_s", deserialize_with = "duration_de")]
    timeout: Duration,
}

// libp2p contains configs for libp2p
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct Libp2p {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    listen_addresses: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    bootstrap_peers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    protected_peers: Vec<String>,

    #[serde(skip_serializing_if = "is_zero")]
    conn_mgr_low: usize,
    #[serde(skip_serializing_if = "is_zero")]
    conn_mgr_high: usize,
    #[serde(serialize_with = "duration_s", deserialize_with = "duration_de")]
    conn_mgr_grace: Duration,
}

// // Full Node

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(default, rename_all = "PascalCase")]
pub struct Metrics {
    pub nickname: String,
    pub head_notifs: bool,
    pub pubsub_tracing: bool,
}

// // storage Miner

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "PascalCase")]
pub struct SectorBuilder {
    pub path: String,
    pub worker_count: usize,

    pub disable_local_pre_commit: bool,
    pub disable_local_commit: bool,
    // could only put end of this struct, due to limit of `toml`
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<sectorbuilder::fs::PathConfig>,
}

pub fn default_full_node() -> FullNode {
    Default::default()
}

pub fn default_storage_miner() -> StorageMiner {
    Default::default()
}

pub fn config_comment<S: Serialize>(obj: &S) -> io::Result<String> {
    let s = toml::to_string(obj).map_err(other_io_err)?;
    let mut buf = String::new();
    buf.push_str("# Default config:\n");
    buf.push_str(&s);
    let buf = buf.replace("\n", "\n#  ");
    let buf = buf.replace("#  \n", "#\n");
    let buf = buf.replace("#  [", "[");
    let buf = buf.trim_end().to_string();
    Ok(buf)
}

#[test]
#[serial_test::serial]
fn test_storage_config() {
    set_role(Role::StorageMiner);
    let expert = r#"# Default config:
[API]
#  ListenAddress = "/ip4/127.0.0.1/tcp/2345/http"
#  Timeout = "30s"
#
[Libp2p]
#  ListenAddresses = ["/ip4/0.0.0.0/tcp/0", "/ip6/::/tcp/0"]
#  ConnMgrLow = 150
#  ConnMgrHigh = 180
#  ConnMgrGrace = "20s"
#
[SectorBuilder]
#  Path = ""
#  WorkerCount = 5
#  DisableLocalPreCommit = false
#  DisableLocalCommit = false
#"#;
    let config = default_storage_miner();
    let s = config_comment(&config).unwrap();
    assert_eq!(s, expert);
    let new = s.replace("# Default config:", "").replace("#", "");
    let config2 = toml::from_str(&new).unwrap();
    assert_eq!(config, config2);

    let def: StorageMiner = toml::from_str(&s).unwrap();
    assert_eq!(config, def);
}

#[test]
#[serial_test::serial]
fn test_fullnode_config() {
    let expert = r#"# Default config:
[API]
#  ListenAddress = "/ip4/127.0.0.1/tcp/1234/http"
#  Timeout = "30s"
#
[Libp2p]
#  ListenAddresses = ["/ip4/0.0.0.0/tcp/0", "/ip6/::/tcp/0"]
#  ConnMgrLow = 150
#  ConnMgrHigh = 180
#  ConnMgrGrace = "20s"
#
[Metrics]
#  Nickname = ""
#  HeadNotifs = false
#  PubsubTracing = false
#"#;
    set_role(Role::FullNode);
    let config = default_full_node();
    let s = config_comment(&config).unwrap();
    assert_eq!(s, expert);
    let new = s.replace("# Default config:", "").replace("#", "");
    let config2 = toml::from_str(&new).unwrap();
    assert_eq!(config, config2);

    let def: FullNode = toml::from_str(&s).unwrap();
    assert_eq!(config, def);
}
