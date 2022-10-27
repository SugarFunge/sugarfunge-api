use actix_web::{error, web, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::Pair;
use subxt::rpc::Health;
use sugarfunge_api_types::primitives::*;
use url::Url;

use crate::state::AppState;

#[derive(Serialize, Deserialize, Debug, Display)]
#[display(fmt = "{:?} {:?}", message, description)]
pub struct RequestError {
    pub message: serde_json::Value,
    pub description: String,
}

pub fn map_subxt_err(e: subxt::Error) -> actix_web::Error {
    // TODO: json_err should be a json Value to improve UX
    let json_err = json!(e.to_string());
    let req_error = RequestError {
        message: json_err,
        description: "Subxt error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_sf_err(
    e: subxt::Error,
) -> actix_web::Error {
    // TODO: json_err should be a json Value to improve UX
    let json_err = json!(e.to_string());
    let req_error = RequestError {
        message: json_err,
        description: "Sugarfunge error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_fula_err(
    e: subxt::Error,
) -> actix_web::Error {

    let json_err = json!(e.to_string().replace("\"", ""));
    let req_error = RequestError {
        message: json_err,
        description: "Fula Pallet error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn map_account_err(e: sp_core::crypto::PublicError) -> actix_web::Error {
    let json_err: serde_json::Value = json!("Invalid account");
    let req_error = RequestError {
        message: json_err,
        description: format!("{:?}", e),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}

pub fn get_pair_from_seed(seed: &Seed) -> error::Result<sp_core::sr25519::Pair> {
    sp_core::sr25519::Pair::from_string(seed.as_str(), None).map_err(|e| {
        let req_error = RequestError {
            message: json!(&format!("{:?}", e)),
            description: "API error".into(),
        };
        let req_error = serde_json::to_string_pretty(&req_error).unwrap();
        error::ErrorBadRequest(req_error)
    })
}

pub fn url_to_string(url: Url) -> String {
    let mut res = url.to_string();
    match (url.port(), url.port_or_known_default()) {
        (None, Some(port)) => {
            res.insert_str(res.len() - 1, &format!(":{}", port));
            res
        }
        _ => res,
    }
}

pub async fn health_check(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let health: Health = api
        .rpc()
        .system_health()
        .await
        .map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(health))
}
