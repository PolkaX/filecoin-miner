use std::{
    collections::HashSet,
    thread,
    time::{Duration, Instant, SystemTime},
};

use chrono::{DateTime, NaiveDateTime, Utc};
use cid::Cid;
use log::{debug, error, info, warn};
use lru::LruCache;
use thiserror::Error;

use plum_address::{Address, Network};
use plum_bigint::BigInt;
use plum_block::{BlockHeader, BlockMsg};
use plum_crypto::{compute_vrf, Signature, SignatureType};
use plum_message::SignedMessage;
use plum_ticket::{EPostProof, Ticket};
use plum_tipset::Tipset;

use api::SyncFullNodeApi;
use gen::ElectionPoStProver;

pub const BLOCK_DELAY: u64 = 45;
pub const PROPAGATION_DELAY: u64 = 5;

pub const BLOCK_MESSAGE_LIMIT: u64 = 512;

pub type Result<T> = anyhow::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Check if miner {0} is slashed")]
    MaybeSlashed(Address),
    #[error("`{0}` has no miner power")]
    NoMiningPower(Address),
    #[error("Empty ProofInput")]
    EmptyProofInput,
    #[error("Tipset error {0}")]
    TipsetError(#[from] plum_tipset::TipsetError),
    #[error("{0}")]
    ApiError(#[from] api::ApiError),
    #[error("anyhow error {0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("other error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

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

pub struct Miner<Api, E> {
    api: Api,
    epp: E,
    addresses: Vec<Address>,
    last_work: Option<MiningBase>,
    mined_block_heights: LruCache<u64, Vec<Address>>,
}

fn now_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn wait_until(end_timestamp: u64) {
    thread::sleep(Duration::from_secs(end_timestamp - now_timestamp()))
}

fn dummy_cid() -> Cid {
    let cid: Cid = "bafyreicmaj5hhoy5mgqvamfhgexxyergw7hdeshizghodwkjg6qmpoco7i"
        .parse()
        .unwrap();
    cid
}

fn dummy_block_header(cid: Cid) -> BlockHeader {
    let id = 123;
    BlockHeader {
        miner: Address::new_id_addr(id).unwrap(),
        ticket: Ticket {
            vrf_proof: Vec::new(),
        },
        epost_proof: EPostProof {
            proof: Vec::new(),
            post_rand: Vec::new(),
            candidates: Vec::new(),
        },
        parents: Vec::new(),
        parent_weight: 0u128.into(),
        height: 0u64,
        parent_state_root: cid.clone(),
        parent_message_receipts: cid.clone(),
        messages: cid,
        bls_aggregate: Signature::new_bls("boo! im a signature"),
        timestamp: 0u64,
        block_sig: Signature::new_bls("boo! im a signature"),
        fork_signaling: 0u64,
    }
}

fn sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}

impl<Api: SyncFullNodeApi, E: ElectionPoStProver> Miner<Api, E> {
    pub fn register(&mut self) {}

    pub fn unregister(&mut self) {}

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

    fn wait(&self) -> Result<()> {
        Ok(())
    }

    fn create_block(
        &self,
        base: &MiningBase,
        addr: &Address,
        ticket: &Ticket,
        proof: &EPostProof,
        pending: Vec<SignedMessage>,
    ) -> Result<BlockMsg> {
        let mut msgs = select_messages(0u8, &base.ts, pending)?;

        if msgs.len() > BLOCK_MESSAGE_LIMIT as usize {
            error!("SelectMessages returned too many messages: {}", msgs.len());
            msgs = msgs
                .into_iter()
                .take(BLOCK_MESSAGE_LIMIT as usize)
                .collect();
        }

        let uts = base.ts.min_timestamp() + BLOCK_DELAY * (base.null_rounds + 1);

        let nheight = base.ts.height() + base.null_rounds + 1;

        Ok(self.api.miner_create_block_sync(
            &addr,
            base.ts.key(),
            ticket,
            proof,
            &msgs,
            nheight,
            uts,
        )?)
    }

    fn has_power(&self, addr: &Address, ts: &Tipset) -> Result<bool> {
        let power = self
            .api
            .state_miner_power_sync(addr, ts.key())
            .map_err(|_| Error::MaybeSlashed(addr.clone()))?;
        Ok(power.miner_power > 0.into())
    }

    fn get_miner_worker(&self, addr: &Address, ts: Option<&Tipset>) -> Result<Address> {
        // TODO
        // api.state_call()
        //
        Ok(addr.clone())
    }

    fn compute_vrf(&self, miner_addr: &Address, input: Vec<u8>) -> Result<Vec<u8>> {
        todo!()
        /*
        let worker_addr = self.get_miner_worker(miner_addr, None)?;
        Ok(compute_vrf(&worker_addr, gen::DSepTicket, input, miner_addr).as_bytes())
        */
    }

    fn compute_ticket(&self, addr: &Address, base: &MiningBase) -> Result<Ticket> {
        let vrf_base = base.ts.min_ticket().vrf_proof.clone();
        let vrf_out = self.compute_vrf(addr, vrf_base)?;
        Ok(Ticket { vrf_proof: vrf_out })
    }

    fn mine_one(&self, addr: &Address, base: &mut MiningBase) -> Result<BlockMsg> {
        debug!("attempting to mine a block, tipset: {:?}", base.ts.cids());

        let start = Instant::now();

        // slashed or just have no power yet.
        if !self.has_power(addr, &base.ts)? {
            base.null_rounds += 1;
            return Err(Error::NoMiningPower(addr.clone()));
        }

        info!(
            "Time delta now and our mining base (nulls: {})",
            base.null_rounds
        );

        let ticket = self.compute_ticket(addr, base)?;

        let proof_in = gen::is_round_winner(
            &base.ts,
            base.ts.height() + base.null_rounds + 1,
            addr,
            &self.epp,
            &self.api,
        )?;

        if proof_in.is_none() {
            base.null_rounds += 1;
            return Err(Error::EmptyProofInput);
        }

        let proof_in = gen::ProofInput::default();

        // get pending message early.
        let pending = self.api.mpool_pending_sync(base.ts.key())?;

        let proof = gen::compute_proof(&self.epp, &proof_in)?;

        let b = self.create_block(base, addr, &ticket, &proof, pending)?;

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

    fn mine(&mut self) -> Result<()> {
        let mut last_base = MiningBase::new(Tipset::new(vec![dummy_block_header(dummy_cid())])?);

        loop {
            // TODO: handle stop singal?

            let prebase = match self.get_best_mining_candidate() {
                Ok(x) => x,
                Err(err) => {
                    error!("failed to get best mining candidate: {:?}", err);
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
            };

            // Wait until propagation delay period after block we plan to mine on.
            wait_until(prebase.ts.min_timestamp() + PROPAGATION_DELAY);

            let mut base = match self.get_best_mining_candidate() {
                Ok(x) => x,
                Err(err) => {
                    error!("failed to get best mining candidate: {:?}", err);
                    continue;
                }
            };

            if base == last_base {
                warn!(
                    "BestMiningCandidate from the previous round: {:?} (nulls:{})",
                    last_base.ts.cids(),
                    last_base.null_rounds
                );
                sleep(BLOCK_DELAY);
                continue;
            }

            last_base = base.clone();

            let mut blks = Vec::new();

            // TODO: handle addresses in multiple threads?
            for addr in self.addresses.iter() {
                match self.mine_one(addr, &mut base) {
                    Ok(blk) => blks.push(blk),
                    Err(err) => {
                        error!("mining block miner for {} failed: {:?}", addr, err);
                    }
                }
            }

            if !blks.is_empty() {
                // Check block time
                let blk_time = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(blks[0].header.timestamp as i64, 0),
                    Utc,
                );
                let now = Utc::now();
                // TODO: milliseconds?
                if now < blk_time {
                    thread::sleep((blk_time - now).to_std().expect("Won't panic"));
                } else {
                    warn!(
                        "mined block in the past, block-time: {}, time: {}, duration: {}",
                        blk_time,
                        now,
                        blk_time - now
                    )
                }

                // Check if there is a miner that created two blocks in this round.
                let mut winners = HashSet::new();
                for blk in blks.iter() {
                    if winners.contains(&blk.header.miner) {
                        error!("2 blocks for the same miner. Throwing hands in the air. Report this. It it important, blocks: {:?}", blks);
                        continue;
                    } else {
                        winners.insert(blk.header.miner.clone());
                    }
                }

                for blk in blks {
                    // Check if the blk is in the cache.
                    // If it's already in the cache, no need to run sync_submit_block() then.
                    if let Some(miners) = self.mined_block_heights.get(&blk.header.height) {
                        if miners.contains(&blk.header.miner) {
                            warn!("Created a block at the same height as another block we've created, height:{}, miner:{}, parents:{:?}", blk.header.height, blk.header.miner, blk.header.parents);
                            continue;
                        } else {
                            let v = self
                                .mined_block_heights
                                .get_mut(&blk.header.height)
                                .unwrap();
                            v.push(blk.header.miner.clone());
                        }
                    } else {
                        self.mined_block_heights
                            .put(blk.header.height, vec![blk.header.miner.clone()]);
                    }

                    if let Err(err) = self.api.sync_submit_block_sync(&blk) {
                        error!("failed to submit newly mined block: {:?}", err);
                    }
                }
            } else {
                // next_round
                // next_round = time.unix(base.ts.min_timestamp() + BLOCK_DELAY * base.null_rounds)
                //
                // wait_until( base.ts.min_timestamp() + BLOCK_DELAY * base.null_rounds )
            }
        }
    }
}

pub type ActorLookup = u8;

fn select_messages(
    al: ActorLookup,
    ts: &Tipset,
    msgs: Vec<SignedMessage>,
) -> Result<Vec<SignedMessage>> {
    Ok(Vec::new())
}
