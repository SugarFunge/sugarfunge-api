use serde::{Deserialize, Serialize};
use crate::primitives::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub storage: Account,
    pub manifest_metadata: serde_json::Value,
    pub replication_factor: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestOutput {
    pub uploader: Account,
    pub storage: Vec<Account>,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestInput {
    pub seed: Seed,
    pub manifest_metadata: serde_json::Value,
    pub replication_factor: u8,
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
pub struct RemoveFromManifestInput {
    pub seed: Seed,
    pub storage: Account,
    pub cid: Cid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveFromManifestOutput {
    pub uploader: Account,
    pub cid: Cid,
    pub storage: Option<Account>,
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
    pub storage: Account,
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
    pub storage: Vec<Account>,
    pub manifest_data: ManifestData,
    pub replication_available: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestAvailable {
    pub manifest_data: ManifestData,
    pub replication_available: u8
}

