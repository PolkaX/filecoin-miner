use anyhow::Result;

use plum_address::Address;
use plum_bigint::BigInt;
use plum_block::BlockMsg;
use plum_message::{MessageReceipt, SignedMessage, UnsignedMessage};
use plum_ticket::{EPostProof, Ticket};
use plum_tipset::{Tipset, TipsetKey};

#[derive(Debug, Clone)]
pub struct MinerPower {
    pub miner_power: BigInt,
    pub total_power: BigInt,
}

pub struct ChainSectorInfo {
    pub sector_id: u64,
    pub comm_d: Vec<u8>,
    pub comm_r: Vec<u8>,
}

pub type MethodCall = std::result::Result<MessageReceipt, &'static str>;

pub trait FullNode {
    // chain
    fn chain_head(&self) -> Result<&Tipset>;
    fn chain_tip_set_weight(&self, _: &Tipset) -> Result<u128>;
    fn chain_get_randomness(&self, _: &TipsetKey, _: i64) -> Result<Vec<u8>>;

    // syncer
    fn sync_submit_block(&self, _: BlockMsg) -> Result<()>;

    // messages
    fn mpool_pending(&self, _: &Tipset) -> Result<Vec<SignedMessage>>;

    // miner
    fn miner_create_block(
        &self,
        _: &Address,
        _: &Tipset,
        _: &Ticket,
        _: &EPostProof,
        _: Vec<SignedMessage>,
        _: u64,
        _: u64,
    ) -> Result<BlockMsg>;

    // other
    fn state_miner_power(&self, _: &Address, _: &Tipset) -> Result<MinerPower>;
    fn state_miner_worker(&self, _: &Address, _: &Tipset) -> Result<Address>;
    fn state_miner_sector_size(&self, _: &Address, _: &Tipset) -> Result<u64>;
    fn state_miner_proving_set(&self, _: &Address, _: &Tipset) -> Result<Vec<ChainSectorInfo>>;

    fn state_call(&self, _: &UnsignedMessage, _: &Tipset) -> Result<MethodCall>;
}
