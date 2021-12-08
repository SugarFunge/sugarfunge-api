use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpRequest, HttpResponse};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::Pair;
use std::str::FromStr;
use subxt::sp_runtime::traits::IdentifyAccount;
use subxt::PairSigner;

#[derive(Serialize, Deserialize)]
pub struct CreateAccountOutput {
    seed: String,
    account: String,
}

/// Generate a unique seed and its associated account
pub async fn create(_req: HttpRequest) -> error::Result<HttpResponse> {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let seed = hex::encode(seed);
    let seed = format!("//{}", seed);
    let pair = get_pair_from_seed(&seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(CreateAccountOutput {
        seed,
        account: format!("{}", account),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountInput {
    input: FundAccountArg,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountArg {
    seed: String,
    account: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct FundAccountOutput {
    amount: u128,
}

/// Fund a given account with the default amount
pub async fn fund(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let account = subxt::sp_runtime::MultiAddress::Id(account);
    let amount_input = req.input.amount;
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .balances()
        .transfer(account, amount_input)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::balances::events::Transfer>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(FundAccountOutput { amount: event.2 })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceInput {
    input: AccountBalanceArg,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceArg {
    account: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceOutput {
    balance: u128,
}

/// Get balance for given account
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AccountBalanceInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api.storage().system().account(account, None).await;
    let data = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AccountBalanceOutput {
        balance: data.data.free,
    }))
}
