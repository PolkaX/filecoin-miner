// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait PaychApi: RpcClient {
    ///
    async fn paych_get(
        &self,
        from: &Address,
        to: &Address,
        ensure_funds: &BigInt,
    ) -> Result<ChannelInfo> {
        self.request(
            "PaychGet",
            vec![
                helper::serialize_with(address_json::serialize, from),
                helper::serialize_with(address_json::serialize, to),
                helper::serialize_with(bigint_json::serialize, ensure_funds),
            ],
        )
        .await
    }
    ///
    async fn paych_list(&self) -> Result<Vec<Address>> {
        let addresses: Vec<helper::Address> = self.request("PaychList", vec![]).await?;
        Ok(addresses.into_iter().map(|address| address.0).collect())
    }
    ///
    async fn paych_status(&self, addr: &Address) -> Result<PaychStatus> {
        self.request(
            "PaychStatus",
            vec![helper::serialize_with(address_json::serialize, addr)],
        )
        .await
    }
    ///
    async fn paych_close(&self, addr: &Address) -> Result<Cid> {
        let cid: helper::Cid = self
            .request(
                "PaychClose",
                vec![helper::serialize_with(address_json::serialize, addr)],
            )
            .await?;
        Ok(cid.0)
    }
    ///
    async fn paych_allocate_lane(&self, addr: &Address) -> Result<u64> {
        self.request(
            "PaychAllocateLane",
            vec![helper::serialize_with(address_json::serialize, addr)],
        )
        .await
    }
    /*
    ///
    async fn paych_new_payment(
        &self,
        from: &Address,
        to: &Address,
        vouchers: &[VoucherSpec],
    ) -> Result<PaymentInfo>;
    ///
    async fn paych_voucher_check_valid(&self, addr: &Address, sign_vouch: SignedVoucher) -> Result<()>;
    ///
    async fn paych_voucher_check_spendable(
        &self,
        addr: &Address,
        sign_vouch: SignedVoucher,
        secret: &[u8],
        proof: &[u8],
    ) -> Result<bool>;
    ///
    async fn paych_voucher_create(&self, addr: &Address, amt: BigInt, lane: u64)
        -> Result<SignedVoucher>;
    ///
    async fn paych_voucher_add(
        &self,
        addr: &Address,
        signed_vouch: SignedVoucher,
        proof: &[u8],
        min_delta: BigInt,
    ) -> Result<BigInt>;
    ///
    async fn paych_voucher_list(&self, addr: &Address) -> Result<Vec<SignedVoucher>>;
    ///
    async fn paych_voucher_submit(&self, addr: &Address, signed_vouch: SignedVoucher) -> Result<Cid>;
    */
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelInfo {
    #[serde(with = "address_json")]
    pub channel: Address,
    #[serde(with = "cid_json")]
    pub channel_message: Cid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaychStatus {
    #[serde(with = "address_json")]
    pub control_addr: Address,
    pub direction: PchDir,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
pub enum PchDir {
    Undef = 0,
    Inbound = 1,
    Outbound = 2,
}

/*
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentInfo {
    pub channel: Address,
    pub channel_message: Cid,
    pub vouchers: Vec<SignedVoucher>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoucherSpec {
    pub amount: BigInt,
    pub time_lock: u64,
    pub min_close: u64,
    pub extra: ModVerifyParams,
}
*/
