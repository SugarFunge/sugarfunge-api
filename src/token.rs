use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct CreateClassInput {
    input: CreateClassArg,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassArg {
    seed: String,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateClassOutput {
    class_id: u64,
    account: String,
}

/// Create a token class for an account
pub async fn create_class(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .token()
        .create_class(metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;

    let result = result
        .find_event::<sugarfunge::token::events::ClassCreated>()
        .map_err(map_scale_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateClassOutput {
            class_id: event.0,
            account: event.1.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::token::events::ClassCreated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateInput {
    input: CreateArg,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArg {
    seed: String,
    class_id: u64,
    token_id: u64,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct CreateOutput {
    class_id: u64,
    token_id: u64,
    account: String,
}

/// Create a token for an account
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap_or_default();
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .token()
        .create_token(req.input.class_id, req.input.token_id, metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;

    let result = result
        .find_event::<sugarfunge::token::events::TokenCreated>()
        .map_err(map_scale_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateOutput {
            class_id: event.0,
            token_id: event.1,
            account: event.2.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::token::events::ClassCreated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct MintInput {
    input: MintArg,
}

#[derive(Serialize, Deserialize)]
pub struct MintArg {
    seed: String,
    account: String,
    class_id: u64,
    token_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintOutput {
    account: String,
    class_id: u64,
    token_id: u64,
    amount: u128,
}

/// Mint amount of token id to account
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .token()
        .mint(
            account,
            req.input.class_id,
            req.input.token_id,
            req.input.amount,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::token::events::Mint>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintOutput {
            account: event.0.to_string(),
            class_id: event.1,
            token_id: event.2,
            amount: event.3,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::TokenMint"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenBalanceInput {
    input: TokenBalanceArg,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBalanceArg {
    account: String,
    class_id: u64,
    token_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct TokenBalanceOutput {
    amount: u128,
}

/// Get balance for given token id
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<TokenBalanceInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .token()
        .balances(account, (req.input.class_id, req.input.token_id), None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(TokenBalanceOutput { amount }))
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromInput {
    input: TransferFromArg,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromArg {
    seed: String,
    from: String,
    to: String,
    class_id: u64,
    token_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromOutput {
    from: String,
    to: String,
    class_id: u64,
    token_id: u64,
    amount: u128,
}

/// Transfer token from to accounts
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferFromInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account_from =
        sp_core::sr25519::Public::from_str(&req.input.from).map_err(map_account_err)?;
    let account_to = sp_core::sr25519::Public::from_str(&req.input.to).map_err(map_account_err)?;
    let account_from = sp_core::crypto::AccountId32::from(account_from);
    let account_to = sp_core::crypto::AccountId32::from(account_to);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .token()
        .transfer_from(
            account_from,
            account_to,
            req.input.class_id,
            req.input.token_id,
            req.input.amount,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::token::events::Transferred>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(TransferFromOutput {
            from: event.0.to_string(),
            to: event.1.to_string(),
            class_id: event.2,
            token_id: event.3,
            amount: event.4,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::token::events::Transferred"),
        })),
    }
}
