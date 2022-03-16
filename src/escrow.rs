use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use sp_core::crypto::AccountId32;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge_api_types::escrow::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::frame_support::storage::bounded_vec::BoundedVec;

pub async fn register(
    data: web::Data<AppState>,
    req: web::Json<RegisterEscrowInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.metadata).unwrap_or_default();
    let metadata = BoundedVec(metadata);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .escrow()
        .register_escrow(req.class_id.into(), metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::escrow::events::Register>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(RegisterEscrowOutput {
            who: event.who.into(),
            class_id: event.class_id.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::escrow::events::Register"),
        })),
    }
}

fn transform_owners_input(in_owners: Vec<String>) -> Vec<AccountId32> {
    in_owners
        .into_iter()
        .map(|current_owner| {
            sp_core::crypto::AccountId32::from(
                sp_core::sr25519::Public::from_str(&current_owner).unwrap(),
            )
        })
        .collect()
}

fn transform_owners_output(in_owners: Vec<AccountId32>) -> Vec<String> {
    in_owners
        .into_iter()
        .map(|current_owner| current_owner.to_string())
        .collect()
}

pub async fn create_escrow(
    data: web::Data<AppState>,
    req: web::Json<CreateEscrowInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let owners = transform_owners_input(req.owners.clone());
    /*let owners =
        req.owners.clone()
        .into_iter()
        .map(|current_owner|
            sp_core::crypto::AccountId32::from(sp_core::sr25519::Public::from_str(&current_owner).unwrap())
        )
        .collect()
    ;*/

    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .escrow()
        .create_account(req.class_id.into(), owners, req.shares.clone())
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::escrow::events::AccountCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateEscrowOutput {
            escrow: event.escrow.to_string(),
            class_id: event.class_id.into(),
            asset_id: event.asset_id.into(),
            owners: transform_owners_output(event.owners),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::escrow::events::AccountCreated"),
        })),
    }
}

pub async fn sweep_assets(
    data: web::Data<AppState>,
    req: web::Json<SweepAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let escrow = sp_core::sr25519::Public::from_str(&req.escrow).map_err(map_account_err)?;
    let escrow = sp_core::crypto::AccountId32::from(escrow);
    let to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .escrow()
        .sweep_assets(to, escrow)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::escrow::events::Sweep>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(SweepAssetsOutput {
            escrow: event.escrow.to_string(),
            who: event.who.into(),
            to: event.to.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::escrow::events::Sweep"),
        })),
    }
}

pub async fn deposit_assets(
    data: web::Data<AppState>,
    req: web::Json<DepositAssetsInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let escrow = sp_core::sr25519::Public::from_str(&req.escrow).map_err(map_account_err)?;
    let escrow = sp_core::crypto::AccountId32::from(escrow);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .escrow()
        .deposit_assets(
            escrow,
            req.class_ids.clone(),
            req.asset_ids.clone(),
            req.amounts.clone(),
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::escrow::events::Deposit>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(DepositAssetsOutput {
            escrow: event.escrow.to_string(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::escrow::events::Deposit"),
        })),
    }
}
