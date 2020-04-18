pub const BLOCK_SPACE: &str = "/block";
pub const STAGING_SPACE: &str = "/staging";
pub const METADATA_SPACE: &str = "/metadata";
pub const SECTORBUILDER_SPACE: &str = "/sectorbuilder";

pub const ALL_NAMESPACE: [&str; 4] = [
    METADATA_SPACE,
    BLOCK_SPACE,
    STAGING_SPACE,
    SECTORBUILDER_SPACE,
];

pub const SECTOR_SIZES: [usize; 1] = [32 << 30];
