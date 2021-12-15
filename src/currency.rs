use crate::state::*;
use crate::sugarfunge;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use subxt::PairSigner;
use sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId;

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

/// Create a class for an account
pub async fn create(
    data: web::Data<AppState>,
    req: web::Json<CreateClassInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);

    let metadata: Vec<u8> = serde_json::to_vec(&req.input.metadata).unwrap_or_default();

    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .create_class(metadata)
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;

    let result = result
        .find_event::<sugarfunge::asset::events::ClassCreated>()
        .map_err(map_scale_err)?;

    match result {
        Some(event) => Ok(HttpResponse::Ok().json(CreateClassOutput {
            class_id: event.0,
            account: event.1.to_string(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::ClassCreated"),
        })),
    }
}

#[derive(Deserialize)]
pub struct IssueAssetInput {
    input: IssueAssetArg,
}

#[derive(Deserialize)]
pub struct IssueAssetArg {
    seed: String,
    account: String,
    asset_id: u64,
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
pub struct IssueAssetOutput {
    asset_id: u64,
    account: String,
    amount: i128,
}

/// Issue amount of asset id
pub async fn issue(
    data: web::Data<AppState>,
    req: web::Json<IssueAssetInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let account = subxt::sp_runtime::MultiAddress::Id(account);
    let currency_id =
        sugarfunge::runtime_types::sugarfunge_primitives::CurrencyId::Id(req.input.asset_id);
    let call = sugarfunge::runtime_types::sugarfunge_runtime::Call::OrmlCurrencies(
        sugarfunge::runtime_types::orml_currencies::module::Call::update_balance {
            who: account,
            currency_id,
            amount: req.input.amount,
        },
    );
    let api = data.api.lock().unwrap();
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
        Some(event) => Ok(HttpResponse::Ok().json(IssueAssetOutput {
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
pub struct AssetIssuanceInput {
    input: AssetIssuanceArg,
}

#[derive(Serialize, Deserialize)]
pub struct AssetIssuanceArg {
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetIssuanceOutput {
    amount: u128,
}

/// Get total issuance for given asset id
pub async fn issuance(
    data: web::Data<AppState>,
    req: web::Json<AssetIssuanceInput>,
) -> error::Result<HttpResponse> {
    let api = data.api.lock().unwrap();
    let currency_id = CurrencyId::Id(req.input.asset_id);
    let result = api
        .storage()
        .orml_assets()
        .total_issuance(currency_id, None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetIssuanceOutput { amount }))
}

#[derive(Serialize, Deserialize)]
pub struct MintAssetInput {
    input: MintAssetArg,
}

#[derive(Serialize, Deserialize)]
pub struct MintAssetArg {
    seed: String,
    account: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct MintAssetOutput {
    account: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

/// Mint amount of asset id to account
pub async fn mint(
    data: web::Data<AppState>,
    req: web::Json<MintAssetInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.input.seed)?;
    let signer = PairSigner::new(pair);
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api
        .tx()
        .asset()
        .mint(
            account,
            req.input.class_id,
            req.input.asset_id,
            req.input.amount,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::asset::events::Mint>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(MintAssetOutput {
            account: event.0.to_string(),
            class_id: event.1,
            asset_id: event.2,
            amount: event.3,
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
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AssetBalanceOutput {
    amount: u128,
}

/// Get balance for given asset id
pub async fn balance(
    data: web::Data<AppState>,
    req: web::Json<AssetBalanceInput>,
) -> error::Result<HttpResponse> {
    let account =
        sp_core::sr25519::Public::from_str(&req.input.account).map_err(map_account_err)?;
    let account = sp_core::crypto::AccountId32::from(account);
    let api = data.api.lock().unwrap();
    let result = api
        .storage()
        .asset()
        .balances(account, (req.input.class_id, req.input.asset_id), None)
        .await;
    let amount = result.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(AssetBalanceOutput { amount }))
}

#[derive(Serialize, Deserialize)]
pub struct TransferAssetInput {
    input: TransferAssetArg,
}

#[derive(Serialize, Deserialize)]
pub struct TransferAssetArg {
    seed: String,
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct TransferAssetOutput {
    from: String,
    to: String,
    class_id: u64,
    asset_id: u64,
    amount: u128,
}

/// Transfer asset from to accounts
pub async fn transfer_from(
    data: web::Data<AppState>,
    req: web::Json<TransferAssetInput>,
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
        .asset()
        .transfer_from(
            account_from,
            account_to,
            req.input.class_id,
            req.input.asset_id,
            req.input.amount,
        )
        .sign_and_submit_then_watch(&signer)
        .await
        .map_err(map_subxt_err)?;
    let result = result
        .find_event::<sugarfunge::asset::events::Transferred>()
        .map_err(map_scale_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(TransferAssetOutput {
            from: event.0.to_string(),
            to: event.1.to_string(),
            class_id: event.2,
            asset_id: event.3,
            amount: event.4,
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::asset::events::Transferred"),
        })),
    }
}
