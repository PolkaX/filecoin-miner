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
pub struct BigInt(#[serde(with = "plum_bigint::bigint_json")] pub plum_bigint::BigInt);
