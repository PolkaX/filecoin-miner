use crate::error::SealWorkerError;
use crate::TaskType;
use std::io::Read;

pub type Result<T> = std::result::Result<T, SealWorkerError>;

pub struct Worker {
    api: String,
    minerEndpoint: String,
    repo: String,
    auth: String,
    //sb: SectorBuilder<T>,
}
