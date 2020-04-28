// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};
use plum_message::{message_receipt_json, unsigned_message_json, MessageReceipt, UnsignedMessage};
use plum_tipset::{tipset_json, tipset_key_json, Tipset, TipsetKey};
use plum_types::Actor;
use plum_vm::{execution_result_json, ExecutionResult};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait StateApi: RpcClient {
    /// if tipset is nil, we'll use heaviest
    async fn state_call(&self, msg: &UnsignedMessage, key: &TipsetKey) -> Result<InvocResult> {
        self.request(
            "StateCall",
            vec![
                helper::serialize_with(unsigned_message_json::serialize, msg),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_replay(&self, key: &TipsetKey, cid: &Cid) -> Result<InvocResult> {
        self.request(
            "StateReplay",
            vec![
                helper::serialize_with(tipset_key_json::serialize, key),
                helper::serialize_with(cid_json::serialize, cid),
            ],
        )
        .await
    }
    ///
    async fn state_get_actor(&self, addr: &Address, key: &TipsetKey) -> Result<Actor> {
        let actor: helper::Actor = self
            .request(
                "StateGetActor",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(tipset_key_json::serialize, key),
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
        let cids: Vec<helper::Cid> = self
            .request(
                "StateListMessages",
                vec![
                    helper::serialize_with(unsigned_message_json::serialize, msg),
                    helper::serialize_with(tipset_key_json::serialize, key),
                    helper::serialize(&height),
                ],
            )
            .await?;
        Ok(cids.into_iter().map(|cid| cid.0).collect())
    }

    ///
    async fn state_network_name(&self) -> Result<String> {
        self.request("StateNetworkName", vec![]).await
    }
    /*
    ///
    async fn state_miner_sectors(
        &self,
        addr: &Address,
        field: &BitField,
        what: bool,
        key: &TipsetKey,
    ) -> Result<Vec<ChainSectorInfo>> {
        self.request(
            "StateMinerSectors",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(bitfield_json::serialize, field),
                helper::serialize(&what),
                helper::serialize_with(tipset_key_json::serialize, key),
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
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_power(&self, addr: &Address, key: &TipsetKey) -> Result<MinerPower> {
        self.request(
            "StateMinerPower",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_info(&self, addr: &Address, key: &TipsetKey) -> Result<MinerInfo> {
        self.request(
            "StateMinerInfo",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_deadlines(&self, addr: &Address, key: &TipsetKey) -> Result<miner::DeadlineInfo> {
        self.request(
            "StateMinerDeadlines",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
            .await
    }
    */
    ///
    async fn state_miner_faults(&self, addr: &Address, key: &TipsetKey) -> Result<Vec<u64>> {
        self.request(
            "StateMinerFaults",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_miner_initial_pledge_collateral(
        &self,
        addr: &Address,
        sector_number: u64,
        key: &TipsetKey,
    ) -> Result<BigInt> {
        let bigint: helper::BigInt = self
            .request(
                "StateMinerInitialPledgeCollateral",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize(&sector_number),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(bigint.0)
    }
    ///
    async fn state_miner_available_balance(
        &self,
        addr: &Address,
        key: &TipsetKey,
    ) -> Result<BigInt> {
        let bigint: helper::BigInt = self
            .request(
                "StateMinerAvailableBalance",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(bigint.0)
    }
    /*
    ///
    async fn state_sector_pre_commit_info(
        &self,
        addr: &Address,
        sector_number: u64,
        key: &TipsetKey,
    ) -> Result<miner::SectorPreCommitOnChainInfo> {
        let bigint: helper::BigInt = self
            .request(
                "StateSectorPreCommitInfo",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize(&sector_number),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(bigint.0)
    }
    */
    ///
    async fn state_pledge_collateral(&self, key: &TipsetKey) -> Result<BigInt> {
        let bigint: helper::BigInt = self
            .request(
                "StatePledgeCollateral",
                vec![helper::serialize_with(tipset_key_json::serialize, key)],
            )
            .await?;
        Ok(bigint.0)
    }
    ///
    async fn state_wait_msg(&self, cid: &Cid) -> Result<MsgLookup> {
        self.request(
            "StateWaitMsg",
            vec![helper::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }
    ///
    async fn state_search_msg(&self, cid: &Cid) -> Result<MsgLookup> {
        self.request(
            "StateSearchMsg",
            vec![helper::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }
    ///
    async fn state_list_miners(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request(
            "StateListMiners",
            vec![helper::serialize_with(tipset_key_json::serialize, key)],
        )
        .await
    }
    ///
    async fn state_list_actors(&self, key: &TipsetKey) -> Result<Vec<Address>> {
        self.request(
            "StateListActors",
            vec![helper::serialize_with(tipset_key_json::serialize, key)],
        )
        .await
    }

    ///
    async fn state_market_balance(&self, addr: &Address, key: &TipsetKey) -> Result<MarketBalance> {
        self.request(
            "StateMarketBalance",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    ///
    async fn state_market_participants(
        &self,
        key: &TipsetKey,
    ) -> Result<HashMap<String, MarketBalance>> {
        self.request(
            "StateMarketParticipants",
            vec![helper::serialize_with(tipset_key_json::serialize, key)],
        )
        .await
    }
    /*
    ///
    async fn state_market_deals(&self, key: &TipsetKey) -> Result<HashMap<String, MarketDeal>>;
    ///
    async fn state_market_storage_deal(
        &self,
        deal_id: u64,
        key: &TipsetKey,
    ) -> Result<MarketDeal> {
        self.request(
            "StateMarketStorageDeal",
            vec![
                helper::serialize(&deal_id),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }
    */
    ///
    async fn state_lookup_id(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        let address: helper::Address = self
            .request(
                "StateLookupID",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(address.0)
    }
    ///
    async fn state_account_key(&self, addr: &Address, key: &TipsetKey) -> Result<Address> {
        let address: helper::Address = self
            .request(
                "StateAccountKey",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(address.0)
    }
    ///
    async fn state_changed_actors(&self, old: &Cid, new: &Cid) -> Result<HashMap<String, Actor>> {
        let map: HashMap<String, helper::Actor> = self
            .request(
                "StateChangedActors",
                vec![
                    helper::serialize_with(cid_json::serialize, old),
                    helper::serialize_with(cid_json::serialize, new),
                ],
            )
            .await?;
        Ok(map.into_iter().map(|(k, v)| (k, v.0)).collect())
    }
    ///
    async fn state_get_receipt(&self, cid: &Cid, key: &TipsetKey) -> Result<MessageReceipt> {
        let msg_receipt: helper::MessageReceipt = self
            .request(
                "StateGetReceipt",
                vec![
                    helper::serialize_with(cid_json::serialize, cid),
                    helper::serialize_with(tipset_key_json::serialize, key),
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
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize_with(tipset_key_json::serialize, key),
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
    ) -> Result<ComputeStateOutput> {
        let msgs = msgs
            .iter()
            .cloned()
            .map(helper::UnsignedMessage)
            .collect::<Vec<_>>();
        self.request(
            "StateCompute",
            vec![
                helper::serialize(&height),
                helper::serialize(&msgs),
                helper::serialize_with(tipset_key_json::serialize, key),
            ],
        )
        .await
    }

    ///
    async fn msig_get_available_balance(&self, addr: &Address, key: &TipsetKey) -> Result<BigInt> {
        let bigint: helper::BigInt = self
            .request(
                "MsigGetAvailableBalance",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(tipset_key_json::serialize, key),
                ],
            )
            .await?;
        Ok(bigint.0)
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
pub struct MsgLookup {
    #[serde(with = "message_receipt_json")]
    pub receipt: MessageReceipt,
    #[serde(rename = "TipSet")]
    #[serde(with = "tipset_json")]
    pub tipset: Tipset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketBalance {
    #[serde(with = "bigint_json")]
    escrow: BigInt,
    #[serde(with = "bigint_json")]
    locked: BigInt,
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
    pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComputeStateOutput {
    #[serde(with = "cid_json")]
    pub root: Cid,
    pub trace: Vec<InvocResult>,
}
