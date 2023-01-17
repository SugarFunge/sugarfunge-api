use crate::account;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use serde_json::json;
use std::str::FromStr;
use subxt::ext::sp_core::sr25519::Public;
use subxt::ext::sp_runtime::AccountId32;
use subxt::storage::address::{StorageHasher, StorageMapKey};
use subxt::tx::PairSigner;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_fula::Manifest as ManifestRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_fula::ManifestStorageData as ManifestStorageDataRuntime;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;
// use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;

pub async fn update_manifest(
    data: web::Data<AppState>,
    req: web::Json<UpdateManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    let cid = BoundedVec(cid);

    let api = &data.api;

    let call = sugarfunge::tx().fula().update_manifest(
        cid,
        req.pool_id.into(),
        req.active_cycles,
        req.missed_cycles,
        req.active_days,
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestStorageUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ManifestUpdatedOutput {
            storer: event.storer.into(),
            pool_id: event.pool_id.into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            active_days: event.active_days,
            active_cycles: event.active_cycles,
            missed_cycles: event.missed_cycles,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UpdateManifests"),
            description: format!(""),
        })),
    }
}

pub async fn upload_manifest(
    data: web::Data<AppState>,
    req: web::Json<UploadManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cid: Vec<u8> = req.manifest_metadata["job"]["uri"]
        .to_string()
        .replace("\"", "")
        .into_bytes();
    let cid = BoundedVec(cid);

    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest_metadata).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().upload_manifest(
        manifest,
        cid,
        req.pool_id.into(),
        req.replication_factor.into(),
    );

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ManifestOutput {
            uploader: event.uploader.into(),
            storage: transform_vec_string_to_account(transform_storage_output(event.storage)),
            manifest_metadata: serde_json::from_slice(event.manifest.as_slice())
                .unwrap_or_default(),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UploadManifests"),
            description: format!(""),
        })),
    }
}

pub async fn storage_manifest(
    data: web::Data<AppState>,
    req: web::Json<StorageManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    let cid = BoundedVec(cid);

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .storage_manifest(cid, req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::StorageManifestOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {        
        Some(event) => Ok(HttpResponse::Ok().json(StorageManifestOutput {
            storer: event.storer.into(),
            cid: transform_vec_string_to_cid(transform_vec_u8_to_string(event.cid)),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::StorageManifest"),
            description: format!(""),
        })),
    }
}

pub fn transform_vec_u8_to_string(in_vec: Vec<u8>) -> Vec<String> {
    in_vec
        .into_iter()
        .map(|cid| cid.to_string())
        .collect()
}

pub fn transform_vec_string_to_cid(in_vec: Vec<String>) -> Vec<Cid> {
    in_vec
        .into_iter()
        .map(|cid| Cid::from(cid))
        .collect()
}

pub async fn remove_manifest(
    data: web::Data<AppState>,
    req: web::Json<RemoveManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
    let cid = BoundedVec(cid);
    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .remove_manifest(cid, req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestRemoved>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveManifestOutput {
            uploader: event.uploader.into(),
            cid: transform_vec_string_to_cid(transform_vec_u8_to_string(event.cid)),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveManifest"),
            description: format!(""),
        })),
    }
}

// pub async fn remove_storer(
//     data: web::Data<AppState>,
//     req: web::Json<RemoveStorerInput>,
// ) -> error::Result<HttpResponse> {
//     let pair = get_pair_from_seed(&req.seed)?;
//     let signer = PairSigner::new(pair);
//     let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
//     // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
//     let cid = BoundedVec(cid);
//     let storage = Public::from_str(&req.storage.as_str()).map_err(map_account_err)?;
//     let storage = AccountId32::from(storage);

//     let api = &data.api;

//     let call = sugarfunge::tx()
//         .fula()
//         .remove_storer(storage, cid, req.pool_id.into());

//     let result = api
//         .tx()
//         .sign_and_submit_then_watch(&call, &signer, Default::default())
//         .await
//         .map_err(map_subxt_err)?
//         .wait_for_finalized_success()
//         .await
//         .map_err(map_fula_err)?;
//     let result = result
//         .find_first::<sugarfunge::fula::events::RemoveStorerOutput>()
//         .map_err(map_subxt_err)?;
//     if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
//         return Err(value_error);
//     }
//     match result {
//         Some(event) => Ok(HttpResponse::Ok().json(RemoveStorerOutput {
//             uploader: event.uploader.into(),
//             storage: transform_option_account_value(event.storage),
//             cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
//             pool_id: event.pool_id.into(),
//         })),
//         None => Ok(HttpResponse::BadRequest().json(RequestError {
//             message: json!("Failed to find sugarfunge::fula::events::RemoveStorer"),
//             description: format!(""),
//         })),
//     }
// }

pub async fn remove_stored_manifest(
    data: web::Data<AppState>,
    req: web::Json<RemoveStoringManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
    let cid = BoundedVec(cid);
    let uploader = Public::from_str(&req.uploader.as_str()).map_err(map_account_err)?;
    let uploader = AccountId32::from(uploader);

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .remove_stored_manifest(cid, req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::RemoveStorerOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveStoringManifestOutput {
            storage: transform_option_account_value(event.storage),
            cid: transform_vec_string_to_cid(transform_vec_u8_to_string(event.cid)),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveStorer"),
            description: format!(""),
        })),
    }
}

pub async fn get_all_manifests(
    data: web::Data<AppState>,
    req: web::Json<GetAllManifestsInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage().fula().manifests_root().to_bytes();
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        StorageMapKey::new(&key_value, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    }
    // println!("query_key account_to len: {}", query_key.len());

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();

        // let account_from_idx = 96;
        // let account_from_key = key.0.as_slice()[account_from_idx..(account_from_idx + 32)].to_vec();
        // let account_from_id = AccountId32::decode(&mut &account_from_key[..]);
        // let account_from_id = Account::from(account_from_id.unwrap());
        // println!("account_from_id: {:?}", account_from_id);

        // let manifest_idx = 144;
        // let manifest_key = key.0.as_slice()[manifest_idx..].to_vec();
        // let manifest_id =
        //     ManifestRuntime::<AccountId32,Vec<u8>>::decode(&mut &manifest_key[..]);
        // let manifest_id =manifest_id.unwrap();
        // let manifest_value = Manifest{
        //     from: Account::from(manifest_id.from),
        //     to: Account::from(manifest_id.to),
        //     manifest:serde_json::from_slice(manifest_id.manifest.as_slice()).unwrap_or_default(),
        // };
        // println!("manifest: {:?}", manifest_value);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = ManifestRuntime::<AccountId32, Vec<u8>>::decode(&mut &storage_data[..]);
            let value = value.unwrap();
            let manifest_data = ManifestData {
                uploader: Account::from(value.manifest_data.uploader),
                manifest_metadata: serde_json::from_slice(
                    value.manifest_data.manifest_metadata.as_slice(),
                )
                .unwrap_or_default(),
            };
            let storage = value.storage.to_owned();

            let mut storage_vec: Vec<Account> = Vec::new();

            for storer in storage {
                let current_account = Account::try_from(storer).unwrap();
                storage_vec.push(current_account);
            }

            let storers_count: u16 = storage_vec.len() as u16;
            let replication_available = value.replication_factor - storers_count;

            if let Some(uploader_filter) = req.uploader.clone() {
                if AccountId32::from(
                    Public::from_str(&manifest_data.uploader.as_str()).map_err(map_account_err)?,
                ) != AccountId32::from(
                    Public::from_str(&uploader_filter.as_str()).map_err(map_account_err)?,
                ) {
                    meet_requirements = false;
                }
            }

            if let Some(storage_filter) = req.storage.clone() {
                if !value.storage.to_owned().contains(&AccountId32::from(
                    Public::from_str(&storage_filter.as_str()).map_err(map_account_err)?,
                )) {
                    meet_requirements = false;
                }
            }

            if meet_requirements {
                result_array.push(Manifest {
                    storage: storage_vec,
                    manifest_data,
                    replication_available: replication_available.into(),
                    pool_id: pool_id.into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllManifestsOutput {
        manifests: result_array,
    }))
}

pub async fn get_available_manifests(
    data: web::Data<AppState>,
    req: web::Json<GetAvailableManifestsInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage().fula().manifests_root().to_bytes();
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        StorageMapKey::new(&key_value, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    }

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));
        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = ManifestRuntime::<AccountId32, Vec<u8>>::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            let storers_count: u16 = value.storage.len() as u16;
            let replication_available = value.replication_factor - storers_count;

            if replication_available > 0 {
                let manifest_metadata =
                    serde_json::from_slice(value.manifest_data.manifest_metadata.as_slice())
                        .unwrap_or_default();
                let uploader = value.manifest_data.uploader.into();
                result_array.push(ManifestAvailable {
                    manifest_data: ManifestData {
                        uploader,
                        manifest_metadata,
                    },
                    replication_available: replication_available.into(),
                    pool_id: pool_id.into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAvailableManifestsOutput {
        manifests: result_array,
    }))
}

pub async fn get_all_manifests_storer_data(
    data: web::Data<AppState>,
    req: web::Json<GetAllManifestsStorerDataInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage()
        .fula()
        .manifests_storer_data_root()
        .to_bytes();
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        StorageMapKey::new(&key_value, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
        // println!("query_key pool_id len: {}", query_key.len());
    }

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.0.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        let account_idx = 68;
        let account_key = key.0.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        let cid_idx = 116;
        let cid_key = key.0.as_slice()[cid_idx..].to_vec();
        let cid_id = String::decode(&mut &cid_key[..]);
        let cid_id = cid_id.unwrap();
        // println!("cid_id: {:?}", cid_id);

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = ManifestStorageDataRuntime::decode(&mut &storage_data[..]);
            let manifest_value = value.unwrap();

            if let Some(uploader_filter) = req.storer.clone() {
                if AccountId32::from(
                    Public::from_str(&account_id.as_str()).map_err(map_account_err)?,
                ) != AccountId32::from(
                    Public::from_str(&uploader_filter.as_str()).map_err(map_account_err)?,
                ) {
                    meet_requirements = false;
                }
            }

            if meet_requirements {
                result_array.push(ManifestStorageData {
                    active_cycles: manifest_value.active_cycles,
                    missed_cycles: manifest_value.missed_cycles,
                    active_days: manifest_value.active_days,
                    pool_id: pool_id.into(),
                    account: account_id,
                    cid: cid_id.into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllManifestsStorerDataOutput {
        manifests: result_array,
    }))
}
