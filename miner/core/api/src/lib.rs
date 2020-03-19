use anyhow::Result;

use plum_bigint::BigInt;

use types::{
    Address, BlockMsg, EPostProof, Message, MessageReceipt, SignedMessage, Ticket, TipSet,
    TipSetKey,
};

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
    fn chain_head(&self) -> Result<&TipSet>;
    fn chain_tip_set_weight(&self, _: &TipSet) -> Result<u128>;
    fn chain_get_randomness(&self, _: &TipSetKey, _: i64) -> Result<Vec<u8>>;

    // syncer
    fn sync_submit_block(&self, _: BlockMsg) -> Result<()>;

    // messages
    fn mpool_pending(&self, _: &TipSet) -> Result<Vec<SignedMessage>>;

    // miner
    fn miner_create_block(
        &self,
        _: &Address,
        _: &TipSet,
        _: &Ticket,
        _: &EPostProof,
        _: Vec<SignedMessage>,
        _: u64,
        _: u64,
    ) -> Result<BlockMsg>;

    // other
    fn state_miner_power(&self, _: &Address, _: &TipSet) -> Result<MinerPower>;
    fn state_miner_worker(&self, _: &Address, _: &TipSet) -> Result<Address>;
    fn state_miner_sector_size(&self, _: &Address, _: &TipSet) -> Result<u64>;
    fn state_miner_proving_set(&self, _: &Address, _: &TipSet) -> Result<Vec<ChainSectorInfo>>;

    fn state_call(&self, _: &Message, _: &TipSet) -> Result<MethodCall>;
}
