// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use libp2p_core::PeerId;
use serde::{Deserialize, Serialize};
// use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait ClientApi: RpcClient {
    /// ClientImport imports file under the specified path into filestore
    async fn client_import(&self, r#ref: &FileRef) -> Result<Cid> {
        let cid: helper::Cid = self
            .request("ClientImport", vec![helper::serialize(r#ref)])
            .await?;
        Ok(cid.0)
    }

    /*
    ///
    async fn client_start_deal(&self, params: &StartDealParams) -> Result<Cid> {
        let cid: helper::Cid = self
            .request("ClientStartDeal", vec![helper::serialize(params)])
            .await?;
        Ok(cid.0)
    }

    ///
    async fn client_get_deal_info(&self, cid: &Cid) -> Result<DealInfo> {
        self.request(
            "ClientGetDealInfo",
            vec![helper::serialize_with(cid_json::serialize, cid)],
        )
        .await
    }

    ///
    async fn client_list_deals(&self) -> Result<Vec<DealInfo>> {
        self.request("ClientListDeals", vec![]).await
    }
    */

    ///
    async fn client_has_local(&self, root: &Cid) -> Result<bool> {
        self.request(
            "ClientHasLocal",
            vec![helper::serialize_with(cid_json::serialize, root)],
        )
        .await
    }

    ///
    async fn client_find_data(&self, root: &Cid) -> Result<Vec<QueryOffer>> {
        self.request(
            "ClientFindData",
            vec![helper::serialize_with(cid_json::serialize, root)],
        )
        .await
    }

    ///
    async fn client_retrieve(&self, order: &RetrievalOrder, r#ref: &FileRef) -> Result<()> {
        self.request(
            "ClientFindData",
            vec![helper::serialize(order), helper::serialize(r#ref)],
        )
        .await
    }
    /*
    ///
    async fn client_query_ask(&self, peer_id: &PeerId, miner: &Address) -> Result<SignedStorageAsk>;
    */
    ///
    async fn client_calc_comm_p(&self, inpath: &str, miner: &Address) -> Result<CommPRet> {
        self.request(
            "ClientCalcCommP",
            vec![
                helper::serialize(&inpath),
                helper::serialize_with(address_json::serialize, miner),
            ],
        )
        .await
    }
    ///
    async fn client_gen_car(&self, r#ref: &FileRef, outpath: &str) -> Result<CommPRet> {
        self.request(
            "ClientCalcCommP",
            vec![helper::serialize(r#ref), helper::serialize(&outpath)],
        )
        .await
    }
    /*
    ///
     async fn client_list_imports(&self) -> Result<Vec<Import>>;
    */
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileRef {
    pub path: String,
    #[serde(rename = "IsCAR")]
    pub is_car: bool,
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartDealParams {
    // data: storagemarket::DataRef,
    #[serde(with = "address_json")]
    pub wallet: Address,
    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(with = "bigint_json")]
    pub epoch_price: BigInt,
    pub min_blocks_duration: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Import {
    /*pub status: filestore.Status*/
    pub key: Cid,
    pub file_path: PathBuf,
    pub size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DealInfo {
    #[serde(with = "cid_json")]
    pub proposal_cid: Cid,
    // pub state: DealStates,
    pub message: String,
    #[serde(with = "address_json")]
    pub provider: Address,

    #[serde(rename = "PieceCID")]
    #[serde(with = "cid_json")]
    pub piece_cid: Cid,
    pub size: u64,

    #[serde(with = "bigint_json")]
    pub price_per_epoch: BigInt,
    pub duration: Duration,

    #[serde(rename = "DealID")]
    pub deal_id: u64,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
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
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryOffer {
    pub err: String,

    #[serde(with = "cid_json")]
    pub root: Cid,

    pub size: u64,
    #[serde(with = "bigint_json")]
    pub min_price: BigInt,
    pub payment_interval: u64,
    pub payment_interval_increase: u64,
    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(rename = "MinerPeerID")]
    #[serde(with = "crate::helper::peer_id")]
    pub miner_peer_id: PeerId,
}

impl QueryOffer {
    pub fn order(&self, client: Address) -> RetrievalOrder {
        RetrievalOrder {
            root: self.root.clone(),
            size: self.size,
            total: self.min_price.clone(),
            payment_interval: self.payment_interval,
            payment_interval_increase: self.payment_interval_increase,
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

    pub payment_interval: u64,
    pub payment_interval_increase: u64,

    #[serde(with = "address_json")]
    pub client: Address,
    #[serde(with = "address_json")]
    pub miner: Address,
    #[serde(rename = "MinerPeerID")]
    #[serde(with = "crate::helper::peer_id")]
    pub miner_peer_id: PeerId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommPRet {
    #[serde(with = "cid_json")]
    pub root: Cid,
    pub size: u64,
}
