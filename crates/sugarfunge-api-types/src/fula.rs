use serde::{Deserialize, Serialize};
use crate::primitives::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub storage: Account,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestOutput {
    pub uploader: Account,
    pub storage: Option<Account>,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestInput {
    pub seed: Seed,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestInput {
    pub seed: Seed,
    pub cid: Cid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestOutput {
    pub uploader: Account,
    pub cid: Cid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsInput {
    pub account: Option<Account>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsOutput {
    pub manifests: Vec<Manifest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvailableManifestsOutput {
    pub manifests: Vec<ManifestAvailable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageManifestInput {
    pub seed: Seed,
    pub uploader: Account,
    pub cid: Cid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageManifestOutput {
    pub storage: Option<Account>,
    pub uploader: Account,
    pub cid: Cid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestData {
    pub uploader: Account,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub storage: Option<Account>,
    pub manifest_data: ManifestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestAvailable {
    pub manifest: serde_json::Value,
}

