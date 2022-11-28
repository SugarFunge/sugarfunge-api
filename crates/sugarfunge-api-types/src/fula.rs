use crate::primitives::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateManifestInput {
    pub seed: Seed,
    pub storage: Account,
    pub manifest_metadata: serde_json::Value,
    pub pool_id: PoolId,
    pub replication_factor: ReplicationFactor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestOutput {
    pub uploader: Account,
    pub storage: Vec<Account>,
    pub manifest_metadata: serde_json::Value,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadManifestInput {
    pub seed: Seed,
    pub manifest_metadata: serde_json::Value,
    pub pool_id: PoolId,
    pub replication_factor: ReplicationFactor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestInput {
    pub seed: Seed,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveManifestOutput {
    pub uploader: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStorerInput {
    pub seed: Seed,
    pub storage: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStorerOutput {
    pub uploader: Account,
    pub storage: Option<Account>,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStoringManifestInput {
    pub seed: Seed,
    pub uploader: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveStoringManifestOutput {
    pub uploader: Account,
    pub storage: Option<Account>,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsInput {
    pub pool_id: Option<PoolId>,
    pub uploader: Option<Account>,
    pub storage: Option<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllManifestsOutput {
    pub manifests: Vec<Manifest>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAvailableManifestsInput {
    pub pool_id: Option<PoolId>,
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
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageManifestOutput {
    pub storage: Account,
    pub uploader: Account,
    pub cid: Cid,
    pub pool_id: PoolId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestData {
    pub uploader: Account,
    pub manifest_metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub pool_id: PoolId,
    pub storage: Vec<Account>,
    pub manifest_data: ManifestData,
    pub replication_available: ReplicationFactor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManifestAvailable {
    pub pool_id: PoolId,
    pub manifest_data: ManifestData,
    pub replication_available: ReplicationFactor,
}
