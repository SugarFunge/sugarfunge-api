use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use command::*;
use state::*;
use std::sync::Arc;
use structopt::StructOpt;
use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};
use sugarfunge_api_types::sugarfunge;
use util::url_to_string;

mod account;
mod asset;
mod bag;
mod bundle;
mod command;
mod market;
mod state;
#[cfg(feature = "keycloak")]
mod user;
mod util;
mod validator;

#[cfg(feature = "keycloak")]
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
#[cfg(feature = "keycloak")]
use dotenv::dotenv;
#[cfg(feature = "keycloak")]
use sugarfunge_api_types::config;

#[cfg(not(feature = "keycloak"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let opt = Opt::from_args();

    let api = ClientBuilder::new()
        .set_url(url_to_string(opt.node_server))
        .build()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .to_runtime_api::<sugarfunge::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let state = AppState { api: Arc::new(api) };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(state.clone()))
            .route("account/seeded", web::post().to(account::seeded))
            .route("account/exists", web::post().to(account::exists))
            .route("account/create", web::post().to(account::create))
            .route("account/fund", web::post().to(account::fund))
            .route("account/balance", web::post().to(account::balance))
            .route("asset/create_class", web::post().to(asset::create_class))
            .route("asset/class_info", web::post().to(asset::class_info))
            .route("asset/create", web::post().to(asset::create))
            .route("asset/info", web::post().to(asset::info))
            .route(
                "asset/update_metadata",
                web::post().to(asset::update_metadata),
            )
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/burn", web::post().to(asset::burn))
            .route("asset/balance", web::post().to(asset::balance))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
            .route("bag/register", web::post().to(bag::register))
            .route("bag/create", web::post().to(bag::create))
            .route("bag/sweep", web::post().to(bag::sweep))
            .route("bag/deposit", web::post().to(bag::deposit))
            .route("bundle/register", web::post().to(bundle::register_bundle))
            .route("bundle/mint", web::post().to(bundle::mint_bundle))
            .route("bundle/burn", web::post().to(bundle::burn_bundle))
            .route(
                "validator/add_validator",
                web::post().to(validator::add_validator),
            )
            .route(
                "validator/remove_validator",
                web::post().to(validator::remove_validator),
            )
            .route(
                "market/create_market",
                web::post().to(market::create_market),
            )
            .route(
                "market/create_market_rate",
                web::post().to(market::create_market_rate),
            )
            .route(
                "market/deposit_assets",
                web::post().to(market::deposit_assets),
            )
            .route(
                "market/exchange_assets",
                web::post().to(market::exchange_assets),
            )
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}

#[cfg(feature = "keycloak")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let env = config::init();

    let keycloak_pk = format!(
        "-----BEGIN PUBLIC KEY----- {} -----END PUBLIC KEY-----",
        env.keycloak_public_key
    );

    let opt = Opt::from_args();

    let api = ClientBuilder::new()
        .set_url(url_to_string(opt.node_server))
        .build()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .to_runtime_api::<sugarfunge::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    let state = AppState { api: Arc::new(api) };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let keycloak_auth = KeycloakAuth {
            detailed_responses: true,
            passthrough_policy: AlwaysReturnPolicy,
            keycloak_oid_public_key: DecodingKey::from_rsa_pem(keycloak_pk.as_bytes())
                .unwrap()
                .into_static(),
            required_roles: vec![],
        };

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .app_data(Data::new(env.clone()))
            .wrap(keycloak_auth)
            .wrap(cors)
            .route("account/seeded", web::post().to(account::seeded))
            .route("account/exists", web::post().to(account::exists))
            .route("account/fund", web::post().to(account::fund))
            .route("account/balance", web::post().to(account::balance))
            .route("asset/create_class", web::post().to(asset::create_class))
            .route("asset/class_info", web::post().to(asset::class_info))
            .route("asset/create", web::post().to(asset::create))
            .route("asset/info", web::post().to(asset::info))
            .route(
                "asset/update_metadata",
                web::post().to(asset::update_metadata),
            )
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/burn", web::post().to(asset::burn))
            .route("asset/balance", web::post().to(asset::balance))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
            .route("bag/register", web::post().to(bag::register))
            .route("bag/create", web::post().to(bag::create))
            .route("bag/sweep", web::post().to(bag::sweep))
            .route("bag/deposit", web::post().to(bag::deposit))
            .route("bundle/register", web::post().to(bundle::register_bundle))
            .route("bundle/mint", web::post().to(bundle::mint_bundle))
            .route("bundle/burn", web::post().to(bundle::burn_bundle))
            .route("user/verify_seed", web::get().to(user::verify_seed))
            .route(
                "validator/add_validator",
                web::post().to(validator::add_validator),
            )
            .route(
                "validator/remove_validator",
                web::post().to(validator::remove_validator),
            )
            .route(
                "market/create_market",
                web::post().to(market::create_market),
            )
            .route(
                "market/create_market_rate",
                web::post().to(market::create_market_rate),
            )
            .route(
                "market/deposit_assets",
                web::post().to(market::deposit_assets),
            )
            .route(
                "market/exchange_assets",
                web::post().to(market::exchange_assets),
            )
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
