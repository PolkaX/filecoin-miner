// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

use serde::{Deserialize, Serialize};
use serde_json::{
    value::Serializer as JsonValueSerializer, Error as JsonError, Value as JsonValue,
};

#[inline]
pub(crate) fn serialize<T: Serialize>(value: &T) -> JsonValue {
    value
        .serialize(JsonValueSerializer)
        .expect("Types never fail to serialize")
}

#[inline]
pub(crate) fn serialize_with<F, T>(f: F, value: &T) -> JsonValue
where
    F: Fn(&T, JsonValueSerializer) -> Result<JsonValue, JsonError>,
{
    f(value, JsonValueSerializer).expect("Types never fail to serialize")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerId(#[serde(with = "peer_id")] pub libp2p_core::PeerId);

/// PeerId JSON serialization/deserialization
pub mod peer_id {
    use std::str::FromStr;

    use libp2p_core::PeerId;
    use serde::{de, ser, Deserialize, Serialize};

    /// JSON serialization
    pub fn serialize<S>(peer_id: &PeerId, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        peer_id.to_string().serialize(serializer)
    }

    /// JSON deserialization
    pub fn deserialize<'de, D>(deserializer: D) -> Result<PeerId, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let base58 = String::deserialize(deserializer)?;
        let peer_id = libp2p_core::PeerId::from_str(&base58)
            .map_err(|err| de::Error::custom(err.to_string()))?;
        Ok(peer_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cid(#[serde(with = "cid::ipld_dag_json")] pub cid::Cid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address(#[serde(with = "plum_address::address_json")] pub plum_address::Address);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigInt(#[serde(with = "plum_bigint::bigint_json")] pub plum_bigint::BigInt);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader(
    #[serde(with = "plum_block::block_header_json")] pub plum_block::BlockHeader,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMsg(#[serde(with = "plum_block::block_msg_json")] pub plum_block::BlockMsg);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReceipt(
    #[serde(with = "plum_message::message_receipt_json")] pub plum_message::MessageReceipt,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMessage(
    #[serde(with = "plum_message::signed_message_json")] pub plum_message::SignedMessage,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsignedMessage(
    #[serde(with = "plum_message::unsigned_message_json")] pub plum_message::UnsignedMessage,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EPostProof(#[serde(with = "plum_ticket::epost_proof_json")] pub plum_ticket::EPostProof);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket(#[serde(with = "plum_ticket::ticket_json")] pub plum_ticket::Ticket);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipsetKey(#[serde(with = "plum_tipset::tipset_key_json")] pub plum_tipset::TipsetKey);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tipset(#[serde(with = "plum_tipset::tipset_json")] pub plum_tipset::Tipset);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(#[serde(with = "plum_crypto::signature_json")] pub plum_crypto::Signature);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo(#[serde(with = "plum_wallet::key_info_json")] pub plum_wallet::KeyInfo);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor(#[serde(with = "plum_types::actor::json")] pub plum_types::actor::Actor);
