use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::storage::address::{StorageHasher, StorageMapKey};
use subxt::tx::PairSigner;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;
use codec::Decode;

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

pub async fn manifest(
    data: web::Data<AppState>,
    req: web::Json<ManifestsInput>,
) -> error::Result<HttpResponse> {
    let account_to = sp_core::crypto::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    // let account_from;
    let api = &data.api;
    let mut result_array = Vec::new();

    let mut query_key = sugarfunge::storage().fula().manifests_root().to_bytes();
    println!("query_key manifests_root len: {}", query_key.len());
    StorageMapKey::new(&account_to, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    println!("query_key account_to len: {}", query_key.len());
    // if let Some(operator_id) = req.operator.clone() {
    //     account_from = sp_core::crypto::AccountId32::try_from(&operator_id).map_err(map_account_err)?;
    //     StorageMapKey::new(&account_from, StorageHasher::Blake2_128Concat).to_bytes(&mut query_key);
    //     println!("query_key account_from len: {}", query_key.len());
    // }

    let keys = api
        .storage()
        .fetch_keys(&query_key, 1000, None, None)
        .await
        .map_err(map_subxt_err)?;

    println!("Obtained keys:");
    for key in keys.iter() {
        println!("Key: len: {} 0x{}", key.0.len(), hex::encode(&key));

        if let Some(storage_data) = api
            .storage()
            .fetch_raw(&key.0, None)
            .await
            .map_err(map_subxt_err)?
        {
            let value = u64::decode(&mut &storage_data[..]).unwrap();
            let item = Manifest{
                from: req.operator.clone().unwrap(),
                to: req.account.clone(),
                manifest: json!(""),
                value
            };
            result_array.push(item);
        }
    }
    Ok(HttpResponse::Ok().json(ManifestsOutput {
        manifests: result_array,
    }))
}