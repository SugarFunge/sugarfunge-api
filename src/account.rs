use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpRequest, HttpResponse};
use rand::prelude::*;
use serde_json::json;
use sp_core::Pair;
use subxt::{sp_runtime::traits::IdentifyAccount, PairSigner};
use sugarfunge_api_types::account::*;
use sugarfunge_api_types::primitives::*;
use sugarfunge_api_types::sugarfunge;

/// Generate a unique seed and its associated account
pub async fn create(_req: HttpRequest) -> error::Result<HttpResponse> {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    let seed = hex::encode(seed);
    let seed = format!("//{}", seed);
    let seed = Seed::from(seed);
    let pair = get_pair_from_seed(&seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(CreateAccountOutput {
        seed,
        account: format!("{}", account),
    }))
}

/// Fund a given account with amount
pub async fn fund(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let account = subxt::sp_runtime::MultiAddress::Id(account);
    let amount_input = req.amount;
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .balances()
        .transfer(account, amount_input.into())
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::balances::events::Transfer>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(FundAccountOutput {
            from: event.from.into(),
            to: event.to.into(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
        })),
    }
}

/// Get balance for given account
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AccountBalanceInput>,
) -> error::Result<HttpResponse> {
    let account = sp_core::crypto::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    let api = data.api.lock().unwrap();
    let result = api.storage().system().account(account, None).await;
    let data = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AccountBalanceOutput {
        balance: data.data.free.into(),
    }))
}
