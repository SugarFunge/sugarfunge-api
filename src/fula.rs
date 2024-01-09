use crate::account;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use codec::Decode;
use serde_json::json;
use std::str::FromStr;
use sp_core::sr25519::Public;
use subxt::tx::PairSigner;
use subxt::utils::AccountId32;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::bounded_collections::bounded_vec::BoundedVec;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_fula::{
    Manifest as ManifestRuntime, ManifestAvailable as ManifestAvailableRuntime,
    ManifestStorageData as ManifestStorageDataRuntime,
    ManifestWithPoolId as ManifestWithPoolIdRuntime, StorerData as StorerDataRuntime,
    UploaderData as UploaderDataRuntime,
};
use futures::stream::StreamExt;

pub async fn upload_manifest(
    data: web::Data<AppState>,
    req: web::Json<UploadManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
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
        Some(event) => Ok(HttpResponse::Ok().json(UploadManifestOutput {
            uploader: event.uploader.into(),
            storers: transform_vec_string_to_account(transform_storage_output(event.storer)),
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

pub async fn batch_upload_manifest(
    data: web::Data<AppState>,
    req: web::Json<BatchUploadManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let pool_ids = get_vec_pool_id_from_input(req.pool_id.to_vec());
    let cids = get_vec_cids_from_input(req.cid.to_vec());
    let manifests = get_vec_manifests_from_input(req.manifest_metadata.to_vec());
    let replication_factors =
        get_vec_replication_factor_from_input(req.replication_factor.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx().fula().batch_upload_manifest(
        manifests,
        cids,
        pool_ids,
        replication_factors,
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
        .find_first::<sugarfunge::fula::events::BatchManifestOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BatchUploadManifestOutput {
            uploader: event.uploader.into(),
            manifest_metadata: get_vec_manifests_from_node(event.manifests),
            pool_id: get_vec_pool_id_from_node(event.pool_ids),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::BatchUploadManifests"),
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
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::StorageManifest"),
            description: format!(""),
        })),
    }
}

pub async fn batch_storage_manifest(
    data: web::Data<AppState>,
    req: web::Json<BatchStorageManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cid.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .batch_storage_manifest(cids, req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::BatchStorageManifestOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BatchStorageManifestOutput {
            storer: event.storer.into(),
            pool_id: event.pool_id.into(),
            cid: get_vec_cids_from_node(event.cids),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::BatchStorageManifestOutput"),
            description: format!(""),
        })),
    }
}

pub async fn remove_manifest(
    data: web::Data<AppState>,
    req: web::Json<RemoveManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
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
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveManifest"),
            description: format!(""),
        })),
    }
}

pub async fn batch_remove_manifest(
    data: web::Data<AppState>,
    req: web::Json<BatchRemoveManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cid.to_vec());
    let pool_ids = get_vec_pool_id_from_input(req.pool_id.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .batch_remove_manifest(cids, pool_ids);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::BatchManifestRemoved>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BatchRemoveManifestOutput {
            uploader: event.uploader.into(),
            cid: get_vec_cids_from_node(event.cids),
            pool_id: get_vec_pool_id_from_node(event.pool_ids),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveManifest"),
            description: format!(""),
        })),
    }
}

pub async fn remove_stored_manifest(
    data: web::Data<AppState>,
    req: web::Json<RemoveStoringManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
    let cid = BoundedVec(cid);

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
            storer: transform_option_account_value(event.storer),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            pool_id: event.pool_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveStorer"),
            description: format!(""),
        })),
    }
}

pub async fn batch_remove_stored_manifest(
    data: web::Data<AppState>,
    req: web::Json<BatchRemoveStoringManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cid.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .batch_remove_stored_manifest(cids, req.pool_id.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::BatchRemoveStorerOutput>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BatchRemoveStoringManifestOutput {
            storer: event.storer.into(),
            pool_id: event.pool_id.into(),
            cid: get_vec_cids_from_node(event.cids),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveStorer"),
            description: format!(""),
        })),
    }
}

pub async fn verify_manifest(
    data: web::Data<AppState>,
    req: web::Json<VerifyManifestsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().fula().verify_manifests();

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::VerifiedStorerManifests>()
        .map_err(map_subxt_err)?;
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(VerifyManifestsOutput {
            storer: event.storer.into(),
            valid_manifests: get_vec_cids_from_node(event.valid_cids),
            invalid_manifests: get_vec_cids_from_node(event.invalid_cids),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UploadManifests"),
            description: format!(""),
        })),
    }
}
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
    if let Err(value_error) = account::refund_fees(data, &req.seed.clone()).await {
        return Err(value_error);
    }
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(UpdatedManifestOutput {
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

pub async fn get_all_manifests(
    data: web::Data<AppState>,
    req: web::Json<GetAllManifestsInput>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key:Vec<u8>; 
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        query_key = sugarfunge::storage()
        .fula()
        .manifests_iter1(key_value)
        .to_root_bytes();
    } else {
        query_key = sugarfunge::storage()
        .fula()
        .manifests_iter()
        .to_root_bytes();
    }
    // println!("query_key account_to len: {}", query_key.len());

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ManifestRuntime::<AccountId32, Vec<u8>>::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            let uploaders_data =
                transform_vec_uploader_data_runtime_to_vec_uploader_data(value.users_data);

            if let Some(storer) = req.storer.clone() {
                if let Ok(contained_value) =
                    verify_contains_storer(uploaders_data.to_owned(), storer.clone())
                {
                    if !contained_value {
                        meet_requirements = false;
                    }
                }
            }

            if let Some(uploader) = req.uploader.clone() {
                if let Ok(contained_value) =
                    verify_contains_uploader(uploaders_data.to_owned(), uploader.clone())
                {
                    if !contained_value {
                        meet_requirements = false;
                    }
                }
            }

            if meet_requirements {
                result_array.push(Manifest {
                    pool_id: pool_id.into(),
                    uploaders: uploaders_data.to_owned(),
                    manifest_metadata: serde_json::from_slice(value.manifest_metadata.as_slice())
                        .unwrap_or_default(),
                    size: value.size,
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

    let query_key:Vec<u8>;
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        query_key = sugarfunge::storage()
        .fula()
        .manifests_iter1(key_value)
        .to_root_bytes();
    } else {
        query_key = sugarfunge::storage()
        .fula()
        .manifests_iter()
        .to_root_bytes();
    }

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));
        let pool_id_idx = 48;
        let pool_id_key = key.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ManifestRuntime::<AccountId32, Vec<u8>>::decode(&mut &storage_data[..]);
            let value = value.unwrap();

            let uploaders_data =
                transform_vec_uploader_data_runtime_to_vec_uploader_data(value.users_data);
            if verify_availability(uploaders_data.to_vec()) {
                result_array.push(ManifestAvailable {
                    pool_id: pool_id.into(),
                    manifest_metadata: serde_json::from_slice(value.manifest_metadata.as_slice())
                        .unwrap_or_default(),
                    replication_available: get_added_replication(uploaders_data.to_owned()),
                })
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

    let query_key:Vec<u8>;
    // println!("query_key manifests_root len: {}", query_key.len());

    if let Some(value) = req.pool_id.clone() {
        let key_value: u32 = value.into();
        query_key = sugarfunge::storage()
        .fula()
        .manifests_storer_data_iter1(key_value)
        .to_root_bytes();
        // println!("query_key pool_id len: {}", query_key.len());
    } else {
        query_key = sugarfunge::storage()
        .fula()
        .manifests_storer_data_iter()
        .to_root_bytes();
    }

    let storage = api.storage().at_latest().await.map_err(map_subxt_err)?;

    let keys_stream  = storage
        .fetch_raw_keys(query_key)
        .await
        .map_err(map_subxt_err)?;

    let keys: Vec<Vec<u8>> = keys_stream
        .collect::<Vec<_>>()  // Collect into a Vec<Result<Vec<u8>, Error>>
        .await                // Await the collection process
        .into_iter()          // Convert into an iterator
        .filter_map(Result::ok) // Filter out Ok values, ignore errors
        .collect();           // Collect into a Vec<Vec<u8>>

    // println!("Obtained keys:");
    for key in keys.iter() {
        let mut meet_requirements = true;
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        let pool_id_idx = 48;
        let pool_id_key = key.as_slice()[pool_id_idx..(pool_id_idx + 4)].to_vec();
        let pool_id_id = u32::decode(&mut &pool_id_key[..]);
        let pool_id = pool_id_id.unwrap();
        // println!("pool_id: {:?}", pool_id);

        let account_idx = 68;
        let account_key = key.as_slice()[account_idx..(account_idx + 32)].to_vec();
        let account_id = AccountId32::decode(&mut &account_key[..]);
        let account_id = Account::from(account_id.unwrap());
        // println!("account_id: {:?}", account_id);

        let cid_idx = 116;
        let cid_key = key.as_slice()[cid_idx..].to_vec();
        let cid_id = String::decode(&mut &cid_key[..]);
        let cid_id = cid_id.unwrap();
        // println!("cid_id: {:?}", cid_id);

        if let Some(storage_data) = storage.fetch_raw(key.clone()).await.map_err(map_subxt_err)? {
            let value = ManifestStorageDataRuntime::decode(&mut &storage_data[..]);
            let manifest_value = value.unwrap();

            if let Some(uploader_filter) = req.storer.clone() {
                // Parse the string into a public key
                let uploader_public_key = Public::from_str(&account_id.as_str()).map_err(map_account_err)?;
                let uploader_filter_public_key = Public::from_str(&uploader_filter.as_str()).map_err(map_account_err)?;
                
                // Convert the public keys into a byte array
                let uploader_public_key_bytes: [u8; 32] = uploader_public_key.0;
                let uploader_filter_public_key_bytes: [u8; 32] = uploader_filter_public_key.0;
                
                // Create AccountId32 from the byte arrays
                let uploader_account_id = AccountId32::from(uploader_public_key_bytes);
                let uploader_filter_account_id = AccountId32::from(uploader_filter_public_key_bytes);
                
                // Compare the account IDs
                if uploader_account_id != uploader_filter_account_id {
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
                    state: manifest_value.challenge_state.into(),
                });
            }
        }
    }
    Ok(HttpResponse::Ok().json(GetAllManifestsStorerDataOutput {
        manifests: result_array,
    }))
}

pub async fn get_all_manifests_alter(
    data: web::Data<AppState>,
    req: web::Json<GetAllManifestsInput>,
) -> error::Result<HttpResponse> {
    let pool_id = transform_option_pool_id_value_reverse(req.pool_id);
    let uploader = transform_option_account_value_reverse(req.uploader.clone()).await;
    let storer = transform_option_account_value_reverse(req.storer.clone()).await;

    let pair = get_pair_from_seed(&Seed::from(String::from("//Alice")))?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .get_manifests(pool_id, uploader.unwrap(), storer.unwrap());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::GetManifests>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(GetAllManifestsOutput {
            manifests: transform_get_manifests(event.manifests),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::GetManifests"),
            description: format!(""),
        })),
    }
}

pub async fn get_all_available_manifests_alter(
    data: web::Data<AppState>,
    req: web::Json<GetAvailableManifestsInput>,
) -> error::Result<HttpResponse> {
    let pool_id = transform_option_pool_id_value_reverse(req.pool_id);

    let pair = get_pair_from_seed(&Seed::from(String::from("//Alice")))?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().fula().get_available_manifests(pool_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::GetAvailableManifests>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(GetAvailableManifestsOutput {
            manifests: transform_get_available_manifests(event.manifests),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::GetManifests"),
            description: format!(""),
        })),
    }
}

pub async fn get_all_manifests_storer_data_alter(
    data: web::Data<AppState>,
    req: web::Json<GetAllManifestsStorerDataInput>,
) -> error::Result<HttpResponse> {
    let pool_id = transform_option_pool_id_value_reverse(req.pool_id);

    let storer = transform_option_account_value_reverse(req.storer.clone()).await;

    let pair = get_pair_from_seed(&Seed::from(String::from("//Alice")))?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .get_manifests_storer_data(pool_id, storer.unwrap());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::GetManifestsStorerData>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(GetAllManifestsStorerDataOutput {
            manifests: transform_get_manifests_storer_data(event.manifests),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::GetManifests"),
            description: format!(""),
        })),
    }
}

// AUXILIAR FUNCTIONS

pub fn transform_vec_uploader_data_runtime_to_vec_uploader_data(
    users: Vec<UploaderDataRuntime<AccountId32>>,
) -> Vec<UploaderData> {
    let mut result_array = Vec::new();
    for user_data in users {
        result_array.push(UploaderData {
            uploader: user_data.uploader.into(),
            storers: transform_vec_string_to_account(transform_storage_output(
                user_data.storers.to_vec(),
            )),
            replication_available: (user_data.replication_factor - user_data.storers.len() as u16)
                .into(),
        });
    }
    return result_array;
}

pub fn verify_contains_storer(
    uploaders: Vec<UploaderData>,
    account: Account,
) -> Result<bool, actix_web::Error> {
    let mut result = false;

    // Convert the account string to a Public key and then to a byte array
    let account_public = Public::from_str(&account.as_str()).map_err(map_account_err)?;
    let account_public_bytes: [u8; 32] = account_public.0;
    let account_id = AccountId32::from(account_public_bytes);

    for user_data in uploaders.iter() {
        for storer_data in user_data.storers.iter() {
            // Convert each storer data string to a Public key and then to a byte array
            let storer_public = Public::from_str(&storer_data.as_str()).map_err(map_account_err)?;
            let storer_public_bytes: [u8; 32] = storer_public.0;
            let storer_account_id = AccountId32::from(storer_public_bytes);

            // Compare the account IDs
            if storer_account_id == account_id {
                result = true;
            }
        }
    }
    Ok(result)
}


pub fn verify_contains_uploader(
    uploaders: Vec<UploaderData>,
    account: Account,
) -> Result<bool, actix_web::Error> {
    let mut result = false;

    // Convert the account string to a Public key and then to a byte array
    let account_public = Public::from_str(&account.as_str()).map_err(map_account_err)?;
    let account_public_bytes: [u8; 32] = account_public.0;
    let account_id = AccountId32::from(account_public_bytes);

    for user_data in uploaders.iter() {
        // Convert the uploader string to a Public key and then to a byte array
        let uploader_public = Public::from_str(&user_data.uploader.as_str()).map_err(map_account_err)?;
        let uploader_public_bytes: [u8; 32] = uploader_public.0;
        let uploader_account_id = AccountId32::from(uploader_public_bytes);

        // Compare the account IDs
        if uploader_account_id == account_id {
            result = true;
        }
    }
    Ok(result)
}


pub fn get_vec_cids_from_input(cids: Vec<Cid>) -> Vec<BoundedVec<u8>> {
    return cids
        .iter()
        .map(|cid| BoundedVec(String::from(&cid.clone()).into_bytes()))
        .collect();
}

pub fn get_vec_manifests_from_input(manifests: Vec<serde_json::Value>) -> Vec<BoundedVec<u8>> {
    return manifests
        .iter()
        .map(|manifest_data| BoundedVec(serde_json::to_vec(manifest_data).unwrap_or_default()))
        .collect();
}

pub fn get_vec_cids_from_node(cids: Vec<Vec<u8>>) -> Vec<Cid> {
    return cids
        .iter()
        .map(|cid| Cid::from(String::from_utf8(cid.to_vec()).unwrap_or_default()))
        .collect();
}

pub fn get_vec_manifests_from_node(manifests: Vec<Vec<u8>>) -> Vec<serde_json::Value> {
    return manifests
        .iter()
        .map(|manifest_data| serde_json::from_slice(manifest_data.as_slice()).unwrap_or_default())
        .collect();
}

pub fn get_vec_pool_id_from_input(pool_ids: Vec<PoolId>) -> Vec<u32> {
    return pool_ids.iter().map(|pool_id| (*pool_id).into()).collect();
}

pub fn get_vec_pool_id_from_node(pool_ids: Vec<u32>) -> Vec<PoolId> {
    return pool_ids.iter().map(|pool_id| (*pool_id).into()).collect();
}

pub fn get_vec_replication_factor_from_input(
    repliaction_factors: Vec<ReplicationFactor>,
) -> Vec<u16> {
    return repliaction_factors
        .iter()
        .map(|repliaction_factor| (*repliaction_factor).into())
        .collect();
}

pub fn verify_availability(uploaders: Vec<UploaderData>) -> bool {
    return uploaders
        .iter()
        .position(|x| u16::from(x.replication_available) > 0)
        .is_some();
}

pub fn get_added_replication(uploaders: Vec<UploaderData>) -> ReplicationFactor {
    let mut result = 0;
    for user_data in uploaders {
        result += u16::from(user_data.replication_available);
    }
    return result.into();
}

pub async fn transform_option_account_value_reverse(
    value: Option<Account>,
) -> Result<Option<AccountId32>, actix_web::Error> {
    if let Some(value) = value {
        return Ok(Some(
            AccountId32::try_from(&value).map_err(map_account_err)?,
        ));
    }
    return Ok(None::<AccountId32>);
}

pub fn transform_option_pool_id_value_reverse(value: Option<PoolId>) -> Option<u32> {
    if let Some(value) = value {
        return Some(value.into());
    }
    return None::<u32>;
}

pub fn transform_get_manifests(
    manifests: Vec<ManifestWithPoolIdRuntime<u32, AccountId32, BoundedVec<u8>>>,
) -> Vec<Manifest> {
    let mut result = Vec::new();
    for manifest in manifests {
        result.push(Manifest {
            pool_id: manifest.pool_id.into(),
            uploaders: transform_vec_uploader_data_runtime_to_vec_uploader_data(
                manifest.users_data,
            ),
            manifest_metadata: serde_json::from_slice(manifest.manifest_metadata.0.as_slice())
                .unwrap_or_default(),
            size: manifest.size,
        })
    }
    return result;
}

pub fn transform_get_available_manifests(
    manifests: Vec<ManifestAvailableRuntime<u32, BoundedVec<u8>>>,
) -> Vec<ManifestAvailable> {
    let mut result = Vec::new();
    for manifest in manifests {
        result.push(ManifestAvailable {
            pool_id: manifest.pool_id.into(),
            manifest_metadata: serde_json::from_slice(manifest.manifest_metadata.0.as_slice())
                .unwrap_or_default(),
            replication_available: manifest.replication_factor.into(),
        })
    }
    return result;
}

pub fn transform_get_manifests_storer_data(
    manifests: Vec<StorerDataRuntime<u32, BoundedVec<u8>, AccountId32>>,
) -> Vec<ManifestStorageData> {
    let mut result = Vec::new();
    for manifest in manifests {
        result.push(ManifestStorageData {
            pool_id: manifest.pool_id.into(),
            account: manifest.account.into(),
            cid: Cid::from(String::from_utf8(manifest.cid.0).unwrap_or_default()),
            active_days: manifest.manifest_data.active_days,
            active_cycles: manifest.manifest_data.active_cycles,
            missed_cycles: manifest.manifest_data.missed_cycles,
            state: manifest.manifest_data.challenge_state.into(),
        })
    }
    return result;
}
