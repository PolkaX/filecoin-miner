// Copyright 2020 PolkaX

use ds_rocksdb::DatabaseConfig;
use filecoin_proofs_api::{PaddedBytesAmount, RegisteredSealProof, UnpaddedBytesAmount};
use plum_address::Address;
use plum_sector::SectorSize;
use plum_wallet::KeyInfo;
use proof_wrapper::sealer::Sealer;
use repo::{FsRepo, RepoType};
use specs_storage::Storage;
use std::io::{Seek, SeekFrom, Write};
use std::{fs, path::PathBuf};
use tempfile::NamedTempFile;
use utils::consts;

struct StorageDealProposal {}
struct PreSeal {
    commr: [u8; 32],
    commd: [u8; 32],
    sector_size: u64,
    deal: StorageDealProposal, // to do: actors.StorageDealProposal
}

pub struct GenesisMiner {
    owner: Address,
    woker: Address,
    sector_size: u64,
    sectors: Vec<PreSeal>,
    key: KeyInfo,
}

const SECTORS: i64 = 1;
const ROOT_PATH: &str = "./test";
const SECTOR_SIZE: SectorSize = 2 * 1024;
const SEAL_PROOF_TYPE: RegisteredSealProof = RegisteredSealProof::StackedDrg2KiBV1;
pub fn pre_seal() {
    let sealer = Sealer::new(ROOT_PATH.into(), SEAL_PROOF_TYPE, SECTOR_SIZE);
    let number_of_bytes_in_piece = UnpaddedBytesAmount::from(PaddedBytesAmount(SECTOR_SIZE));
    let piece_bytes: Vec<u8> = (0..number_of_bytes_in_piece.0)
        .map(|_| rand::random::<u8>())
        .collect();
    let mut piece_file = NamedTempFile::new().unwrap();
    piece_file.write_all(&piece_bytes).unwrap();
    piece_file.as_file_mut().sync_all().unwrap();
    piece_file.as_file_mut().seek(SeekFrom::Start(0)).unwrap();

    piece_file.as_file_mut().seek(SeekFrom::Start(0)).unwrap();
    let mut staged_sector_file = NamedTempFile::new().unwrap();
    let (piece_info, size) = sealer
        .add_piece(
            &mut piece_file,
            &mut staged_sector_file,
            number_of_bytes_in_piece,
            &[],
        )
        .unwrap();
}
