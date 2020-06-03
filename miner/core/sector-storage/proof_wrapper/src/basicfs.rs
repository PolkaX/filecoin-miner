// Copyright 2020 PolkaX

use anyhow::Result;
use plum_sector::SectorId;
//use std::collections::HashMap;
use std::fs;
use std::path::Path;
use stores::filetype::{sector_name, SectorFileType, SectorPaths};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SectorFile {
    sector_id: SectorId,
    file_type: SectorFileType,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Provider {
    root: String,
    //wait_sector: HashMap<SectorFile, bool>, // TO DO: safe thread
}

impl Provider {
    pub fn new(root: String) -> Self {
        Provider {
            root,
            //       wait_sector: HashMap::new(),
        }
    }

    pub fn acquire_sector(
        &mut self,
        id: SectorId,
        sector_types: &[SectorFileType],
    ) -> Result<(SectorPaths, bool)> {
        let mut paths = SectorPaths::new(id);
        for file in sector_types.iter() {
            let path = Path::new(&self.root)
                .join(file.to_string())
                .join(sector_name(id));
            if !path.is_dir() {
                fs::create_dir_all(path.clone()).unwrap();
            }
            paths.set_path_by_type(file.clone(), path.as_os_str().to_str().unwrap().to_string());
        }
        Ok((paths, true))
    }
}
