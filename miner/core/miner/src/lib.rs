/*
mod error;
mod utils;

use anyhow::{anyhow, Context};
use api::SyncFullNodeApi;
use gen::ElectionPoStProver;
use log::{debug, error, info, warn};
use plum_address::Address;
use plum_block::BlockMsg;
use plum_crypto::{compute_vrf, VrfPrivateKey, VrfProof};
use plum_message::SignedMessage;
use plum_ticket::{EPostProof, Ticket};
use plum_tipset::Tipset;
use std::future::Future;
use std::sync::{Arc, RwLock};
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

pub use crate::error::Error;

///
pub const BLOCK_DELAY: u64 = 45;
///
pub const PROPAGATION_DELAY: u64 = 5;
/// Maximum number of messages to be included in a Block.
pub const BLOCK_MESSAGE_LIMIT: u64 = 512;

pub type Result<T> = anyhow::Result<T, Error>;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MiningBase {
    pub ts: Tipset,
    pub null_rounds: u64,
}

impl MiningBase {
    pub fn new(ts: Tipset) -> Self {
        Self {
            ts,
            null_rounds: 0u64,
        }
    }
}

pub struct BeaconEntry {
    pub round: u64,
    pub data: Vec<u8>,
    pub prev_round: u64,
}

/// Each miner holds the worker private key to sign the Block without using the API.
#[derive(Clone)]
pub struct Miner<Api, E> {
    api: Api,
    epp: E,
    owner: Address,
    worker_priv_key: VrfPrivateKey,
    last_work: Option<MiningBase>,
}

impl<Api: SyncFullNodeApi, E: 'static + ElectionPoStProver> Miner<Api, E> {
    // TODO: support multiple miners?
    pub fn new(
        &mut self,
        api: Api,
        epp: E,
        owner: Address,
        worker_priv_key: VrfPrivateKey,
    ) -> (Arc<RwLock<Self>>, Box<dyn Future<Output = ()> + 'static>) {
        let miner = Arc::new(RwLock::new(Self {
            api,
            epp,
            owner,
            worker_priv_key,
            last_work: None,
        }));
        let mining_future = start_mining(miner.clone());
        (miner, Box::new(mining_future))
    }

    /// Check whether the tipset of chain head is heavier than the one of miner's last_work,
    /// update last_work when the tipset of chain head is heavier.
    ///
    /// TODO: use self.last_work directly as we actually have ensured it's the best work.
    fn get_best_mining_candidate(&mut self) -> Result<MiningBase> {
        let best_ts = self.api.chain_head_sync()?;

        if let Some(last_work) = &self.last_work {
            if last_work.ts == best_ts {
                return Ok(last_work.clone());
            }
            let best_ts_weight = self.api.chain_tipset_weight_sync(best_ts.key())?;
            let last_ts_weight = self.api.chain_tipset_weight_sync(last_work.ts.key())?;
            if best_ts_weight < last_ts_weight {
                return Ok(last_work.clone());
            }
        }

        let best_work = MiningBase {
            ts: best_ts.clone(),
            null_rounds: 0u64,
        };

        self.last_work = Some(best_work.clone());

        Ok(best_work)
    }

    /// Constructs a new Block using the API.
    fn create_block(
        &self,
        base: &MiningBase,
        addr: &Address,
        ticket: &Ticket,
        proof: &EPostProof,
        pending: Vec<SignedMessage>,
    ) -> Result<BlockMsg> {
        let msgs = select_messages(&self.api, &base.ts, pending)?;
        let uts = base.ts.min_timestamp() + BLOCK_DELAY * (base.null_rounds + 1);
        let blk_height = base.ts.height() + base.null_rounds + 1;
        Ok(self.api.miner_create_block_sync(
            &addr,
            base.ts.key(),
            ticket,
            proof,
            &msgs,
            blk_height,
            uts,
        )?)
    }

    /// Returns true if the miner power is not zero.
    #[inline]
    fn has_power(&self, addr: &Address, ts: &Tipset) -> Result<bool> {
        let power = self
            .api
            .state_miner_power_sync(addr, ts.key())
            .context(Error::MaybeSlashed(addr.clone()))?;
        Ok(power.miner_power > 0.into())
    }

    /// Note the `personalization` passed to `compute_vrf`.
    #[inline]
    fn compute_vrf(&self, addr: &Address, input: &[u8]) -> VrfProof {
        return compute_vrf(&self.worker_priv_key, gen::D_SEP_TICKET, input, addr);
    }

    /// `Ticket` is derived from the minimum ticket from the parent tipset’s block headers, e.g.,
    /// the best Tipset when we are about to mine a new block,
    #[inline]
    fn compute_ticket(&self, addr: &Address, base: &MiningBase) -> Result<Ticket> {
        let vrf_base = base.ts.min_ticket().vrf_proof.clone();
        let vrf_out = self.compute_vrf(addr, &vrf_base);
        Ok(Ticket {
            vrf_proof: vrf_out.as_bytes(),
        })
    }

    /// Try mining a Block.
    fn mine_one(&self, base: &mut MiningBase) -> Result<BlockMsg> {
        debug!("attempting to mine a block, tipset: {:?}", base.ts.cids());

        let start = Instant::now();

        let round = base.ts.height() + base.null_rounds + 1;

        // let base_info = self.api.miner_get_base_info();
        let base_info: Option<u8> = None;

        if base_info.is_none() {
            base.null_rounds += 1;
            return Err(anyhow!("failed to get mining base info").into());
        }

        // slashed or just have no power yet.
        // if !self.has_power(&self.owner, &base.ts)? {
        // base.null_rounds += 1;
        // return Err(Error::NoMiningPower(self.owner.clone()));
        // }

        info!(
            "Time delta now and our mining base (nulls: {})",
            base.null_rounds
        );

        let ticket = self.compute_ticket(&self.owner, base)?;

        let proof_in = gen::is_round_winner(
            &base.ts,
            base.ts.height() + base.null_rounds + 1,
            &self.owner,
            &self.epp,
            &self.api,
            &self.worker_priv_key,
        )?;

        if proof_in.is_none() {
            base.null_rounds += 1;
            return Err(Error::EmptyProofInput);
        }

        // get pending message early.
        let pending = self.api.mpool_pending_sync(base.ts.key())?;

        let proof = gen::compute_proof(&self.epp, &proof_in.unwrap())?;

        let b = self.create_block(base, &self.owner, &ticket, &proof, pending)?;

        let elapsed = start.elapsed().as_secs();

        info!(
            "mined new block, cid:{}, height:{}, took: {}s",
            b.cid(),
            b.header.height,
            elapsed
        );

        if elapsed > BLOCK_DELAY {
            warn!("CAUTION: block production took longer than the block delay. Your computer may not be fast enough to keep up");
        }

        Ok(b)
    }
}

/// Starts the mining task actually.
async fn start_mining<Api: SyncFullNodeApi, E: 'static + ElectionPoStProver>(
    miner: Arc<RwLock<Miner<Api, E>>>,
) {
    let mut last_base = None;

    let mut miner = miner.write().unwrap();

    loop {
        let prebase = match miner.get_best_mining_candidate() {
            Ok(x) => x,
            Err(err) => {
                error!("failed to get best mining candidate: {:?}", err);
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        // Wait until propagation delay period after block we plan to mine on.
        crate::utils::wait_until(prebase.ts.min_timestamp() + PROPAGATION_DELAY);

        let mut base = match miner.get_best_mining_candidate() {
            Ok(x) => x,
            Err(err) => {
                error!("failed to get best mining candidate: {:?}", err);
                continue;
            }
        };

        if let Some(ref l_base) = last_base {
            if base == *l_base {
                warn!(
                    "BestMiningCandidate from the previous round: {:?} (nulls:{})",
                    l_base.ts.cids(),
                    l_base.null_rounds
                );
                crate::utils::sleep(BLOCK_DELAY);
                continue;
            }
        }

        last_base = Some(base.clone());

        match miner.mine_one(&mut base) {
            Ok(blk) => {
                info!("miner {} mined one block successfully", miner.owner);
                // TODO: optimize sync_submit_block_sync logic using Lru like the go version?
                if let Err(err) = miner.api.sync_submit_block_sync(&blk) {
                    error!("failed to submit newly mined block: {:?}", err);
                }
            }
            Err(err) => {
                error!("mining block miner for {} failed: {:?}", miner.owner, err);
            }
        }
    }
}

/// Select at most [`BLOCK_MESSAGE_LIMIT`] valid messages given the chunk of [`SignedMessage`].
///
/// Currently we perform the following validations:
///
/// - the nonce of message should always match the sender's.
/// - the sender has enough balance.
fn select_messages<Api: SyncFullNodeApi>(
    actor_lookup: &Api,
    ts: &Tipset,
    msgs: Vec<SignedMessage>,
) -> Result<Vec<SignedMessage>> {
    let mut out = Vec::new();
    let mut actors_in_pool = HashMap::new();

    for msg in msgs.into_iter() {
        let from = msg.message.from.clone();

        if !actors_in_pool.contains_key(&from) {
            if let Ok(actor) = actor_lookup.state_get_actor_sync(&from, ts.key()) {
                actors_in_pool.insert(from.clone(), actor);
            } else {
                warn!(
                    "[select_messages]failed to check message sender balance, skipping message: {:?}",
                    msg
                );
                continue;
            }
        }

        let from_in_pool = actors_in_pool
            .get_mut(&from)
            .expect("Every sender's actor in mempool has been initialized from the state; qed");

        if from_in_pool.balance < msg.message.required_funds() {
            warn!(
                "[select_messages]message in mempool does not have enough funds: {}, skipping",
                msg.cid()
            );
            continue;
        }

        if msg.message.nonce != from_in_pool.nonce {
            warn!(
                "[select_messages]message in mempool has a different nonce, message nonce:{}, expected nonce of sender:{}",
                msg.message.nonce, from_in_pool.nonce
            );
            continue;
        }

        from_in_pool.nonce += 1;
        from_in_pool.balance -= msg.message.required_funds();

        out.push(msg);

        if out.len() >= BLOCK_MESSAGE_LIMIT as usize {
            break;
        }
    }

    Ok(out)
}
*/
