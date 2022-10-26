use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use command::*;
use state::*;
use util::url_to_string;
use std::sync::Arc;
use structopt::StructOpt;
use subxt::{client::OnlineClient, PolkadotConfig};

mod account;
mod asset;
mod bag;
mod bundle;
mod command;
mod fula;
mod market;
mod state;
mod util;
mod validator;
mod subscription;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let opt = Opt::from_args();

    let api = OnlineClient::<PolkadotConfig>::from_url(url_to_string(opt.node_server))
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

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
            .route("health", web::post().to(util::health_check))
            .service(web::resource("/ws").route(web::get().to(subscription::ws)))
            // .route("/ws", web::get().to(subscription::ws))
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
            .route("asset/balances", web::post().to(asset::balances))
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
            .route("fula/manifest/update",web::post().to(fula::update_manifest),)
            .route("fula/manifest", web::post().to(fula::get_all_manifests))
            .route("fula/manifest/remove", web::post().to(fula::remove_manifest))
            .route("fula/manifest/remove_storer", web::post().to(fula::remove_storer))
            .route("fula/manifest/remove_storing_manifest", web::post().to(fula::remove_storing_manifest))
            .route("fula/manifest/upload", web::post().to(fula::upload_manifest))
            .route("fula/manifest/available", web::get().to(fula::get_available_manifests))
            .route("fula/manifest/storage",web::post().to(fula::storage_manifest))
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
