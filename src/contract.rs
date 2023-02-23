use crate::util::*;
use actix_web::{error, web, HttpResponse};
use contract_integration::admin_calls::*;
use serde_json::json;
use sp_core::U256;
use sugarfunge_api_types::contract::*;
use sugarfunge_api_types::primitives::*;

pub async fn contract_mint_to(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = mint_to(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::MintTo"),
            description: format!(""),
        })),
    }
}

pub async fn contract_burn_from(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = burn_from(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::BurnFrom"),
            description: format!(""),
        })),
    }
}

pub async fn contract_transfer(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = transfer(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Transfer"),
            description: format!(""),
        })),
    }
}

pub async fn contract_total_supply() -> error::Result<HttpResponse> {
    let result = total_supply().await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractTotalSupplyOutput {
            total_supply: remove_decimals_from_u256(event.total_supply, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::TotalSupply"),
            description: format!(""),
        })),
    }
}

pub async fn contract_allowance(
    req: web::Json<ContractAllowanceInput>,
) -> error::Result<HttpResponse> {
    let result = allowance(req.owner_address.as_str(), req.spender_address.as_str()).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractAllowanceOutput {
            allowance: remove_decimals_from_u256(event.allowance, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Allowance"),
            description: format!(""),
        })),
    }
}

pub async fn contract_increase_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = increase_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::IncreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn contract_decrease_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = decrease_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::DecreaseAllowance"),
            description: format!(""),
        })),
    }
}
