use plum_sector::SectorId;

use crate::filetype::SectorFileType;
use crate::index::StorageId;

pub type Result<T> = std::result::Result<T, StoresError>;
#[derive(thiserror::Error, Debug)]
pub enum StoresError {
    #[error("tmp")]
    Tmp,
    #[error("url parse error:{0}")]
    URLErr(#[from] url::ParseError),
    #[error("sys call error:{0}")]
    UixErr(#[from] nix::Error),
    #[error("io err:{0}")]
    IoErr(#[from] std::io::Error),
    #[error("json parse err:{0}")]
    JsonErr(#[from] serde_json::Error),
    #[error("can't both find and allocate a sector|existing:{0:?}|allocate:{1:?}")]
    SameType(SectorFileType, SectorFileType),
    #[error("couldn't find a suitable path for a sector")]
    NoSuitablePath,
    #[error("not found sector, sector_id:{0:?}, type:{1:?}")]
    NotFoundSector(SectorId, SectorFileType),
    #[error("path not found:{0}")]
    PathNotFound(StorageId),
}
