// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use plum_address::{address_json, Address};
use plum_bigint::BigInt;
use plum_crypto::{signature_json, Signature};
use plum_message::{unsigned_message_json, SignedMessage, UnsignedMessage};
use plum_wallet::{key_info_json, KeyInfo};

use crate::client::RpcClient;
use crate::errors::Result;
use crate::helper;

///
#[async_trait::async_trait]
pub trait WalletApi: RpcClient {
    ///
    async fn wallet_new(&self, sig_type: &str) -> Result<Address> {
        let address: helper::Address = self
            .request("WalletNew", vec![helper::serialize(&sig_type)])
            .await?;
        Ok(address.0)
    }
    ///
    async fn wallet_has(&self, addr: &Address) -> Result<bool> {
        self.request(
            "WalletHas",
            vec![helper::serialize_with(address_json::serialize, addr)],
        )
        .await
    }
    ///
    async fn wallet_list(&self) -> Result<Vec<Address>> {
        let addresses: Vec<helper::Address> = self.request("WalletList", vec![]).await?;
        Ok(addresses.into_iter().map(|address| address.0).collect())
    }
    ///
    async fn wallet_balance(&self, addr: &Address) -> Result<BigInt> {
        let bigint: helper::BigInt = self
            .request(
                "WalletBalance",
                vec![helper::serialize_with(address_json::serialize, addr)],
            )
            .await?;
        Ok(bigint.0)
    }
    ///
    async fn wallet_sign(&self, addr: &Address, msg: &[u8]) -> Result<Signature> {
        let signature: helper::Signature = self
            .request(
                "WalletSign",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize(&base64::encode(msg)),
                ],
            )
            .await?;
        Ok(signature.0)
    }
    ///
    async fn wallet_sign_message(
        &self,
        addr: &Address,
        msg: &UnsignedMessage,
    ) -> Result<SignedMessage> {
        let signed_msg: helper::SignedMessage = self
            .request(
                "WalletSignMessage",
                vec![
                    helper::serialize_with(address_json::serialize, addr),
                    helper::serialize_with(unsigned_message_json::serialize, msg),
                ],
            )
            .await?;
        Ok(signed_msg.0)
    }
    ///
    async fn wallet_verify(
        &self,
        addr: &Address,
        msg: &[u8],
        signature: &Signature,
    ) -> Result<bool> {
        self.request(
            "WalletVerify",
            vec![
                helper::serialize_with(address_json::serialize, addr),
                helper::serialize(&base64::encode(msg)),
                helper::serialize_with(signature_json::serialize, signature),
            ],
        )
        .await
    }
    ///
    async fn wallet_default_address(&self) -> Result<Address> {
        let address: helper::Address = self.request("WalletDefaultAddress", vec![]).await?;
        Ok(address.0)
    }
    ///
    async fn wallet_set_default(&self, addr: &Address) -> Result<()> {
        self.request(
            "WalletSetDefault",
            vec![helper::serialize_with(address_json::serialize, addr)],
        )
        .await
    }
    ///
    async fn wallet_export(&self, addr: &Address) -> Result<KeyInfo> {
        let key_info: helper::KeyInfo = self
            .request(
                "WalletExport",
                vec![helper::serialize_with(address_json::serialize, addr)],
            )
            .await?;
        Ok(key_info.0)
    }
    ///
    async fn wallet_import(&self, info: &KeyInfo) -> Result<Address> {
        let address: helper::Address = self
            .request(
                "WalletImport",
                vec![helper::serialize_with(key_info_json::serialize, info)],
            )
            .await?;
        Ok(address.0)
    }
}
