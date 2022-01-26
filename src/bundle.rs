use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleInput {
    creator: String,
    class_id: u64,
    asset_id: u64,
    bundle_id: u64,
    schema: serde_json::Value,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterBundleOutput {
    bundle_id: u64,
    creator: String,
    class_id: u64,
    asset_id: u64,
}

pub async fn register_bundle(
    data: web::Data<AppState>,
    req: web::Json<RegisterBundleInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.creator)?;
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .bundle()
    .do_register_bundle(
        req.creator,
        req.class_id,
        req.asset_id,
        req.bundle_id
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
            bundle_id: event.bundle_id,
            creator: event.creator.to_string(),
            class_id: event.class_id,
            asset_id: event.asset_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
        })),
    }
}
