use std::fs;
use std::path::Path;

use crate::error::{Result, StoresError};
use crate::TARGET;

use log::debug;

pub fn move_(from: &Path, to_: &Path) -> Result<()> {
    let from_file = from.file_name().ok_or(StoresError::Tmp)?;
    let to_file = to_.file_name().ok_or(StoresError::Tmp)?;
    if from_file != to_file {
        return Err(StoresError::Tmp);
    }
    debug!(
        target: TARGET,
        "move sector data, from:{:?} to:{:?}", from, to_
    );

    let to_dir = to_.parent().ok_or(StoresError::Tmp)?;
    if !to_dir.exists() {
        fs::create_dir_all(to_dir)?;
    }

    fs::rename(from, to_)?;

    Ok(())
}
