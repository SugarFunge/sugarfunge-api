use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use hex::ToHex;
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::bundle::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

fn hash(s: &[u8]) -> sp_core::H256 {
    sp_io::hashing::blake2_256(s).into()
}

pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let schema = (
        BoundedVec(req.schema.class_ids.to_vec()),
        BoundedVec(
            req.schema
                .asset_ids
                .iter()
                .map(|x| BoundedVec(x.to_vec()))
                .collect(),
        ),
        BoundedVec(
            req.schema
                .amounts
                .iter()
                .map(|x| BoundedVec(x.to_vec()))
                .collect(),
        ),
    );
    let bundle_id = hash(&schema.encode());
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .bundle()
        .register_bundle(
            req.class_id.into(),
            req.asset_id.into(),
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
            who: event.who.into(),
            bundle_id: event.bundle_id.encode_hex(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
        })),
    }
}

pub async fn mint_bundle(
    data: web::Data<AppState>,
    req: web::Json<MintBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from =
        sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .bundle()
        .mint_bundle(account_from, account_to, bundle_id, req.amount.into())
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
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.to_string(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Mint"),
        })),
    }
}

pub async fn burn_bundle(
    data: web::Data<AppState>,
    req: web::Json<BurnBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_from =
        sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let bundle_id = sp_core::H256::from_str(&req.bundle_id).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .bundle()
        .burn_bundle(account_from, account_to, bundle_id, req.amount.into())
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
            who: event.who.into(),
            from: event.from.into(),
            to: event.to.into(),
            bundle_id: event.bundle_id.to_string(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Burn"),
        })),
    }
}
