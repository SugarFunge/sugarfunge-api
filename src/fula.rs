use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::storage::address::{StorageHasher, StorageMapKey};
use subxt::tx::PairSigner;
use sp_core::crypto::AccountId32;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::primitives::Account;
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
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().update_manifest(account_to, manifest);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(UpdateManifestOutput {
            from: event.from.into(),
            to: event.to.into(),
            manifest: serde_json::from_slice(event.manifest.as_slice()).unwrap_or_default(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UpdateManifests"),
            description: format!(""),
        })),
    }
}

pub async fn burn_manifest(
    data: web::Data<AppState>,
    req: web::Json<UpdateManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().burn(account_to, manifest);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestBurned>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(UpdateManifestOutput {
            from: event.from.into(),
            to: event.to.into(),
            manifest: serde_json::from_slice(event.manifest.as_slice()).unwrap_or_default(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::UpdateManifests"),
            description: format!(""),
        })),
    }
}

pub async fn manifest(
    data: web::Data<AppState>,
    req: web::Json<ManifestsInput>,
) -> error::Result<HttpResponse> {
    let account_to = sp_core::crypto::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    let account_from;
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage().fula().manifests_root().to_bytes();
    // println!("query_key manifests_root len: {}", query_key.len());
    StorageMapKey::new(&account_to, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    // println!("query_key account_to len: {}", query_key.len());
    if let Some(operator_id) = req.operator.clone() {
        account_from = sp_core::crypto::AccountId32::try_from(&operator_id).map_err(map_account_err)?;
        StorageMapKey::new(&account_from, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
        // println!("query_key account_from len: {}", query_key.len());
    }

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
            let item = Manifest{
                from: Account::from(value.from),
                to: Account::from(value.to),
                manifest:serde_json::from_slice(value.manifest.as_slice()).unwrap_or_default(),
            };
            result_array.push(item);
        }
    }
    Ok(HttpResponse::Ok().json(ManifestsOutput {
        manifests: result_array,
    }))
}