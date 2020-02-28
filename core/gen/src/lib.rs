use anyhow::Result;
use api::FullNode;
use types::{Address, EPostProof, TipSet};

pub const DSepTicket: u64 = 1;
pub const DSepElectionPost: u64 = 2;

pub trait ElectionPoStProver {
    fn generate_candidates();
    fn compute_proof();
}

pub struct ProofInputReal {
    // sectors sectorbuilder.SortedPublicSectorInfo
    hvrf: Vec<u8>,
    // winners []sectorbuilder.EPostCandidate
    vrfout: Vec<u8>,
}

pub type ProofInput = u8;

pub const EcRandomnessLookback: i64 = 300;

pub type PublicSectorInfo = u8;

pub fn is_round_winner<A: FullNode, E: ElectionPoStProver>(
    ts: &TipSet,
    round: u64,
    miner: &Address,
    epp: &E,
    a: &A,
) -> Result<Option<ProofInput>> {
    let r = a.chain_get_randomness(&ts.key(), EcRandomnessLookback)?;
    let mworker = a.state_miner_worker(miner, ts)?;

    // let vrf_out = compute_proof();
    let proving_set = a.state_miner_proving_set(miner, ts)?;

    // if len(proving_set) == 0

    // let sectors

    let pow = a.state_miner_power(miner, ts)?;

    let ssize = a.state_miner_sector_size(miner, ts)?;

    // for candidates
    //
    // if len(winners) == 0

    Ok(Some(0u8))
}

pub fn compute_proof<E: ElectionPoStProver>(epp: &E, pi: &ProofInput) -> Result<EPostProof> {
    Ok(EPostProof {
        proof: Vec::new(),
        post_rand: Vec::new(),
        candidates: Vec::new(),
    })
}

fn hash_vrf_base(personalization: u64, miner: &Address, input: Vec<u8>) -> Result<Vec<u8>> {
    Ok(Vec::new())
}

pub fn compute_vrf(worker: &Address, miner: &Address, p: u64, input: Vec<u8>) -> Result<Vec<u8>> {
    let sig_input = hash_vrf_base(p, miner, input)?;

    // TODO Impl WalletSign seperately
    // sign
    // let sig = sign()
    //
    //
    Ok(Vec::new())
}
