mod error;

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use blake2_rfc::blake2b::blake2b;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

pub use error::ParamsError;
use log::{error, info};

pub const DIGEST_LEN: usize = 32;
pub type Result<T> = std::result::Result<T, ParamsError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParamsInfo {
    cid: String,
    digest: String,
    sector_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFile {
    name: String,
    info: ParamsInfo,
}

pub fn get_params(
    storage_size: u64,
    gateway: &str,
    params_path: PathBuf,
    parameters: serde_json::Value,
) -> Result<()> {
    let params_map = parameters
        .as_object()
        .ok_or(ParamsError::Object("parameters.json error".into()))?;

    if !params_path.as_path().exists() {
        fs::create_dir(params_path.as_path())?
    }

    let mut childs = vec![];
    for (key, value) in params_map.clone() {
        let info: ParamsInfo = serde_json::from_value(value.clone())?;
        if storage_size != info.sector_size && key.ends_with(".params") {
            continue;
        }
        let mut file_path = params_path.clone();
        file_path.push(&key);

        if file_path.exists() {
            if check_file(file_path.as_path(), &info)? {
                info!("file {:} is ok", file_path.as_path().display());
                return Ok(());
            }
        }

        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
        {
            Ok(cli) => Arc::new(cli),
            Err(e) => return Err(ParamsError::Reqwest(e)),
        };

        let gateway = gateway.to_owned();
        let params_path = params_path.to_owned();
        let child = thread::spawn(move || {
            match fetch(client.clone(), &key, params_path.as_path(), &gateway, info) {
                Ok(_) => info!("fetch {:} from {:} success", key, gateway),
                Err(e) => error!("fetch params error:{}", e),
            };
        });
        childs.push(child);
    }
    for child in childs {
        child.join().expect("panic inner.");
    }
    Ok(())
}

fn fetch(
    client: Arc<reqwest::blocking::Client>,
    name: &str,
    params_path: &Path,
    gateway: &str,
    info: ParamsInfo,
) -> Result<()> {
    let url = gateway.to_owned() + &info.cid;
    info!("fetch {:} from {:}", name, url);

    let mut res = client.get(url.as_str()).send()?;
    let mut file = params_path.to_path_buf();
    file.push(name);
    let mut f = fs::File::create(file.as_path())?;
    res.copy_to(&mut f)?;
    Ok(())
}

fn check_file(file_path: &Path, info: &ParamsInfo) -> Result<bool> {
    let mut fp = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    // read the whole file
    fp.read_to_end(&mut buffer)?;
    let br = blake2b(64, &[], buffer.as_slice());
    let brs = hex::encode(&br);
    let r = if &brs[..DIGEST_LEN] == info.digest {
        true
    } else {
        false
    };
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_params() {
        let size = vec![16777216, 1024, 1073741824, 268435456, 1073741824, 16777216];
        let s = r#"
        {
            "v20-proof-of-spacetime-election-5f585aca354eb68e411c8582ed0efd800792430e4e76d73468c4fc03f1a8d6d2.params": {
            "cid": "QmX7tYeNPWae2fjZ3Am6GB9dmHvLqvoz8dKo3PR98VYxH9",
            "digest": "39a9edec3355516674f0d12b926be493",
            "sector_size": 34359738368
            },
            "v20-proof-of-spacetime-election-5f585aca354eb68e411c8582ed0efd800792430e4e76d73468c4fc03f1a8d6d2.vk": {
            "cid": "QmbNGx7pNbGiEr8ykoHxVXHW2LNSmGdsxKtj1onZCyguCX",
            "digest": "0227ae7df4f2affe529ebafbbc7540ee",
            "sector_size": 34359738368
            },
            "v20-proof-of-spacetime-election-a4e18190d4b4657ba1b4d08a341871b2a6f398e327cb9951b28ab141fbdbf49d.params": {
            "cid": "QmRGZsNp4mp1cZshcXqt3VMuWscAEsiMa2iepF4CsWWoiv",
            "digest": "991041a354b12c280542741f58c7f2ca",
            "sector_size": 1024
            },
            "v20-proof-of-spacetime-election-a4e18190d4b4657ba1b4d08a341871b2a6f398e327cb9951b28ab141fbdbf49d.vk": {
            "cid": "QmWpmrhCGVcfqLyqp5oGAnhPmCE5hGTPaauHi25mpQwRSU",
            "digest": "91fac550e1f9bccab213830bb0c85bd6",
            "sector_size": 1024
            },
            "v20-proof-of-spacetime-election-a9eb6d90b896a282ec2d3a875c6143e3fcff778f0da1460709e051833651559b.params": {
            "cid": "QmenSZXh1EsSyHiSRvA6wb8yaPhYBTjrKehJw96Px5HnN4",
            "digest": "6322eacd2773163ddd51f9ca7d645fc4",
            "sector_size": 1073741824
            },
            "v20-proof-of-spacetime-election-a9eb6d90b896a282ec2d3a875c6143e3fcff778f0da1460709e051833651559b.vk": {
            "cid": "QmPvZoMKofw6eDhDg5ESJA2QAZP8HvM6qMQk7fw4pq9bQf",
            "digest": "0df62745fceac922e3e70847cfc70b52",
            "sector_size": 1073741824
            },
            "v20-proof-of-spacetime-election-bf872523641b1de33553db2a177df13e412d7b3b0103e6696ae0a1cf5d525259.params": {
            "cid": "QmVibFqzkZoL8cwQmzj8njPokCQGCCx4pBcUH77bzgJgV9",
            "digest": "de9d71e672f286706a1673bd57abdaac",
            "sector_size": 16777216
            },
            "v20-proof-of-spacetime-election-bf872523641b1de33553db2a177df13e412d7b3b0103e6696ae0a1cf5d525259.vk": {
            "cid": "QmZa5FX27XyiEXQQLQpHqtMJKLzrcY8wMuj3pxzmSimSyu",
            "digest": "7f796d3a0f13499181e44b5eee0cc744",
            "sector_size": 16777216
            },
            "v20-proof-of-spacetime-election-ffc3fb192364238b60977839d14e3154d4a98313e30d46694a12af54b6874975.params": {
            "cid": "Qmbt2SWWAmMcYoY3DAiRDXA8fAuqdqRLWucJMSxYmzBCmN",
            "digest": "151ae0ae183fc141e8c2bebc28e5cc10",
            "sector_size": 268435456
            },
            "v20-proof-of-spacetime-election-ffc3fb192364238b60977839d14e3154d4a98313e30d46694a12af54b6874975.vk": {
            "cid": "QmUxvPu4xdVmjMFihUKoYyEdXBqxsXkvmxRweU7KouWHji",
            "digest": "95eb89588e9d1832aca044c3a13178af",
            "sector_size": 268435456
            }
        }"#;
        let j = serde_json::from_str(s).unwrap();
        for s in size {
            get_params(
                s,
                "https://ipfs.io/ipfs/",
                PathBuf::from("/tmp/var/tmp/filecoin-proof-parameters/"),
                j,
            );
        }
    }
}
