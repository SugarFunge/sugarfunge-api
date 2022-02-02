use std::str::FromStr;

use crate::state::*;
use crate::sugarfunge;
use crate::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct BundleSchema {
    class_ids: Vec<u64>,
    asset_ids: Vec<Vec<u64>>,
    amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    seed: String,
    class_id: u64,
    asset_id: u64,
    bundle_id: String,
    schema: BundleSchema,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    bundle_id: String,
    who: String,
    class_id: u64,
    asset_id: u64,
}

pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let schema = (
        BoundedVec(req.schema.class_ids.to_vec()),
        BoundedVec(req.schema.asset_ids.iter().map(|x| BoundedVec(x.to_vec())).collect()),
        BoundedVec(req.schema.amounts.iter().map(|x| BoundedVec(x.to_vec())).collect())
    );
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .bundle()
    .register_bundle(
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
            who: event.who.to_string(),
            bundle_id: event.bundle_id.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleInput {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintBundleOutput {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

pub async fn mint_bundle(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from = sp_core::sr25519::Public::from_str(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let account_from = sp_core::crypto::AccountId32::from(account_from);
    let account_to = sp_core::crypto::AccountId32::from(account_to);
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .bundle()
    .mint_bundle(
        account_from,
        account_to,
        bundle_id,
        req.amount,
    )
    .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::bundle::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintBundleOutput {            
            who: event.who.to_string(),
            from: event.from.to_string(),
            to: event.to.to_string(),
            bundle_id: event.bundle_id.to_string(),
            amount: event.amount,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Mint"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleInput {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleOutput {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

pub async fn burn_bundle(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from = sp_core::sr25519::Public::from_str(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::sr25519::Public::from_str(&req.to).map_err(map_account_err)?;
    let account_from = sp_core::crypto::AccountId32::from(account_from);
    let account_to = sp_core::crypto::AccountId32::from(account_to);
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .bundle()
    .burn_bundle(
        account_from,
        account_to,
        bundle_id,
        req.amount,
    )
    .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::bundle::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BurnBundleOutput {            
            who: event.who.to_string(),
            from: event.from.to_string(),
            to: event.to.to_string(),
            bundle_id: event.bundle_id.to_string(),
            amount: event.amount,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Burn"),
        })),
    }
}