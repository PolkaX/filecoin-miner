// Copyright 2020 PolkaX.

use super::*;
use anyhow::Result;
use filecoin_proofs_api::{PaddedBytesAmount, RegisteredSealProof, UnpaddedBytesAmount};
use paired::bls12_381::{Bls12, Fr};
use paired::Engine;
use plum_address::Address;
use rand::{Rng, RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::io::{BufWriter, Read, Seek, SeekFrom, Write};
use std::str::FromStr;
use std::sync::atomic::Ordering;
use tempfile::NamedTempFile;

//type DefaultPieceHasher = storage_proofs::hasher::Sha256Hasher;
const SECTOR_SIZE: u64 = 1024;
type SectorNumber = u64;
pub(crate) const TEST_SEED: [u8; 16] = [
    0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc, 0xe5,
];

/*
pub fn commitment_from_fr<E: Engine>(fr: E::Fr) -> Commitment {
    let mut commitment = [0; 32];
    for (i, b) in fr_into_bytes::<E>(&fr).iter().enumerate() {
        commitment[i] = *b;
    }
    commitment
}

fn build_sector(
    piece_sizes: &[UnpaddedBytesAmount],
    sector_size: SectorSize,
) -> Result<([u8; 32], Vec<PieceInfo>)> {
    let rng = &mut XorShiftRng::from_seed(TEST_SEED);
    let graph = StackedBucketGraph::<DefaultPieceHasher>::new_stacked(
        u64::from(sector_size) as usize / NODE_SIZE,
        DRG_DEGREE.load(Ordering::Relaxed) as usize,
        EXP_DEGREE.load(Ordering::Relaxed) as usize,
        new_seed(),
    )?;

    let mut staged_sector = Vec::with_capacity(u64::from(sector_size) as usize);
    let mut staged_sector_io = std::io::Cursor::new(&mut staged_sector);
    let mut piece_infos = Vec::with_capacity(piece_sizes.len());

    for (i, piece_size) in piece_sizes.iter().enumerate() {
        let piece_size_u = u64::from(*piece_size) as usize;
        let mut piece_bytes = vec![1u8; piece_size_u];
        rng.fill_bytes(&mut piece_bytes);

        let mut piece_file = std::io::Cursor::new(&mut piece_bytes);

        let piece_info =
            filecoin_proofs_v1::generate_piece_commitment(&mut piece_file, *piece_size)?;
        piece_file.seek(SeekFrom::Start(0))?;

        filecoin_proofs_v1::add_piece(
            &mut piece_file,
            &mut staged_sector_io,
            *piece_size,
            &piece_sizes[..i],
        )?;

        piece_infos.push(piece_info);
    }
    assert_eq!(staged_sector.len(), u64::from(sector_size) as usize);

    let data_tree: DataTree = graph.merkle_tree(None, &staged_sector)?;
    let comm_d_root: Fr = data_tree.root().into();
    let comm_d = commitment_from_fr::<Bls12>(comm_d_root);

    Ok((comm_d, piece_infos))
}*/

#[test]
fn seal_pre_commit_test() -> Result<()> {
    use datastore::basic_ds::new_map_datastore;
    use plum_address::{set_network, Network};
    unsafe {
        set_network(Network::Test);
    }

    let rng = &mut XorShiftRng::from_seed(TEST_SEED);

    let sector_size = RegisteredSealProof::StackedDrg2KiBV1.sector_size().0;

    let number_of_bytes_in_piece =
        UnpaddedBytesAmount::from(PaddedBytesAmount(sector_size.clone()));

    let piece_bytes: Vec<u8> = (0..number_of_bytes_in_piece.0)
        .map(|_| rand::random::<u8>())
        .collect();

    let mut piece_file = NamedTempFile::new()?;
    piece_file.write_all(&piece_bytes)?;
    piece_file.as_file_mut().sync_all()?;
    piece_file.as_file_mut().seek(SeekFrom::Start(0))?;

    let piece_info = filecoin_proofs_api::seal::generate_piece_commitment(
        RegisteredSealProof::StackedDrg2KiBV1,
        piece_file.as_file_mut(),
        number_of_bytes_in_piece,
    )?;
    piece_file.as_file_mut().seek(SeekFrom::Start(0))?;

    let mut staged_sector_file = NamedTempFile::new()?;
    filecoin_proofs_api::seal::add_piece(
        RegisteredSealProof::StackedDrg2KiBV1,
        &mut piece_file,
        &mut staged_sector_file,
        number_of_bytes_in_piece,
        &[],
    )?;

    let piece_infos = vec![piece_info];
    let config = types::Config {
        sector_size,
        miner: Address::from_str("t0009").unwrap(),
        worker_threads: 0,
        fall_back_last_id: 0,
        no_commit: true,
        no_pre_commit: true,
        paths: vec![fs::PathConfig::default()],
    };

    let ds = new_map_datastore();

    let mut sector_builder = SectorBuilder::new(&config, ds);
    let ticket = rng.gen();
    sector_builder.seal_pre_commit(1, ticket, &piece_infos);

    Ok(())
}
