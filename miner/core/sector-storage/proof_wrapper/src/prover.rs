// Copyright 2020 PolkaX

use crate::sealer::Sealer;
use anyhow::Result;
use filecoin_proofs_api::post::{generate_window_post, generate_winning_post};
use filecoin_proofs_api::{
    ChallengeSeed, PrivateReplicaInfo, ProverId, RegisteredPoStProof, SectorId, SnarkProof,
};
use std::collections::BTreeMap;

impl specs_storage::Prover for Sealer {
    fn generate_winning_post(
        randomness: &ChallengeSeed,
        replicas: &BTreeMap<SectorId, PrivateReplicaInfo>,
        prover_id: ProverId,
    ) -> Result<Vec<(RegisteredPoStProof, SnarkProof)>> {
        generate_winning_post(randomness, replicas, prover_id)
    }

    fn generate_window_post(
        randomness: &ChallengeSeed,
        replicas: &BTreeMap<SectorId, PrivateReplicaInfo>,
        prover_id: ProverId,
    ) -> Result<Vec<(RegisteredPoStProof, SnarkProof)>> {
        generate_window_post(randomness, replicas, prover_id)
    }
}
