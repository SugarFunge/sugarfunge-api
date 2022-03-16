use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use sp_core::Pair;
use subxt::sp_runtime::traits::IdentifyAccount;
use subxt::PairSigner;
use sugarfunge_api_types::currency::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

/// Issue amount of currency
pub async fn issue(
    data: web::Data<AppState>,
    req: web::Json<IssueCurrencyInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let who = pair.public().into_account();
    let who = sp_core::crypto::AccountId32::from(who);
    let who = subxt::sp_runtime::MultiAddress::Id(who);
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
    let signer = PairSigner::new(pair);
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .orml_tokens()
        .total_issuance(currency_id, None)
        .await;
    let total_issuance = result.map_err(map_subxt_err)?;
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
    let call = sugarfunge::runtime_types::sugarfunge_runtime::Call::OrmlCurrencies(
        sugarfunge::runtime_types::orml_currencies::module::Call::update_balance {
            who,
            currency_id,
            amount: req.amount.saturating_add(total_issuance as i128),
        },
    );
    let result = api
        .tx()
        .sudo()
        .sudo(call)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::orml_currencies::events::BalanceUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(IssueCurrencyOutput {
            currency: Currency {
                class_id: event.currency_id.0.into(),
                asset_id: event.currency_id.1.into(),
            },
            who: event.who.into(),
            amount: event.amount,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::orml_currencies::events::BalanceUpdated"),
        })),
    }
}

/// Get total issuance for given currency
pub async fn issuance(
    data: web::Data<AppState>,
    req: web::Json<CurrencyIssuanceInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
    let result = api
        .storage()
        .orml_tokens()
        .total_issuance(currency_id, None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(CurrencyIssuanceOutput {
        amount: amount.into(),
    }))
}

/// Get total supply for given currency
pub async fn supply(
    data: web::Data<AppState>,
    req: web::Json<CurrencySupplyInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
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

/// Mint amount of currency
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintCurrencyInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .currency()
        .mint(currency_id, req.amount.into())
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::currency::events::Mint>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintCurrencyOutput {
            currency: Currency {
                class_id: event.currency_id.0.into(),
                asset_id: event.currency_id.1.into(),
            },
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::Mint"),
        })),
    }
}

/// Burn amount of currency
pub async fn burn(
    data: web::Data<AppState>,
    req: web::Json<BurnCurrencyInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let currency_id = CurrencyId(req.currency.class_id.into(), req.currency.asset_id.into());
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .currency()
        .burn(currency_id, req.amount.into())
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_first_event::<sugarfunge::currency::events::Burn>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(BurnCurrencyOutput {
            currency: Currency {
                class_id: event.currency_id.0.into(),
                asset_id: event.currency_id.1.into(),
            },
            amount: event.amount.into(),
            who: event.who.into(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::currency::events::Burn"),
        })),
    }
}
