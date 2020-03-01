use anyhow::{anyhow, Result};
use api::FullNode;
use byteorder::{LittleEndian, WriteBytesExt};
use paired::bls12_381::Bls12;
use sectorbuilder::EPostCandidate;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use types::{Address, CborBigInt, EPostProof, EPostTicket, Protocol, TipSet};

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

#[inline]
fn sha256_sum(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(input);
    let result = hasher.result();
    result[..].into()
}

pub const SectorChallengeRatioDiv: u64 = 25;
fn election_post_challenge_count(sectors: u64, faults: u64) -> u64 {
    if sectors - faults == 0 {
        return 0;
    }
    (sectors - faults - 1) / SectorChallengeRatioDiv + 1
}

// TODO: impl is_ticket_winner in block_header?
fn is_ticket_winner(partial_ticket: &[u8], ssizeI: u64, snum: u64, totpow: &CborBigInt) -> bool {
    let ssize = CborBigInt(ssizeI.into());
    let ssampled = election_post_challenge_count(snum, 0);

    let h = sha256_sum(partial_ticket);

    // TODO:

    true
}

pub fn is_round_winner<A: FullNode, E: ElectionPoStProver>(
    ts: &TipSet,
    round: u64,
    miner: &Address,
    epp: &E,
    a: &A,
) -> Result<Option<ProofInput>> {
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
            assert!(s.comm_r.len() == 32);
            comm_r.copy_from_slice(&s.comm_r[..32]);
            PublicSectorInfo {
                sector_id: s.sector_id,
                comm_r,
            }
        })
        .collect::<Vec<_>>();
    let sector_infos_len = sector_infos.len() as u64;
    let sectors = SortedPublicSectorInfo::new(sector_infos);

    let r = a.chain_get_randomness(&ts.key(), EcRandomnessLookback)?;
    let vrfout = compute_vrf(&mworker, miner, DSepElectionPost, r)?;
    let hvrf = sha256_sum(&vrfout);

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

fn hash_vrf_base(personalization: u64, miner: &Address, input: Vec<u8>) -> Result<Vec<u8>> {
    if miner.protocol() != Protocol::ID {
        return Err(anyhow!(
            "miner address for computing vrf must be an ID address"
        ));
    }

    let mut p = vec![];
    p.write_u64::<LittleEndian>(personalization).unwrap();

    let mut hasher = Sha256::new();

    hasher.input(p);
    hasher.input([0]);
    hasher.input(input);
    hasher.input([0]);
    hasher.input(miner.as_bytes());

    let result = hasher.result();

    Ok(result[..].into())
}

pub fn compute_vrf(worker: &Address, miner: &Address, p: u64, input: Vec<u8>) -> Result<Vec<u8>> {
    let sig_input = hash_vrf_base(p, miner, input)?;

    if miner.protocol() != Protocol::BLS {
        return Err(anyhow!("miner worker address was not a BLS key"));
    }

    // TODO handle private key properly
    let worker_privkey = b"private key";
    let signature = sigs::bls_sign(worker_privkey, &sig_input)?;

    Ok(signature)
}

#[test]
fn hash_vrf_base_should_work() {
    let expected = [
        205, 222, 242, 112, 187, 202, 132, 201, 228, 22, 159, 222, 95, 177, 98, 0, 44, 122, 186,
        244, 226, 147, 152, 214, 117, 55, 203, 184, 188, 220, 52, 8,
    ];

    let p = 789u64;
    let id_addr = Address::new_id_addr(123).unwrap();
    let input = [1, 3, 5, 7, 9].to_vec();
    let sig_input = hash_vrf_base(p, &id_addr, input).unwrap();
    assert_eq!(sig_input, &expected[..]);
}

#[test]
fn using_the_right_sha256() {
    let mut hasher = Sha256::new();
    let input = [1, 3, 5, 7, 9].to_vec();
    hasher.input(input);
    let out: Vec<u8> = hasher.result()[..].into();
    let expected = [
        119, 111, 2, 67, 168, 67, 53, 240, 218, 36, 102, 101, 210, 157, 114, 73, 200, 95, 223, 65,
        110, 133, 45, 49, 228, 194, 172, 23, 38, 249, 116, 101,
    ];
    assert_eq!(out, &out[..]);
}
