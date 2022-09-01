use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub to: Account,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestOutput {
    pub from: Account,
    pub to: Account,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestsInput {
    pub account: Account,
    pub operator: Option<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub from: Account,
    pub to: Account,
    pub manifest: serde_json::Value,
    pub value: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestsOutput {
    pub manifests: Vec<Manifest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetManifestsInput {
    pub from: Account,
    pub to: Account,
}