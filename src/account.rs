use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpRequest, HttpResponse};
use rand::prelude::*;
use serde_json::json;
use sp_core;
use sp_core::Pair;
use sp_runtime::traits::IdentifyAccount;
use subxt::tx::PairSigner;
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
    let account: sp_core::sr25519::Public = pair.public().into();
    let account = account.into_account();
    Ok(HttpResponse::Ok().json(CreateAccountOutput {
        seed,
        account: Account::from(format!("{}", account)),
    }))
}

/// Compute account from seed
pub async fn seeded(req: web::Json<SeededAccountInput>) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let account = pair.public().into_account();
    Ok(HttpResponse::Ok().json(SeededAccountOutput {
        seed: req.seed.clone(),
        account: Account::from(format!("{}", account)),
    }))
}

/// Fund a given account with amount
pub async fn fund(
    data: web::Data<AppState>,
    req: web::Json<FundAccountInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    //let signer = sp_core::sr25519::Pair::try_from(pair).unwrap();
    let signer = PairSigner::new(pair);
    let account = subxt::utils::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let account = subxt::utils::MultiAddress::Id(account);
    let amount_input = req.amount;
    let api = &data.api;

    let call = sugarfunge::tx()
        .balances()
        .transfer(account, amount_input.into());

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::balances::events::Transfer>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(FundAccountOutput {
            from: event.from.into(),
            to: event.to.into(),
            amount: event.amount.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::Transfer"),
            description: format!("Error in account::fund"),
        })),
    }
}

/// Get balance for given account
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AccountBalanceInput>,
) -> error::Result<HttpResponse> {
    let account = subxt::utils::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    let api = &data.api;

    let call = sugarfunge::storage().system().account(&account);

    //let result = api.storage().fetch(&call, None).await;
    let block = api.blocks().at(None).await.unwrap();
    let data = block.storage().fetch(&call).await.unwrap();
    match data {
        Some(data) => Ok(HttpResponse::Ok().json(AccountBalanceOutput {
            balance: data.data.free.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::balance"),
            description: format!("Error in account::balance"),
        })),
    }
}

/// Check if account exists and is active
pub async fn exists(
    data: web::Data<AppState>,
    req: web::Json<AccountExistsInput>,
) -> error::Result<HttpResponse> {
    let account = subxt::utils::AccountId32::try_from(&req.account).map_err(map_account_err)?;
    let account_out = account.clone();
    let api = &data.api;

    let call = sugarfunge::storage().system().account(&account);

    let storage = api.storage().at(None).await.map_err(map_subxt_err)?;
    let data = storage.fetch(&call).await.map_err(map_subxt_err)?;
    match data {
        Some(data) => Ok(HttpResponse::Ok().json(AccountExistsOutput {
            account: account_out.into(),
            exists: data.providers > 0,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::balances::events::balance"),
            description: format!("Error in account::exist"),
        })),
    }
}
