use actix_web::error;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::Pair;

#[derive(Serialize, Deserialize, Debug, Display)]
pub struct RequestError {
    pub message: serde_json::Value,
}

pub fn map_subxt_err(e: subxt::Error) -> actix_web::Error {
    let json_err: serde_json::Value = match e {
        subxt::Error::Rpc(rpc) => match rpc {
            jsonrpsee_types::error::Error::Request(err) => {
                serde_json::from_str(&err.to_string()).unwrap_or(json!("RPC error"))
            }
            _ => json!(rpc.to_string()),
        },
        _ => json!("default".to_string()),
    };
    let req_error = RequestError { message: json_err };
    error::ErrorBadRequest(req_error)
}

pub fn get_pair_from_seed(seed: &str) -> error::Result<sp_core::sr25519::Pair> {
    sp_core::sr25519::Pair::from_string(&seed, None).map_err(|e| {
        let req_error = RequestError {
            message: json!(&format!("{:?}", e)),
        };
        let req_error = serde_json::to_string(&req_error).unwrap();
        error::ErrorBadRequest(req_error)
    })
}
