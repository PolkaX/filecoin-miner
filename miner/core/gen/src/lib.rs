use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use api::FullNode;
use byteorder::{LittleEndian, WriteBytesExt};
use paired::bls12_381::Bls12;
use sectorbuilder::EPostCandidate;

use plum_address::{Address, Protocol};
use plum_bigint::BigInt;
use plum_crypto::{compute_vrf, VrfPrivateKey};
use plum_hashing::sha256;
use plum_ticket::{EPostProof, EPostTicket};
use plum_tipset::Tipset;

pub const DSepTicket: u64 = 1;
pub const DSepElectionPost: u64 = 2;

pub const EcRandomnessLookback: i64 = 300;

// CommitmentBytesLen is the number of bytes in a CommR, CommD, CommP, and CommRStar.
pub const CommitmentBytesLen: usize = 32;

#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct PublicSectorInfo {
    pub sector_id: u64,
    pub comm_r: [u8; CommitmentBytesLen],
}

impl Ord for PublicSectorInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comm_r.cmp(&other.comm_r)
    }
}

impl PartialOrd for PublicSectorInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct SortedPublicSectorInfo(Vec<PublicSectorInfo>);

// TODO: add test
impl SortedPublicSectorInfo {
    pub fn new(infos: Vec<PublicSectorInfo>) -> Self {
        let mut infos = infos;
        infos.sort();
        Self(infos)
    }
}

pub trait ElectionPoStProver {
    fn generate_candidates(
        &self,
        _: &SortedPublicSectorInfo,
        _: &[u8],
    ) -> Result<Vec<EPostCandidate>>;
    fn compute_proof(
        &self,
        _: &SortedPublicSectorInfo,
        _: &[u8],
        _: &[EPostCandidate],
    ) -> Result<Vec<u8>>;
}

pub struct SectorBuilderEpp;

impl ElectionPoStProver for SectorBuilderEpp {
    // TODO: sectorbuilder.generate_candidates?
    // filecoin-proofs
    fn generate_candidates(
        &self,
        ssi: &SortedPublicSectorInfo,
        rand: &[u8],
    ) -> Result<Vec<EPostCandidate>> {
        Ok(Vec::new())
    }

    fn compute_proof(
        &self,
        ssi: &SortedPublicSectorInfo,
        rand: &[u8],
        winners: &[EPostCandidate],
    ) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct ProofInput {
    sectors: SortedPublicSectorInfo,
    hvrf: Vec<u8>,
    winners: Vec<EPostCandidate>,
    vrfout: Vec<u8>,
}

pub const SectorChallengeRatioDiv: u64 = 25;
fn election_post_challenge_count(sectors: u64, faults: u64) -> u64 {
    if sectors - faults == 0 {
        return 0;
    }
    (sectors - faults - 1) / SectorChallengeRatioDiv + 1
}

// TODO: impl is_ticket_winner in block_header?
fn is_ticket_winner(partial_ticket: &[u8], ssizeI: u64, snum: u64, totpow: &BigInt) -> bool {
    let ssize: BigInt = ssizeI.into();
    let ssampled = election_post_challenge_count(snum, 0);

    let h = sha256(partial_ticket);

    // TODO:

    true
}

pub fn is_round_winner<A: FullNode, E: ElectionPoStProver>(
    ts: &Tipset,
    round: u64,
    miner: &Address,
    epp: &E,
    a: &A,
) -> Result<Option<ProofInput>> {
    todo!()
    /*
    let mworker = a.state_miner_worker(miner, ts)?;

    let pset = a.state_miner_proving_set(miner, ts)?;
    if pset.is_empty() {
        return Ok(None);
    }

    let sector_infos = pset
        .iter()
        .map(|s| {
            let mut comm_r = [0u8; 32];
            // TODO: is this can be ensured?
            assert_eq!(s.comm_r.len(), 32);
            comm_r.copy_from_slice(&s.comm_r[..32]);
            PublicSectorInfo {
                sector_id: s.sector_id,
                comm_r,
            }
        })
        .collect::<Vec<_>>();
    let sector_infos_len = sector_infos.len() as u64;
    let sectors = SortedPublicSectorInfo::new(sector_infos);

    let r = a.chain_get_randomness(ts.key(), EcRandomnessLookback)?;
    let vrfout = compute_vrf(&mworker, DSepElectionPost, r, miner)?;
    let hvrf = sha256(&vrfout).to_vec();

    let candidates = epp.generate_candidates(&sectors, &hvrf)?;
    let pow = a.state_miner_power(miner, ts)?;
    let ssize = a.state_miner_sector_size(miner, ts)?;

    let winners = candidates
        .into_iter()
        .filter(|c| {
            is_ticket_winner(
                &sectorbuilder::fr32::fr_into_bytes::<Bls12>(&c.partial_ticket),
                ssize,
                sector_infos_len,
                &pow.total_power,
            )
        })
        .collect::<Vec<_>>();

    // no winners, sad
    if winners.is_empty() {
        return Ok(None);
    }

    Ok(Some(ProofInput {
        sectors,
        hvrf,
        winners,
        vrfout,
    }))
    */
}

pub fn compute_proof<E: ElectionPoStProver>(epp: &E, pi: &ProofInput) -> Result<EPostProof> {
    let proof = epp.compute_proof(&pi.sectors, &pi.hvrf, &pi.winners)?;

    let candidates = pi
        .winners
        .iter()
        .map(|w| {
            // TODO: sector_id uses u64 or SectorId?
            // TODO: partial is actually a fixed size array [u8; 32], not an arbitray Vec<u8>.
            let partial = sectorbuilder::fr32::fr_into_bytes::<Bls12>(&w.partial_ticket);
            EPostTicket {
                partial,
                sector_id: w.sector_id.into(),
                challenge_index: w.sector_challenge_index,
            }
        })
        .collect::<Vec<_>>();

    Ok(EPostProof {
        proof,
        post_rand: pi.vrfout.clone(),
        candidates,
    })
}
