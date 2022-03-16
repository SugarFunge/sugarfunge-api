use actix_web::error;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::Pair;
use sugarfunge_api_types::primitives::*;

#[derive(Serialize, Deserialize, Debug, Display)]
pub struct RequestError {
    pub message: serde_json::Value,
}

pub fn map_subxt_err(e: subxt::Error) -> actix_web::Error {
    let json_err: serde_json::Value = match e {
        subxt::Error::Rpc(rpc) => match rpc {
            jsonrpsee_types::error::Error::Request(e) => {
                serde_json::from_str(&e).unwrap_or(json!(&e))
            }
            _ => json!(rpc.to_string()),
        },
        subxt::Error::Runtime(e) => match e {
            subxt::RuntimeError::Module(subxt::PalletError {
                pallet: _,
                error: e,
                description: _,
            }) => json!(e),
            _ => json!(e.to_string()),
        },
        _ => json!(e.to_string()),
    };
    let req_error = RequestError { message: json_err };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_account_err(_e: sp_core::crypto::PublicError) -> actix_web::Error {
    let json_err: serde_json::Value = json!("Invalid account");
    let req_error = RequestError { message: json_err };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn get_pair_from_seed(seed: &Seed) -> error::Result<sp_core::sr25519::Pair> {
    sp_core::sr25519::Pair::from_string(seed.as_str(), None).map_err(|e| {
        let req_error = RequestError {
            message: json!(&format!("{:?}", e)),
        };
        let req_error = serde_json::to_string_pretty(&req_error).unwrap();
        error::ErrorBadRequest(req_error)
    })
}
