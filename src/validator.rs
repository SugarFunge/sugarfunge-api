use std::str::FromStr;

use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
// TO DO: Here is using the exporting from the dependencies like in the sugarfunge-node is done
use subxt::tx::PairSigner;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;
// TO DO: Here is the exported of the session keys type needed for the set_keys function
use sugarfunge_api_types::sugarfunge::runtime_types::sugarfunge_runtime::opaque::SessionKeys;
use sugarfunge_api_types::validator::*;

pub async fn add_validator(
    data: web::Data<AppState>,
    req: web::Json<AddValidatorInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let validator_public =
        sp_core::sr25519::Public::from_str(req.validator_id.as_str()).map_err(map_account_err)?;
    let validator_bytes: [u8; 32] = validator_public.0; // Convert Public key to a byte array
    let validator_id = subxt::utils::AccountId32::from(validator_bytes); // Create AccountId32 from the byte array
    let call = sugarfunge::runtime_types::sugarfunge_validator_set::pallet::Call::add_validator {
        validator_id,
    };
    let call = sugarfunge::runtime_types::sugarfunge_runtime::RuntimeCall::ValidatorSet(call);
    let api = &data.api;

    let call_value = sugarfunge::tx().sudo().sudo(call);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call_value, &signer, Default::default())
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
            description: String::new(),
        })),
    }
}

pub async fn remove_validator(
    data: web::Data<AppState>,
    req: web::Json<RemoveValidatorInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let validator_public =
        sp_core::sr25519::Public::from_str(req.validator_id.as_str()).map_err(map_account_err)?;
    let validator_bytes: [u8; 32] = validator_public.0; // Convert Public key to a byte array

    // You'll need to replace this with the correct method from your specific Substrate framework
    // This is just a placeholder and might not be correct for your setup
    let validator_id = subxt::utils::AccountId32::from(validator_bytes); // Attempt to create AccountId32 from the byte array

    let call =
        sugarfunge::runtime_types::sugarfunge_validator_set::pallet::Call::remove_validator {
            validator_id: validator_id,
        };
    let call = sugarfunge::runtime_types::sugarfunge_runtime::RuntimeCall::ValidatorSet(call);
    let call_value = sugarfunge::tx().sudo().sudo(call);
    let api = &data.api;

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call_value, &signer, Default::default())
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
            description: String::new(),
        })),
    }
}

pub async fn set_keys(
    data: web::Data<AppState>,
    req: web::Json<SetKeysInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    // TO DO: Here the types converted are not the ones expected, but if you check the sugarfunge-node it is executed like this and it works
    let aura = sp_core::sr25519::Public::from_str(req.aura.as_str()).map_err(map_account_err)?;
    let grandpa =
        sp_core::sr25519::Public::from_str(req.grandpa.as_str()).map_err(map_account_err)?;

    let api = &data.api;

    let aura: sugarfunge_api_types::sugarfunge::runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public = unsafe { std::mem::transmute(aura) };
    let grandpa: sugarfunge_api_types::sugarfunge::runtime_types::sp_consensus_grandpa::app::Public = unsafe { std::mem::transmute(grandpa) };

    // TODO: Here is where the error happens because the types are not the ones expected, if you try to use .into() it requires to create a Into<> function maybe that is the best approach
    let session_keys = SessionKeys { aura, grandpa };

    let call = sugarfunge::tx()
        .session()
        .set_keys(session_keys, "0x".into());

    let _result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    Ok(HttpResponse::Ok().json(SetKeysOutput {
        aura: req.aura.clone(),
        grandpa: req.grandpa.clone(),
    }))
}
