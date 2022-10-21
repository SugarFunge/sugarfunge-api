use std::str::FromStr;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
//use sp_core::offchain::storage;
use subxt::storage::address::{StorageHasher, StorageMapKey};
use subxt::tx::PairSigner;
use sp_core::crypto::AccountId32;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::primitives::{Account, Cid, transform_vec_string_to_account};
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;
use codec::Decode;
use sugarfunge_api_types::sugarfunge::runtime_types::functionland_fula::Manifest as ManifestRuntime;

pub async fn update_manifest(
    data: web::Data<AppState>,
    req: web::Json<UpdateManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_storage = sp_core::crypto::AccountId32::try_from(&req.storage).map_err(map_account_err)?;

    let cid: Vec<u8> = req.manifest_metadata["job"]["uri"].to_string().replace("\"", "").into_bytes();
    let cid = BoundedVec(cid);

    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest_metadata).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().update_manifest(account_storage, manifest, cid, req.replication_factor);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestOutput>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ManifestOutput {
            uploader: event.uploader.into(),
            storage: transform_vec_string_to_account(transform_storage_output(event.storage)),
            manifest_metadata: serde_json::from_slice(event.manifest.as_slice()).unwrap_or_default(),
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
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let cid: Vec<u8> = req.manifest_metadata["job"]["uri"].to_string().replace("\"", "").into_bytes();
    let cid = BoundedVec(cid);

    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest_metadata).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().upload_manifest(manifest,cid, req.replication_factor);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestOutput>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(ManifestOutput {
            uploader: event.uploader.into(),
            storage: transform_vec_string_to_account(transform_storage_output(event.storage)),
            manifest_metadata: serde_json::from_slice(event.manifest.as_slice()).unwrap_or_default(),
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
    let account_uploader = sp_core::crypto::AccountId32::try_from(&req.uploader).map_err(map_account_err)?;

    let api = &data.api;

    let call = sugarfunge::tx().fula().storage_manifest(account_uploader,cid);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::StorageManifestOutput>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(StorageManifestOutput {
            uploader: event.uploader.into(),
            storage: event.storage.into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::StorageManifest"),
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
    // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
    let cid = BoundedVec(cid);
    let api = &data.api;

    let call = sugarfunge::tx().fula().remove_manifest(cid);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestRemoved>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveManifestOutput {
            uploader: event.uploader.into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default())
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveManifest"),
            description: format!(""),
        })),
    }
}

pub async fn remove_from_manifest(
    data: web::Data<AppState>,
    req: web::Json<RemoveFromManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let cid: Vec<u8> = String::from(&req.cid.clone()).into_bytes();
    // let cid: Vec<u8> = serde_json::to_vec(&req.cid.clone()).unwrap_or_default();
    let cid = BoundedVec(cid);
    let storer = sp_core::sr25519::Public::from_str(&req.storage.as_str()).map_err(map_account_err)?;
    let storer = sp_core::crypto::AccountId32::from(storer);

    let api = &data.api;

    let call = sugarfunge::tx().fula().remove_storer(storer, cid);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::RemoveStorerOutput>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveFromManifestOutput {
            uploader: event.uploader.into(),
            storage: event.storage.unwrap().into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default())
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::RemoveManifest"),
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

    if let Some(value) = req.account.clone() {
        let account = sp_core::crypto::AccountId32::try_from(&value).map_err(map_account_err)?;
        StorageMapKey::new(&account, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    }
    // println!("query_key account_to len: {}", query_key.len());

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        // let account_to_idx = 48;
        // let account_to_key = key.0.as_slice()[account_to_idx..(account_to_idx + 32)].to_vec();
        // let account_to_id = AccountId32::decode(&mut &account_to_key[..]);
        // let account_to_id = Account::from(account_to_id.unwrap());
        // println!("account_to_id: {:?}", account_to_id);

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
            let value = 
             ManifestRuntime::<AccountId32,Vec<u8>>::decode(&mut &storage_data[..]);
            let value =value.unwrap();
            let manifest_data = ManifestData{
                uploader: Account::from(value.manifest_data.uploader),
                manifest_metadata:serde_json::from_slice(value.manifest_data.manifest_metadata.as_slice()).unwrap_or_default(),
            };
            let storage = value.storage;

            let mut storage_vec:Vec<Account> = Vec::new();

            for storer in storage {
                let current_account = Account::try_from(storer).unwrap();
                storage_vec.push(current_account);
            }            

            result_array.push(Manifest { storage:storage_vec , manifest_data });
        }
    }
    Ok(HttpResponse::Ok().json(GetAllManifestsOutput {
        manifests: result_array,
    }))
}

pub async fn get_available_manifests(
    data: web::Data<AppState>,
) -> error::Result<HttpResponse> {
    let api = &data.api;
    let mut result_array = Vec::new();

    let query_key = sugarfunge::storage().fula().manifests_root().to_bytes();
    // println!("query_key manifests_root len: {}", query_key.len());

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    // println!("Obtained keys:");
    for key in keys.iter() {
        // println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = 
             ManifestRuntime::<AccountId32,Vec<u8>>::decode(&mut &storage_data[..]);
            let value =value.unwrap(); 

            /*for current_manifest in value.storage {  
                if current_manifest{ //verificar si esta vacia la cuenta
                    let manifest = serde_json::from_slice(value.manifest_data.manifest_metadata.as_slice()).unwrap_or_default();
                    result_array.push(ManifestAvailable{manifest});
                }
            }   */         
        }
    }
    Ok(HttpResponse::Ok().json(GetAvailableManifestsOutput{
        manifests: result_array,
    }))
}

pub fn transform_storage_output(storers: Vec<AccountId32>) -> Vec<String> {
    storers
        .into_iter()
        .map(|current_storer| current_storer.to_string())
        .collect()
}
