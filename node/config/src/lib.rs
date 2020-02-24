mod default;
mod utils;

use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

use node_utils::other_io_err;
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
    api: API,
    libp2p: Libp2p,
}

// FullNode is a full node config
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct FullNode {
    #[serde(flatten)]
    common: Common,
    metrics: Metrics,
}

impl LoadConfig for FullNode {}

// // common

// StorageMiner is a storage miner config
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StorageMiner {
    #[serde(flatten)]
    common: Common,
    sector_builder: SectorBuilder,
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
    nickname: String,
    head_notifs: bool,
    pubsub_tracing: bool,
}

// // storage Miner

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(default, rename_all = "PascalCase")]
pub struct SectorBuilder {
    path: String,
    // todo, change to PathConfig in rust-sectorbuilder
    #[serde(skip_serializing_if = "Vec::is_empty")]
    storage: Vec<PathBuf>,
    worker_count: usize,

    disable_local_pre_commit: bool,
    disable_local_commit: bool,
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
    let config = default_full_node();
    let s = config_comment(&config).unwrap();
    assert_eq!(s, expert);
    let new = s.replace("# Default config:", "").replace("#", "");
    let config2 = toml::from_str(&new).unwrap();
    assert_eq!(config, config2);

    let def: FullNode = toml::from_str(&s).unwrap();
    assert_eq!(config, def);
}
