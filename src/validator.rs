use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use std::str::FromStr;
use subxt::tx::PairSigner;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::validator::*;

pub async fn add_validator(
    data: web::Data<AppState>,
    req: web::Json<AddValidatorInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let validator_id =
        sp_core::sr25519::Public::from_str(&req.validator_id.as_str()).map_err(map_account_err)?;
    let validator_id = sp_core::crypto::AccountId32::from(validator_id);
    let call = sugarfunge::runtime_types::sugarfunge_validator_set::pallet::Call::add_validator {
        validator_id,
    };
    let call = sugarfunge::runtime_types::sugarfunge_runtime::Call::ValidatorSet(call);
    let api = &data.api;
    let result = api
        .tx()
        .sudo()
        .sudo(call)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;

    let result = result
        .find_first::<sugarfunge::validator_set::events::ValidatorAdditionInitiated>()
        .map_err(map_subxt_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(AddValidatorOutput {
            validator_id: ValidatorId::from(event.0.to_string()),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::validator::events::AddValidator"),
            description: format!(""),
        })),
    }
}

pub async fn remove_validator(
    data: web::Data<AppState>,
    req: web::Json<RemoveValidatorInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let validator_id =
        sp_core::sr25519::Public::from_str(&req.validator_id.as_str()).map_err(map_account_err)?;
    let validator_id = sp_core::crypto::AccountId32::from(validator_id);
    let call =
        sugarfunge::runtime_types::sugarfunge_validator_set::pallet::Call::remove_validator {
            validator_id,
        };
    let call = sugarfunge::runtime_types::sugarfunge_runtime::Call::ValidatorSet(call);
    let api = &data.api;
    let result = api
        .tx()
        .sudo()
        .sudo(call)
        .map_err(map_subxt_err)?
        .sign_and_submit_then_watch(&signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;

    let result = result
        .find_first::<sugarfunge::validator_set::events::ValidatorRemovalInitiated>()
        .map_err(map_subxt_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RemoveValidatorOutput {
            validator_id: ValidatorId::from(event.0.to_string()),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::validator::events::RemoveValidator"),
            description: format!(""),
        })),
    }
}
