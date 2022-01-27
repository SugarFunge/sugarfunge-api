use std::str::FromStr;

use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    seed: String,
    creator: String,
    class_id: u64,
    asset_id: u64,
    bundle_id: String,
    schema: serde_json::Value,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    bundle_id: String,
    creator: String,
    class_id: u64,
    asset_id: u64,
}

pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.creator).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(to);
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let schema: Vec<u8> = serde_json::to_vec(&req.schema).unwrap_or_default();
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .bundle()
    .register_bundle(
        to,
        req.class_id,
        req.asset_id,
        bundle_id,
        schema,
        metadata,
    )
    .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::bundle::events::Register>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RegisterBundleOutput {            
            creator: event.creator.to_string(),
            bundle_id: event.bundle_id.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
        })),
    }
}
