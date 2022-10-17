use serde::{Deserialize, Serialize};
use crate::primitives::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub to: Account,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestOutput {
    pub from: Account,
    pub to: Option<Account>,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestInput {
    pub seed: Seed,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestOutput {
    pub from: Account,
    pub to: Option<Account>,
    pub manifest: serde_json::Value,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestInput {
    pub seed: Seed,
    pub cid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestOutput {
    pub from: Account,
    pub cid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsInput {
    pub account: Option<Account>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsOutput {
    pub manifests: Vec<ManifestStorage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvailableManifestsOutput {
    pub manifests: Vec<ManifestAvailable>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub from: Account,
    pub manifest: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestStorage {
    pub to: Option<Account>,
    pub manifest: Manifest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestAvailable {
    pub manifest: serde_json::Value,
}