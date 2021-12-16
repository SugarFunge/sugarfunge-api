use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

#[derive(Deserialize)]
pub struct IssueCurrencyInput {
    input: IssueCurrencyArg,
}

#[derive(Deserialize)]
pub struct IssueCurrencyArg {
    seed: String,
    account: String,
    currency_id: u64,
    amount: i128,
}

impl Into<u64> for CurrencyId {
    fn into(self) -> u64 {
        match self {
            CurrencyId::Asset(asset) => asset as u64,
            CurrencyId::Id(id) => id,
        }
    }
}

#[derive(Serialize)]
pub struct IssueCurrencyOutput {
    asset_id: u64,
    account: String,
    amount: i128,
}

/// Issue amount of currency
pub async fn issue(
    data: web::Data<AppState>,
    req: web::Json<IssueCurrencyInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let account = subxt::sp_runtime::MultiAddress::Id(account);
    let currency_id = CurrencyId::Id(req.input.currency_id);
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .orml_tokens()
        .total_issuance(currency_id, None)
        .await;
    let total_issuance = result.map_err(map_subxt_err)?;
    let currency_id = CurrencyId::Id(req.input.currency_id);
    let call = sugarfunge::runtime_types::sugarfunge_runtime::Call::OrmlCurrencies(
        sugarfunge::runtime_types::orml_currencies::module::Call::update_balance {
            who: account,
            currency_id,
            amount: req.input.amount.saturating_add(total_issuance as i128),
        },
    );
    let result = api
        .tx()
        .sudo()
        .sudo(call)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::orml_currencies::events::BalanceUpdated>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(IssueCurrencyOutput {
            asset_id: event.0.into(),
            account: event.1.to_string(),
            amount: event.2,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::orml_currencies::events::BalanceUpdated"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceInput {
    input: CurrencyIssuanceArg,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceArg {
    currency_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencyIssuanceOutput {
    amount: u128,
}

/// Get total issuance for given currency
pub async fn issuance(
    data: web::Data<AppState>,
    req: web::Json<CurrencyIssuanceInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let currency_id = CurrencyId::Id(req.input.currency_id);
    let result = api
        .storage()
        .orml_tokens()
        .total_issuance(currency_id, None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(CurrencyIssuanceOutput { amount }))
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyInput {
    input: CurrencySupplyArg,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyArg {
    currency_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CurrencySupplyOutput {
    total_supply: u128,
}

/// Get total supply for given currency
pub async fn supply(
    data: web::Data<AppState>,
    req: web::Json<CurrencySupplyInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let currency_id = CurrencyId::Id(req.input.currency_id);
    let result = api
        .storage()
        .currency()
        .currency_assets(currency_id, None)
        .await;
    let asset_info = result.map_err(map_subxt_err)?;
    let total_supply = if let Some(asset_info) = asset_info {
        asset_info.total_supply
    } else {
        0
    };
    Ok(HttpResponse::Ok().json(CurrencySupplyOutput { total_supply }))
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyInput {
    input: MintCurrencyArg,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyArg {
    seed: String,
    currency_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintCurrencyOutput {
    currency_id: u64,
    amount: u128,
    account: String,
}

/// Mint amount of currency
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintCurrencyInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let currency_id = CurrencyId::Id(req.input.currency_id);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .currency()
        .mint(currency_id, req.input.amount)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::currency::events::AssetMint>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintCurrencyOutput {
            currency_id: event.0.into(),
            amount: event.1,
            account: event.2.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::AssetMint"),
        })),
    }
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceInput {
    input: AssetBalanceArg,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceArg {
    account: String,
    currency_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    amount: u128,
}

/// Get balance for given currency
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api.storage().currency().currency_class(None).await;
    let currency_class = result.map_err(map_subxt_err)?.unwrap_or(0);
    let result = api
        .storage()
        .asset()
        .balances(account, (currency_class, req.input.currency_id), None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetBalanceOutput { amount }))
}
