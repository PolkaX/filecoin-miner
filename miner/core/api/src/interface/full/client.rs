// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use std::time::Duration;

use async_trait::async_trait;
use libp2p_core::PeerId;
use serde::{de, ser, Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
pub trait ClientApi: RpcClient {
    /// ClientImport imports file under the specified path into filestore
    async fn client_import(&self, path: &str) -> Result<Cid> {
        let cid: crate::helpers::Cid = self
            .request("ClientImport", vec![crate::helpers::serialize(&path)])
            .await?;
        Ok(cid.0)
    }

    ///
    async fn client_start_deal(
        &self,
        cid: &Cid,
        addr: &Address,
        miner: &Address,
        epoch_price: &BigInt,
        blocks_duration: u64,
    ) -> Result<Cid> {
        let cid: crate::helpers::Cid = self
            .request(
                "ClientStartDeal",
                vec![
                    crate::helpers::serialize_with(cid_json::serialize, cid),
                    crate::helpers::serialize_with(address_json::serialize, addr),
                    crate::helpers::serialize_with(address_json::serialize, miner),
                    crate::helpers::serialize_with(bigint_json::serialize, epoch_price),
                    crate::helpers::serialize(&blocks_duration),
                ],
            )
            .await?;
        Ok(cid.0)
    }

    ///
    async fn client_get_deal_info(&self, cid: &Cid) -> Result<DealInfo> {
        self.request(
            "ClientGetDealInfo",
            vec![crate::helpers::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }

    ///
    async fn client_list_deals(&self) -> Result<Vec<DealInfo>> {
        self.request("ClientListDeals", vec![]).await
    }

    ///
    async fn client_has_local(&self, root: &Cid) -> Result<bool> {
        self.request(
            "ClientHasLocal",
            vec![crate::helpers::serialize_with(cid_json::serialize, root)],
        )
        .await
    }

    ///
    async fn client_find_data(&self, root: &Cid) -> Result<Vec<QueryOffer>> {
        self.request(
            "ClientFindData",
            vec![crate::helpers::serialize_with(cid_json::serialize, root)],
        )
        .await
    }

    ///
    async fn client_retrieve(&self, order: &RetrievalOrder, path: &str) -> Result<()> {
        self.request(
            "ClientFindData",
            vec![
                crate::helpers::serialize(order),
                crate::helpers::serialize(&path),
            ],
        )
        .await
    }
    /*
    ///
    async fn client_query_ask(&self, peer_id: &PeerId, miner: &Address) -> Result<SignedStorageAsk>;

    ///
     async fn client_list_imports(&self) -> Result<Vec<Import>>;
    */
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Import {
    /*pub status: filestore.Status*/
    pub key: Cid,
    pub file_path: PathBuf,
    pub size: u64,
}
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealInfo {
    #[serde(with = "cid_json")]
    pub proposal_cid: Cid,
    pub state: DealStates,
    #[serde(with = "address_json")]
    pub provider: Address,

    pub piece_ref: Vec<u8>, // cid bytes
    pub size: u64,

    #[serde(with = "bigint_json")]
    pub price_per_epoch: BigInt,
    pub duration: Duration,

    #[serde(rename = "DealID")]
    pub deal_id: u64,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum DealStates {
    DealUnknown = 0,
    DealRejected,
    DealAccepted,
    DealStaged,
    DealSealing,
    DealFailed,
    DealComplete,
    DealError,
}

impl ser::Serialize for DealStates {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for DealStates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match u8::deserialize(deserializer)? {
            0 => DealStates::DealUnknown,
            1 => DealStates::DealRejected,
            2 => DealStates::DealAccepted,
            3 => DealStates::DealStaged,
            4 => DealStates::DealSealing,
            5 => DealStates::DealFailed,
            6 => DealStates::DealComplete,
            7 => DealStates::DealError,
            i => return Err(de::Error::custom(format!("unexpect integer {}", i))),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryOffer {
    pub err: String,

    #[serde(with = "cid_json")]
    pub root: Cid,

    pub size: u64,
    #[serde(with = "bigint_json")]
    pub min_price: BigInt,

    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(with = "crate::helpers::peer_id")]
    #[serde(rename = "MinerPeerID")]
    pub miner_peer_id: PeerId,
}

impl QueryOffer {
    pub fn order(&self, client: Address) -> RetrievalOrder {
        RetrievalOrder {
            root: self.root.clone(),
            size: self.size,
            total: self.min_price.clone(),

            client,
            miner: self.miner.clone(),
            miner_peer_id: self.miner_peer_id.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetrievalOrder {
    // TODO: make this less unixfs specific
    #[serde(with = "cid_json")]
    pub root: Cid,
    pub size: u64,
    // TODO: support offset
    #[serde(with = "bigint_json")]
    pub total: BigInt,

    #[serde(with = "address_json")]
    pub client: Address,
    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(with = "crate::helpers::peer_id")]
    #[serde(rename = "MinerPeerID")]
    pub miner_peer_id: PeerId,
}
