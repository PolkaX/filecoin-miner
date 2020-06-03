// Copyright 2020 PolkaX

use anyhow::{bail, Result};
use filecoin_proofs_api::{seal, seal::SealPreCommitPhase2Output, PieceInfo, RegisteredSealProof};
use log::info;
use plum_sector::{SectorId, SectorSize};
use plum_types::to_prove_id;
use specs_storage::Sealer as SealerTrait;
use specs_storage::*;
use std::{fs, path};
use stores::filetype::{SectorFileType, SectorFileTypes};

type SectorProvider = crate::basicfs::Provider;

pub struct Sealer {
    pub seal_proof_type: RegisteredSealProof,
    ssize: SectorSize,
    sectors: SectorProvider,
}

impl Sealer {
    pub fn new(root_path: String, seal_proof_type: RegisteredSealProof, ssize: SectorSize) -> Self {
        Sealer {
            seal_proof_type,
            ssize,
            sectors: SectorProvider::new(root_path),
        }
    }
}

impl SealerTrait for Sealer {
    fn seal_pre_commit1(
        &mut self,
        sector: SectorId,
        ticket: SealRandomness,
        pieces: &[PieceInfo],
    ) -> Result<PreCommit1Out> {
        let mut all_types = Vec::new();
        for file in SectorFileTypes::iter() {
            all_types.push(*file);
        }
        let (paths, _) = self.sectors.acquire_sector(sector, &all_types).unwrap();
        /*let attr = fs::metadata(paths.paths[&SectorFileType::FTSealed].clone())?;
        if !attr.is_dir() {
            bail!("NO_SEALED_FILE")
        }*/
        // TO DO: test cache

        /*let mut sum = UnpaddedPieceSize::new(0).unwrap();
        for piece in pieces.iter() {
            sum = sum + piece.size;
        }
        let ussize = PaddedPieceSize::new(self.ssize).unwrap().unpadded();
        if !(ussize == sum) {
            bail!("aggregated piece sizes don't match sector size")
        }*/
        let prove_id = to_prove_id(sector.miner).unwrap();
        info!(
            "sealed path: {:?}",
            paths.paths[&SectorFileType::FTSealed].clone()
        );
        seal::seal_pre_commit_phase1(
            self.seal_proof_type,
            paths.paths[&SectorFileType::FTCache].clone(),
            paths.paths[&SectorFileType::FTUnsealed].clone(),
            paths.paths[&SectorFileType::FTSealed].clone(),
            prove_id,
            sector.number.into(),
            ticket,
            pieces,
        )
    }

    fn seal_pre_commit2(
        &mut self,
        sector: SectorId,
        pc1o: PreCommit1Out,
    ) -> Result<SealPreCommitPhase2Output> {
        let mut all_types = Vec::new();
        for file in SectorFileTypes::iter() {
            if *file == SectorFileType::FTUnsealed {
                continue;
            }
            all_types.push(*file);
        }
        let (paths, _) = self.sectors.acquire_sector(sector, &all_types).unwrap();
        seal::seal_pre_commit_phase2(
            pc1o,
            paths.paths[&SectorFileType::FTCache].clone(),
            paths.paths[&SectorFileType::FTSealed].clone(),
        )
    }

    fn seal_commit1(
        &mut self,
        sector: SectorId,
        ticket: SealRandomness,
        seed: InteractiveSealRandomness,
        pieces: &[PieceInfo],
        pco2: SealPreCommitPhase2Output,
    ) -> Result<Commit1Out> {
        let mut all_types = Vec::new();
        for file in SectorFileTypes::iter() {
            if *file == SectorFileType::FTUnsealed {
                continue;
            }
            all_types.push(*file);
        }
        let (paths, _) = self.sectors.acquire_sector(sector, &all_types).unwrap();

        let prove_id = to_prove_id(sector.miner).unwrap();
        seal::seal_commit_phase1(
            paths.paths[&SectorFileType::FTCache].clone(),
            paths.paths[&SectorFileType::FTSealed].clone(),
            prove_id,
            sector.number.into(),
            ticket,
            seed,
            pco2,
            pieces,
        )
    }
    fn seal_commit2(&mut self, sector: SectorId, c1o: Commit1Out) -> Result<Proof> {
        let prove_id = to_prove_id(sector.miner).unwrap();
        seal::seal_commit_phase2(c1o, prove_id, sector.number.into())
    }
    fn finalize_sector(&mut self, sector: SectorId) -> Result<()> {
        let (paths, _) = self
            .sectors
            .acquire_sector(sector, &[SectorFileType::FTCache])
            .unwrap();
        seal::clear_cache(
            self.ssize,
            path::Path::new(&paths.paths[&SectorFileType::FTCache]),
        )
    }
}
