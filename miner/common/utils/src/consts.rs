
pub const METADATA_SPACE: &'static str = "/metadata";
pub const BLOCK_SPACE: &'static str = "/block";
pub const STAGING_SPACE: &'static str = "/staging";
pub const SECTORBUILDER_SPACE: &'static str = "/sectorbuilder";

pub const ALL_NAMESPACE: [&'static str; 4] = [
    METADATA_SPACE,
    BLOCK_SPACE,
    STAGING_SPACE,
    SECTORBUILDER_SPACE,
];

pub const SECTOR_SIZES: [usize; 1] = [32 << 30];
