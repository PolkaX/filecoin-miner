// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use async_std::task::block_on;
use async_trait::async_trait;
use serde::{de, ser, Deserialize, Serialize};

use cid::{ipld_dag_json as cid_json, Cid};
use plum_address::{address_json, Address};
use plum_bigint::{bigint_json, BigInt};

use crate::client::RpcClient;
use crate::errors::Result;

///
#[async_trait]
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
                crate::helpers::serialize_with(address_json::serialize, from),
                crate::helpers::serialize_with(address_json::serialize, to),
                crate::helpers::serialize_with(bigint_json::serialize, ensure_funds),
            ],
        )
        .await
    }
    ///
    async fn paych_list(&self) -> Result<Vec<Address>> {
        let addresses: Vec<crate::helpers::Address> = self.request("PaychList", vec![]).await?;
        Ok(addresses.into_iter().map(|address| address.0).collect())
    }
    ///
    async fn paych_status(&self, addr: &Address) -> Result<PaychStatus> {
        self.request(
            "PaychStatus",
            vec![crate::helpers::serialize_with(
                address_json::serialize,
                addr,
            )],
        )
        .await
    }
    ///
    async fn paych_close(&self, addr: &Address) -> Result<Cid> {
        let cid: crate::helpers::Cid = self
            .request(
                "PaychClose",
                vec![crate::helpers::serialize_with(
                    address_json::serialize,
                    addr,
                )],
            )
            .await?;
        Ok(cid.0)
    }
    ///
    async fn paych_allocate_lane(&self, addr: &Address) -> Result<u64> {
        self.request(
            "PaychAllocateLane",
            vec![crate::helpers::serialize_with(
                address_json::serialize,
                addr,
            )],
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

pub trait SyncPaychApi: PaychApi {
    ///
    fn paych_get_sync(
        &self,
        from: &Address,
        to: &Address,
        ensure_funds: &BigInt,
    ) -> Result<ChannelInfo> {
        block_on(async { PaychApi::paych_get(self, from, to, ensure_funds).await })
    }
    ///
    fn paych_list_sync(&self) -> Result<Vec<Address>> {
        block_on(async { PaychApi::paych_list(self).await })
    }
    ///
    fn paych_status_sync(&self, addr: &Address) -> Result<PaychStatus> {
        block_on(async { PaychApi::paych_status(self, addr).await })
    }
    ///
    fn paych_close_sync(&self, addr: &Address) -> Result<Cid> {
        block_on(async { PaychApi::paych_close(self, addr).await })
    }
    ///
    fn paych_allocate_lane_sync(&self, addr: &Address) -> Result<u64> {
        block_on(async { PaychApi::paych_allocate_lane(self, addr).await })
    }
    /*
    ///
    fn paych_new_payment_sync(
        &self,
        from: &Address,
        to: &Address,
        vouchers: &[VoucherSpec],
    ) -> Result<PaymentInfo>;
    ///
    fn paych_voucher_check_valid_sync(&self, addr: &Address, sign_vouch: SignedVoucher) -> Result<()>;
    ///
    fn paych_voucher_check_spendable_sync(
        &self,
        addr: &Address,
        sign_vouch: SignedVoucher,
        secret: &[u8],
        proof: &[u8],
    ) -> Result<bool>;
    ///
    fn paych_voucher_create_sync(&self, addr: &Address, amt: BigInt, lane: u64)
        -> Result<SignedVoucher>;
    ///
    fn paych_voucher_add_sync(
        &self,
        addr: &Address,
        signed_vouch: SignedVoucher,
        proof: &[u8],
        min_delta: BigInt,
    ) -> Result<BigInt>;
    ///
    fn paych_voucher_list_sync(&self, addr: &Address) -> Result<Vec<SignedVoucher>>;
    ///
    fn paych_voucher_submit_sync(&self, addr: &Address, signed_vouch: SignedVoucher) -> Result<Cid>;
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
#[derive(Copy, Clone, Debug)]
pub enum PchDir {
    Undef = 0,
    Inbound = 1,
    Outbound = 2,
}

impl ser::Serialize for PchDir {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for PchDir {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(match u8::deserialize(deserializer)? {
            0 => PchDir::Undef,
            1 => PchDir::Inbound,
            2 => PchDir::Outbound,
            i => return Err(de::Error::custom(format!("unexpect integer {}", i))),
        })
    }
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
