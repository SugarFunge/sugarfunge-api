use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowInput {
    seed: String,
    owner: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEscrowOutput {
    escrow: String,
    operator: String,
    owner: String,
}

pub async fn create_escrow(
    data: web::Data<AppState>,
    req: web::Json<CreateEscrowInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let to = sp_core::sr25519::Public::from_str(&req.owner).map_err(map_account_err)?;
    let to = sp_core::crypto::AccountId32::from(owner);
    let api = data.api.lock().unwrap();
    let result = api
    .tx()
    .escrow()
    .create_escrow(
        to,
    )
    .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::dex::events::CurrencyToAsset>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateEscrowOutput {
            escrow: event.escrow,
            operator: event.operator,
            owner: event.owner,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::escrow::events::Created"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsInput {
    seed: String,
    escrow: String,
    class_id: u64,
    asset_ids: Vec<u64>,
    amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct DepositAssetsOutput {
    escrow: String,
    operator: String,
    owner: String,
}