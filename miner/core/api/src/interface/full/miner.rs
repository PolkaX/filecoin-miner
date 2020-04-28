// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/*
use serde::{Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};
use plum_block::{beacon_entry_json, BeaconEntry, Ticket, ElectionProof, BlockMsg};
use plum_message::{message_receipt_json, unsigned_message_json, MessageReceipt, UnsignedMessage, SignedMessage};
use plum_tipset::{tipset_json, tipset_key_json, Tipset, TipsetKey};
use plum_types::Actor;
use plum_vm::{execution_result_json, ExecutionResult};
*/

use crate::client::RpcClient;
// use crate::errors::Result;
// use crate::helper;

///
#[async_trait::async_trait]
pub trait MinerApi: RpcClient {
    /*
    async fn miner_get_base_info(&self, addr: &Address, height: u64, key: &TipsetKey) -> Result<MinerBaseInfo> {

    }
    async fn miner_create_block(&self, template: &BlockTemplate) -> Result<BlockMsg> {

    }
    */
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningBaseInfo {
    #[serde(with = "bigint_json")]
    pub miner_power: BigInt,
    #[serde(with = "bigint_json")]
    pub network_power: BigInt,
    pub sectors: Vec<SectorInfo>,
    #[serde(with = "address_json")]
    pub worker_key: Address,
    pub sector_size: u64,
    #[serde(with = "beacon_entry_json")]
    pub prev_beacon_entry: BeaconEntry,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockTemplate {
    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(with = "tipset_key_json")]
    pub parents: TipsetKey,
    #[serde(with = "ticket_json")]
    pub ticket: Ticket,
    #[serde(with = "election_proof_json")]
    pub eproof: ElectionProof,
    #[serde(with = "beacon_entry_json::vec")]
    pub beacon_values: Vec<BeaconEntry>,
    #[serde(with = "signed_message_json")]
    pub message: SignedMessage,
    pub epoch: u64,
    pub timestamp: u64,
    #[serde(with = "post_proof_json::vec")]
    pub winning_post_proof: Vec<PostProof>,
}
*/
