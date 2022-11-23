use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::ext::sp_runtime::AccountId32;
use subxt::tx::PairSigner;
use sugarfunge_api_types::pool::*;
use sugarfunge_api_types::primitives::Account;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;
// use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;

pub async fn create_pool(
    data: web::Data<AppState>,
    req: web::Json<CreatePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let pool_name = req.pool_name.clone().into_bytes();

    let peer_id = req.peer_id.clone().into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().create(pool_name, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::PoolCreated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreatePoolOutput {
            owner: transform_option_value(event.owner).into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::PoolCreated"),
            description: format!(""),
        })),
    }
}

pub async fn leave_pool(
    data: web::Data<AppState>,
    req: web::Json<LeavePoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().leave_pool(req.pool_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::ParticipantLeft>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(LeavePoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn join_pool(
    data: web::Data<AppState>,
    req: web::Json<JoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let peer_id = req.peer_id.clone().into_bytes();
    let peer_id = BoundedVec(peer_id);

    let api = &data.api;

    let call = sugarfunge::tx().pool().join(req.pool_id, peer_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::JoinRequested>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(JoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::ParticipantLeft"),
            description: format!(""),
        })),
    }
}

pub async fn cancel_join_pool(
    data: web::Data<AppState>,
    req: web::Json<CancelJoinPoolInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().pool().cancel_join(req.pool_id);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::RequestWithdrawn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CancelJoinPoolOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::RequestWithdrawn"),
            description: format!(""),
        })),
    }
}

pub async fn vote(
    data: web::Data<AppState>,
    req: web::Json<VoteInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);

    let account = AccountId32::try_from(&req.account).map_err(map_account_err)?;

    let api = &data.api;

    let call = sugarfunge::tx()
        .pool()
        .vote(req.pool_id, account, req.vote_value);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_pool_err)?;
    let result = result
        .find_first::<sugarfunge::pool::events::Accepted>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(VoteOutput {
            account: event.account.into(),
            pool_id: event.pool_id,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::pool::events::Accepted"),
            description: format!(""),
        })),
    }
}

fn transform_option_value(value: Option<AccountId32>) -> Option<Account> {
    if let Some(value) = value {
        return Some(value.into());
    }
    return None::<Account>;
}
