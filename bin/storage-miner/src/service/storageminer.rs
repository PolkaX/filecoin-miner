use std::io;

use datastore::{key, Batching};
use plum_address::Address;

use crate::error::*;

pub fn load_miner_addr<DS: Batching>(ds: &DS) -> Result<Address> {
    let addr = ds.get(&key::Key::new("miner-address"))?;
    let a = Address::new_from_bytes(&addr)?;
    Ok(a)
}

pub fn save_miner_addr<DS: Batching>(ds: &DS, addr: &Address) -> Result<()> {
    ds.put(key::Key::new("miner-address"), addr.as_bytes())?;
    Ok(())
}
