use crate::fula::get_vec_cids_from_input;
use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::ext::sp_core::Pair;
use subxt::ext::sp_runtime::traits::IdentifyAccount;
use subxt::tx::PairSigner;
use sugarfunge_api_types::challenge::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;

pub async fn generate_challenge(
    data: web::Data<AppState>,
    req: web::Json<GenerateChallengeInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let api = &data.api;

    let call = sugarfunge::tx().fula().generate_challenge();

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::Challenge>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(GenerateChallengeOutput {
            challenger: event.challenger.into(),
            challenged: event.challenged.into(),
            cid: Cid::from(String::from_utf8(event.cid).unwrap_or_default()),
            state: event.state.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::GenerateChallenge"),
            description: format!(""),
        })),
    }
}

pub async fn verify_challenge(
    data: web::Data<AppState>,
    req: web::Json<VerifyChallengeInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed.clone())?;
    let signer = PairSigner::new(pair);

    let cids = get_vec_cids_from_input(req.cids.to_vec());

    let api = &data.api;

    let call = sugarfunge::tx()
        .fula()
        .verify_challenge(req.pool_id.into(), cids);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_fula_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::Challenge>()
        .map_err(map_subxt_err)?;
    match result {
        Some(_) => Ok(HttpResponse::Ok().json({})),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::fula::events::VerifyChallenge"),
            description: format!(""),
        })),
    }
}

pub async fn mint_labor_tokens(
    _data: web::Data<AppState>,
    req: web::Json<MintLaborTokensInput>,
) -> error::Result<HttpResponse> {
    // TO DO: The implementation of the calculation and mint from the fula-pallet

    let pair = get_pair_from_seed(&req.seed)?;
    let account: subxt::ext::sp_core::sr25519::Public = pair.public().into();
    let account = account.into_account();

    Ok(HttpResponse::Ok().json(MintLaborTokensOutput {
        account: Account::from(format!("{}", account)),
        class_id: req.class_id,
        asset_id: req.asset_id,
        amount: 0.into(),
    }))
}

pub async fn verify_pending_challenge(
    _data: web::Data<AppState>,
    req: web::Json<VerifyPendingChallengeInput>,
) -> error::Result<HttpResponse> {
    // TO DO: The implementation of the calculation and mint from the fula-pallet

    let pair = get_pair_from_seed(&req.seed)?;
    let account: subxt::ext::sp_core::sr25519::Public = pair.public().into();
    let account = account.into_account();

    Ok(HttpResponse::Ok().json(VerifyPendingChallengeOutput {
        account: Account::from(format!("{}", account)),
        pending: true,
    }))
}

pub async fn verify_file_size(
    _data: web::Data<AppState>,
    req: web::Json<VerifyFileSizeInput>,
) -> error::Result<HttpResponse> {
    // TO DO: The implementation of the calculation and mint from the fula-pallet

    let pair = get_pair_from_seed(&req.seed)?;
    let account: subxt::ext::sp_core::sr25519::Public = pair.public().into();
    let account = account.into_account();

    Ok(HttpResponse::Ok().json(VerifyFileSizeOutput {
        account: Account::from(format!("{}", account)),
        cids: Vec::<Cid>::new(),
    }))
}

pub async fn provide_file_size(
    _data: web::Data<AppState>,
    req: web::Json<ProvideFileSizeInput>,
) -> error::Result<HttpResponse> {
    // TO DO: The implementation of the calculation and mint from the fula-pallet

    let pair = get_pair_from_seed(&req.seed)?;
    let account: subxt::ext::sp_core::sr25519::Public = pair.public().into();
    let account = account.into_account();

    Ok(HttpResponse::Ok().json(ProvideFileSizeOutput {
        account: Account::from(format!("{}", account)),
        pool_id: req.pool_id,
        cids: req.cids.to_vec(),
        sizes: req.sizes.to_vec(),
    }))
}
