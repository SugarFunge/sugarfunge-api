use crate::state::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use serde_json::json;
use subxt::tx::PairSigner;
use sugarfunge_api_types::fula::*;
use sugarfunge_api_types::sugarfunge;
use sugarfunge_api_types::sugarfunge::runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec;

pub async fn update_manifest(
    data: web::Data<AppState>,
    req: web::Json<UpdateManifestInput>,
) -> error::Result<HttpResponse> {
    let pair = get_pair_from_seed(&req.seed)?;
    let signer = PairSigner::new(pair);
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let manifest: Vec<u8> = serde_json::to_vec(&req.manifest).unwrap_or_default();
    let manifest = BoundedVec(manifest);
    let api = &data.api;

    let call = sugarfunge::tx().fula().update_manifest(account_to, manifest);

    let result = api
        .tx()
        .sign_and_submit_then_watch(&call, &signer, Default::default())
        .await
        .map_err(map_subxt_err)?
        .wait_for_finalized_success()
        .await
        .map_err(map_sf_err)?;
    let result = result
        .find_first::<sugarfunge::fula::events::ManifestUpdated>()
        .map_err(map_subxt_err)?;
    match result {
        Some(event) => Ok(HttpResponse::Ok().json(UpdateManifestOutput {
            from: event.from.into(),
            to: event.to.into(),
            manifest: serde_json::from_slice(event.manifest.as_slice()).unwrap_or_default(),
        })),
        None => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to find sugarfunge::bundle::events::Register"),
            description: format!(""),
        })),
    }
}

pub async fn manifest(
    data: web::Data<AppState>,
    req: web::Json<ManifestsInput>,
) -> error::Result<HttpResponse> {
    let account_from =
        sp_core::crypto::AccountId32::try_from(&req.from).map_err(map_account_err)?;
    let account_to = sp_core::crypto::AccountId32::try_from(&req.to).map_err(map_account_err)?;
    let api = &data.api;

    let call = sugarfunge::storage().fula().manifests(&account_to, &account_from);

    let result = api.storage().fetch(&call, None).await;

    let manifest = result.map_err(map_subxt_err)?;

    if let Some(manifest) = manifest {
        if let Ok(manifest) = serde_json::from_slice(manifest.manifest.0.as_slice()) {
            Ok(HttpResponse::Ok().json(ManifestsOutput {
                manifests: vec![Manifest {
                    from: account_from.into(),
                    to: account_to.into(),
                    manifest: manifest,
                }],
            }))
        } else {
            Ok(HttpResponse::BadRequest().json(RequestError {
                message: json!("Manifest no JSON."),
                description: format!(""),
            }))
        }
    } else {
        Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("No manifest."),
            description: format!(""),
        }))
    }
}