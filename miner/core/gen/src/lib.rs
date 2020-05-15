/* Rewrite gen
 *
use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use api::FullNodeApi;
use sectorbuilder::EPostCandidate;

use plum_address::Address;
use plum_bigint::BigInt;
use plum_crypto::{compute_vrf, VrfPrivateKey, VrfProof};
use plum_hashing::sha256;
use plum_sector::{PoStProof, PostTicket};
use plum_tipset::Tipset;

/// A constant used as `personalization` in `compute_proof()` of miner crate.
pub const D_SEP_TICKET: u64 = 1;
/// A constant used as `personalization` in `compute_proof()`.
pub const D_SEP_ELECTION_POST: u64 = 2;

/// Miner draws a randomness ticket from the randomness chain from a given epoch `SPC_LOOKBACK_POST` back.
/// Noted as EcRandomnessLookback in lotus.
pub const SPC_LOOKBACK_POST: u64 = 300;

/// CommitmentBytesLen is the number of bytes in a CommR, CommD, CommP, and CommRStar.
pub const CommitmentBytesLen: usize = 32;

const ExpectedLeadersPerEpoch: u64 = 5;

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
        // /Users/xuliucheng/src/github.com/filecoin-project/lotus/extern/filecoin-ffi NewSortedPublicSectorInfo
        // FIXME: Compare comm_r
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

pub const sha256bits: u64 = 256;
pub const SectorChallengeRatioDiv: u64 = 25;

/// Returns the size of sampled sectors against the proving set with the faults sectors excluded.
///
/// numSectorsMiner = len(miner.provingSet)
/// numSectorsSampled = ceil(numSectorsMiner * EPoStSampleRate)
fn election_post_challenge_count(num_sectors: u64, faults: u64) -> u64 {
    if num_sectors - faults == 0 {
        return 0;
    }

    (num_sectors - faults - 1) / SectorChallengeRatioDiv + 1
}

/// Returns true if the `partial_ticket` is a winner.
///
/// const maxChallengeTicketSize = 2^len(H)
///
/// def TicketIsWinner(challengeTicket):
///     Check that `ChallengeTicket < Target`
///     return challengeTicket * networkPower * numSectorsSampled < activePowerInSector * EC.ExpectedLeadersPerEpoch * maxChallengeTicketSize * numSectorsMiner
///
///	// Conceptually we are mapping the pseudorandom, deterministic hash output of the challenge ticket onto [0,1]
/// by dividing by 2^HashLen and comparing that to the sector's target.
///
/// if the challenge ticket hash / max hash val < sectorPower / totalPower * ec.ExpectedLeaders * numSectorsMiner / numSectorsSampled
/// it is a winning challenge ticket.
///
/// note that the sectorPower may differ based on the challenged sector
///
/// lhs := challengeTicket * totalPower * numSectorsSampled
/// rhs := maxTicket * activeSectorPower * numSectorsMiner * self.ExpectedLeaders
///
/// We want the miner's expected wins over time to be equal to:
/// w = minerPower/networkPower * EC.ExpectedLeadersPerEpoch
///
/// The miner's likelihood of winning an election in any epoch is, for C tickets randomly drawn
///
/// W = C * target
///   = C * P_i/networkPower * EC.ExpectedLeadersPerEpoch * N/C
///   = N * P_i/networkPower * EC.ExpectedLeadersPerEpoch
///
/// # Argument
///
/// * `partial_ticket` -
/// * `sector_size` -
/// * `num_sectors` - Size of miner's proving set.
/// * `network_power` - Total mining power cross the network.
fn is_ticket_winner(
    partial_ticket: &[u8],
    sector_size: u64,
    num_sectors: u64,
    network_power: &BigInt,
) -> bool {
    let ssize: BigInt = sector_size.into();
    let ssampled: BigInt = election_post_challenge_count(num_sectors, 0).into();

    let h = sha256(partial_ticket);

    let lhs = BigInt::from_signed_bytes_be(&h) * network_power * ssampled;

    let rhs = (ssize << sha256bits as usize) * num_sectors * ExpectedLeadersPerEpoch;

    lhs < rhs
}

#[inline]
fn election_post_compute_vrf(
    worker_priv_key: &VrfPrivateKey,
    input: &[u8],
    owner: &Address,
) -> VrfProof {
    compute_vrf(&worker_priv_key, D_SEP_ELECTION_POST, input, owner)
}

pub fn is_round_winner<Api: FullNodeApi + Sync, E: ElectionPoStProver>(
    ts: &Tipset,
    round: u64,
    owner: &Address,
    epp: &E,
    api: &Api,
    worker_priv_key: &VrfPrivateKey,
) -> Result<Option<ProofInput>> {
    let ts_key = ts.key();

    // FIXME i64 or u64?
    // round is essentially the height IMO.
    let randomness = api.chain_get_randomness_sync(ts_key, (round - SPC_LOOKBACK_POST) as i64)?;

    // Needless as we already have the private key.
    // let worker_addr = api.state_miner_worker(owner, ts_key).await;

    // The differene with `compute_vrf` in miner:
    // 1. input
    // 2. personalization
    let vrfout = election_post_compute_vrf(worker_priv_key, &randomness, owner).as_bytes();

    let proving_set = api.state_miner_proving_set_sync(owner, ts_key)?;

    if proving_set.is_empty() {
        return Ok(None);
    }

    let sector_infos = proving_set
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

    let hvrf = sha256(&vrfout);
    let candidates = epp.generate_candidates(&sectors, &hvrf)?;

    let power = api.state_miner_power_sync(owner, ts_key)?;
    let sector_size = api.state_miner_sector_size_sync(owner, ts_key)?;

    // TODO: partial_ticket?
    let winners = candidates
        .into_iter()
        .filter(|c| {
            is_ticket_winner(
                &sectorbuilder::fr32::fr_into_bytes(&c.partial_ticket),
                sector_size,
                sector_infos_len,
                &power.total_power,
            )
        })
        .collect::<Vec<_>>();

    // no winners, sad
    if winners.is_empty() {
        return Ok(None);
    }

    Ok(Some(ProofInput {
        sectors,
        hvrf: hvrf.to_vec(),
        winners,
        vrfout,
    }))
}

pub fn compute_proof<E: ElectionPoStProver>(epp: &E, pi: &ProofInput) -> Result<PoStProof> {
    let proof = epp
        .compute_proof(&pi.sectors, &pi.hvrf, &pi.winners)
        .map_err(|e| anyhow!("Failed to compute snark for election proof: {:?}", e))?;

    let candidates = pi
        .winners
        .iter()
        .map(|w| {
            // TODO: sector_id uses u64 or SectorId?
            // TODO: partial is actually a fixed size array [u8; 32], not an arbitray Vec<u8>.
            let partial = sectorbuilder::fr32::fr_into_bytes(&w.partial_ticket);
            PostTicket {
                partial,
                sector_id: w.sector_id.into(),
                challenge_index: w.sector_challenge_index,
            }
        })
        .collect::<Vec<_>>();

    Ok(PoStProof {
        proof,
        post_rand: pi.vrfout.clone(),
        candidates,
    })
}
*/
