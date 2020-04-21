// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;

use async_std::task::block_on;
use async_trait::async_trait;
use libp2p_core::PeerId;
use serde::{Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};
use plum_block::BlockMsg;
use plum_message::{
    message_receipt_json, unsigned_message_json, MessageReceipt, SignedMessage, UnsignedMessage,
};
use plum_ticket::{epost_proof_json, ticket_json, EPostProof, Ticket};
use plum_tipset::{tipset_json, tipset_key_json, Tipset, TipsetKey};
use plum_types::{Actor, base64};
use plum_vm::{execution_result_json, ExecutionResult};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait StateApi: RpcClient {
    /// if tipset is nil, we'll use heaviest
    async fn state_call(&self, msg: &UnsignedMessage, key: &TipsetKey) -> Result<InvocResult> {
        self.request(
            "StateCall",
            vec![
                crate::helpers::serialize_with(unsigned_message_json::serialize, msg),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    /*
    ///
    async fn state_replay(&self, key: &TipsetKey, cid: &Cid) -> Result<InvocResult>;
    */
    ///
    async fn state_get_actor(&self, addr: &Address, key: &TipsetKey) -> Result<Actor> {
        let actor: crate::helpers::Actor = self
            .request(
                "StateGetActor",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(actor.0)
    }
    /*
    ///
    fn state_read_state(&self, actor: &Actor, key: &TipsetKey) -> Result<ActorState>;
    */
    ///
    async fn state_list_messages(
        &self,
        msg: &UnsignedMessage,
        key: &TipsetKey,
        height: u64,
    ) -> Result<Vec<Cid>> {
        let cids: Vec<crate::helpers::Cid> = self
            .request(
                "StateListMessages",
                vec![
                    crate::helpers::serialize_with(unsigned_message_json::serialize, msg),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                    crate::helpers::serialize(&height),
                ],
            )
            .await?;
        Ok(cids.into_iter().map(|cid| cid.0).collect())
    }

    ///
    async fn state_miner_sectors(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        self.request(
            "StateMinerSectors",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }

    /// Returns a set of all Active sectors for a particular miner `addr`.
    async fn state_miner_proving_set(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        self.request(
            "StateMinerProvingSet",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }

    ///
    async fn state_miner_power(&self, addr: &Address, key: &TipsetKey) -> Result<MinerPower> {
        self.request(
            "StateMinerPower",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }

    /// Returns the worker address given the miner owner address `addr`.
    async fn state_miner_worker(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        let address: crate::helpers::Address = self
            .request(
                "StateMinerWorker",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(address.0)
    }

    ///
    async fn state_miner_peer_id(&self, addr: &Address, key: &TipsetKey) -> Result<PeerId> {
        let peer_id: crate::helpers::PeerId = self
            .request(
                "StateMinerPeerID",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(peer_id.0)
    }
    ///
    async fn state_miner_election_period_start(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<u64> {
        self.request(
            "StateMinerElectionPeriodStart",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_sector_size(&self, addr: &Address, key: &TipsetKey) -> Result<u64> {
        self.request(
            "StateMinerSectorSize",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_faults(&self, addr: &Address, key: &TipsetKey) -> Result<Vec<u64>> {
        self.request(
            "StateMinerFaults",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_pledge_collateral(&self, key: &TipsetKey) -> Result<BigInt> {
        let bigint: crate::helpers::BigInt = self
            .request(
                "StatePledgeCollateral",
                vec![crate::helpers::serialize_with(
                    tipset_key_json::serialize,
                    key,
                )],
            )
            .await?;
        Ok(bigint.0)
    }
    ///
    async fn state_wait_msg(&self, cid: &Cid) -> Result<MsgWait> {
        self.request(
            "StateWaitMsg",
            vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }
    ///
    async fn state_list_miners(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request(
            "StateListMiners",
            vec![crate::helpers::serialize_with(
                tipset_key_json::serialize,
                key,
            )],
        )
        .await
    }
    ///
    async fn state_list_actors(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request(
            "StateListActors",
            vec![crate::helpers::serialize_with(
                tipset_key_json::serialize,
                key,
            )],
        )
        .await
    }

    /*
    ///
    async fn state_market_balance(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<actors::StorageParticipantBalance>;
    ///
    async fn state_market_participants(
        &self,
        key: &TipsetKey,
    ) -> Result<HashMap<String, actors::StorageParticipantBalance>>;
    ///
    async fn state_market_deals(&self, key: &TipsetKey) -> Result<HashMap<String, actors::OnChainDeal>>;
    */
    /// return actors::OnChainDeal
    async fn state_market_storage_deal(
        &self,
        deal_id: u64,
        key: &TipsetKey,
    ) -> Result<OnChainDeal> {
        self.request(
            "StateMarketStorageDeal",
            vec![
                crate::helpers::serialize(&deal_id),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_lookup_id(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        let address: crate::helpers::Address = self
            .request(
                "StateLookupID",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(address.0)
    }
    ///
    async fn state_changed_actors(&self, old: &Cid, new: &Cid) -> Result<HashMap<String, Actor>> {
        let map: HashMap<String, crate::helpers::Actor> = self
            .request(
                "StateChangedActors",
                vec![
                    crate::helpers::serialize_with(cid_json::serialize, old),
                    crate::helpers::serialize_with(cid_json::serialize, new),
                ],
            )
            .await?;
        Ok(map.into_iter().map(|(k, v)| (k, v.0)).collect())
    }
    ///
    async fn state_get_receipt(&self, cid: &Cid, key: &TipsetKey) -> Result<MessageReceipt> {
        let msg_receipt: crate::helpers::MessageReceipt = self
            .request(
                "StateGetReceipt",
                vec![
                    crate::helpers::serialize_with(cid_json::serialize, cid),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(msg_receipt.0)
    }
    ///
    async fn state_miner_sector_count(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<MinerSectors> {
        self.request(
            "StateMinerSectorCount",
            vec![
                crate::helpers::serialize_with(address_json::serialize, addr),
                crate::helpers::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_compute(
        &self,
        height: u64,
        msgs: &[UnsignedMessage],
        key: &TipsetKey,
    ) -> Result<Cid> {
        let msgs = msgs
            .iter()
            .cloned()
            .map(crate::helpers::UnsignedMessage)
            .collect::<Vec<_>>();
        let cid: crate::helpers::Cid = self
            .request(
                "StateCompute",
                vec![
                    crate::helpers::serialize(&height),
                    crate::helpers::serialize(&msgs),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(cid.0)
    }

    ///
    async fn msig_get_available_balance(&self, addr: &Address, key: &TipsetKey) -> Result<BigInt> {
        let bigint: crate::helpers::BigInt = self
            .request(
                "MsigGetAvailableBalance",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(bigint.0)
    }

    /// This is on StateAPI because miner.Miner requires this, and MinerAPI requires miner.Miner
    async fn miner_create_block(
        &self,
        addr: &Address,
        parent_key: &TipsetKey,
        ticket: &Ticket,
        proof: &EPostProof,
        msgs: &[SignedMessage],
        height: u64,
        ts: u64,
    ) -> Result<BlockMsg> {
        let msgs = msgs
            .iter()
            .cloned()
            .map(crate::helpers::SignedMessage)
            .collect::<Vec<_>>();
        let block_msg: crate::helpers::BlockMsg = self
            .request(
                "StateMinerFaults",
                vec![
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(tipset_key_json::serialize, parent_key),
                    crate::helpers::serialize_with(ticket_json::serialize, ticket),
                    crate::helpers::serialize_with(epost_proof_json::serialize, proof),
                    crate::helpers::serialize(&msgs),
                    crate::helpers::serialize(&height),
                    crate::helpers::serialize(&ts),
                ],
            )
            .await?;
        Ok(block_msg.0)
    }
}

pub trait SyncStateApi: StateApi {
    /// if tipset is nil, we'll use heaviest
    fn state_call_sync(&self, msg: &UnsignedMessage, key: &TipsetKey) -> Result<InvocResult> {
        block_on(async { StateApi::state_call(self, msg, key).await })
    }
    /*
    ///
    fn state_replay_sync(&self, key: &TipsetKey, cid: &Cid) -> Result<InvocResult>;
    */
    ///
    fn state_get_actor_sync(&self, addr: &Address, key: &TipsetKey) -> Result<Actor> {
        block_on(async { StateApi::state_get_actor(self, addr, key).await })
    }
    /*
    ///
    fn state_read_state_sync(&self, actor: &Actor, key: &TipsetKey) -> Result<ActorState>;
    */
    ///
    fn state_list_messages_sync(
        &self,
        msg: &UnsignedMessage,
        key: &TipsetKey,
        height: u64,
    ) -> Result<Vec<Cid>> {
        block_on(async { StateApi::state_list_messages(self, msg, key, height).await })
    }

    ///
    fn state_miner_sectors_sync(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        block_on(async { StateApi::state_miner_sectors(self, addr, key).await })
    }
    ///
    fn state_miner_proving_set_sync(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        block_on(async { StateApi::state_miner_proving_set(self, addr, key).await })
    }
    ///
    fn state_miner_power_sync(&self, addr: &Address, key: &TipsetKey) -> Result<MinerPower> {
        block_on(async { StateApi::state_miner_power(self, addr, key).await })
    }
    ///
    fn state_miner_worker_sync(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        block_on(async { StateApi::state_miner_worker(self, addr, key).await })
    }
    ///
    fn state_miner_peer_id_sync(&self, addr: &Address, key: &TipsetKey) -> Result<PeerId> {
        block_on(async { StateApi::state_miner_peer_id(self, addr, key).await })
    }
    ///
    fn state_miner_election_period_start_sync(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<u64> {
        block_on(async { StateApi::state_miner_election_period_start(self, addr, key).await })
    }
    ///
    fn state_miner_sector_size_sync(&self, addr: &Address, key: &TipsetKey) -> Result<u64> {
        block_on(async { StateApi::state_miner_sector_size(self, addr, key).await })
    }
    ///
    fn state_miner_faults_sync(&self, addr: &Address, key: &TipsetKey) -> Result<Vec<u64>> {
        block_on(async { StateApi::state_miner_faults(self, addr, key).await })
    }
    ///
    fn state_pledge_collateral_sync(&self, key: &TipsetKey) -> Result<BigInt> {
        block_on(async { StateApi::state_pledge_collateral(self, key).await })
    }
    ///
    fn state_wait_msg_sync(&self, cid: &Cid) -> Result<MsgWait> {
        block_on(async { StateApi::state_wait_msg(self, cid).await })
    }
    ///
    fn state_list_miners_sync(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        block_on(async { StateApi::state_list_miners(self, key).await })
    }
    ///
    fn state_list_actors_sync(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        block_on(async { StateApi::state_list_actors(self, key).await })
    }

    /*
    ///
    fn state_market_balance_sync(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<actors::StorageParticipantBalance>;
    ///
    fn state_market_participants_sync(
        &self,
        key: &TipsetKey,
    ) -> Result<HashMap<String, actors::StorageParticipantBalance>>;
    ///
    fn state_market_deals_sync(&self, key: &TipsetKey) -> Result<HashMap<String, actors::OnChainDeal>>;
    */
    ///
    fn state_market_storage_deal_sync(&self, deal_id: u64, key: &TipsetKey) -> Result<OnChainDeal> {
        block_on(async { StateApi::state_market_storage_deal(self, deal_id, key).await })
    }
    ///
    fn state_lookup_id_sync(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        block_on(async { StateApi::state_lookup_id(self, addr, key).await })
    }
    ///
    fn state_changed_actors_sync(&self, old: &Cid, new: &Cid) -> Result<HashMap<String, Actor>> {
        block_on(async { StateApi::state_changed_actors(self, old, new).await })
    }
    ///
    fn state_get_receipt_sync(&self, cid: &Cid, key: &TipsetKey) -> Result<MessageReceipt> {
        block_on(async { StateApi::state_get_receipt(self, cid, key).await })
    }
    ///
    fn state_miner_sector_count_sync(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<MinerSectors> {
        block_on(async { StateApi::state_miner_sector_count(self, addr, key).await })
    }
    ///
    fn state_compute_sync(
        &self,
        height: u64,
        msgs: &[UnsignedMessage],
        key: &TipsetKey,
    ) -> Result<Cid> {
        block_on(async { StateApi::state_compute(self, height, msgs, key).await })
    }

    ///
    fn msig_get_available_balance_sync(&self, addr: &Address, key: &TipsetKey) -> Result<BigInt> {
        block_on(async { StateApi::msig_get_available_balance(self, addr, key).await })
    }

    /// This is on StateAPI because miner.Miner requires this, and MinerAPI requires miner.Miner
    fn miner_create_block_sync(
        &self,
        addr: &Address,
        parent_key: &TipsetKey,
        ticket: &Ticket,
        proof: &EPostProof,
        msgs: &[SignedMessage],
        height: u64,
        ts: u64,
    ) -> Result<BlockMsg> {
        block_on(async {
            StateApi::miner_create_block(self, addr, parent_key, ticket, proof, msgs, height, ts)
                .await
        })
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinerSectors {
    pub pset: u64,
    pub sset: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgWait {
    #[serde(with = "message_receipt_json")]
    pub receipt: MessageReceipt,
    #[serde(rename = "TipSet")]
    #[serde(with = "tipset_json")]
    pub tipset: Tipset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainSectorInfo {
    #[serde(rename = "SectorID")]
    pub sector_id: u64,
    #[serde(with = "base64")]
    pub comm_d: Vec<u8>,
    #[serde(with = "base64")]
    pub comm_r: Vec<u8>,
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActorState {
    pub balance: BigInt,
    // pub state: interface{},
}
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinerPower {
    #[serde(with = "bigint_json")]
    pub miner_power: BigInt,
    #[serde(with = "bigint_json")]
    pub total_power: BigInt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvocResult {
    #[serde(with = "unsigned_message_json")]
    pub msg: UnsignedMessage,
    #[serde(with = "message_receipt_json")]
    pub msg_rct: MessageReceipt,
    #[serde(with = "execution_result_json::vec")]
    pub internal_executions: Vec<ExecutionResult>,
    pub error: String,
}

// TODO: need to move to actor builtin-storagemarket
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OnChainDeal {
    #[serde(with = "base64")]
    piece_ref: Vec<u8>, // cid bytes, TODO: spec says to use cid.Cid, probably not a good idea
    piece_size: u64,

    #[serde(with = "address_json")]
    client: Address,
    #[serde(with = "address_json")]
    provider: Address,

    proposal_expiration: u64,
    duration: u64, // TODO: spec

    #[serde(with = "bigint_json")]
    storage_price_per_epoch: BigInt,
    #[serde(with = "bigint_json")]
    storage_collateral: BigInt,
    activation_epoch: u64, // 0 = inactive
}
