use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::time::Duration;

use node_utils::other_io_err;
use serde::{de, de::Error, Deserialize, Serialize, Serializer};

use super::Role;

// trick global to decide the default value of `listen_address`. should set once before.
static mut ROLE: Role = Role::FullNode;

pub fn set_role(r: Role) {
    unsafe {
        ROLE = r;
    }
}
pub fn get_role() -> Role {
    unsafe { ROLE }
}

pub fn from_file<T: de::DeserializeOwned + Default>(path: &Path) -> io::Result<T> {
    if !path.exists() {
        return Ok(Default::default());
    }
    let mut f = fs::File::open(path)?;
    let mut buf: Vec<u8> = vec![];
    f.read(&mut buf)?;
    toml::from_slice(&buf).map_err(other_io_err)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_zero(num: &usize) -> bool {
    *num == 0
}

pub fn duration_s<S>(d: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let sec = d.as_secs();
    let sec = format!("{}s", sec);
    sec.serialize(s)
}

pub fn duration_de<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut s = String::deserialize(deserializer)?;
    if !s.ends_with("s") {
        return Err(D::Error::custom(format!(
            "duration should end with 's', now is:{}",
            s
        )));
    }
    s.pop();
    let d = s.parse::<u64>().map_err(|e| {
        D::Error::custom(format!(
            "parse duration error, must be an number like [num]s, now is:{}, error:{:?}",
            s, e
        ))
    })?;
    Ok(Duration::from_secs(d))
}
