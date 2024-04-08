use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
use args::*;
use clap::Parser;
use state::*;
use std::sync::Arc;
use subxt::{client::OnlineClient, PolkadotConfig};
use util::url_to_string;

mod account;
mod args;
mod asset;
mod bag;
mod bundle;
mod market;
mod state;
mod subscription;
mod util;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();

    let api = OnlineClient::<PolkadotConfig>::from_url(url_to_string(args.node_server))
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
            .service(web::resource("/ws").route(web::get().to(subscription::ws)))
            // .route("/ws", web::get().to(subscription::ws))
            .route("account/seeded", web::post().to(account::seeded))
            .route("account/exists", web::post().to(account::exists))
            .route("account/create", web::post().to(account::create))
            .route("account/fund", web::post().to(account::fund))
            .route("account/set_balance", web::post().to(account::set_balance))
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
            .route("bundle", web::post().to(bundle::get_bundles_id))
            .route("bundle/data", web::post().to(bundle::get_bundles_data))
            .route(
                "validator/add_validator",
                web::post().to(validator::add_validator),
            )
            .route(
                "validator/remove_validator",
                web::post().to(validator::remove_validator),
            )
            .route("validator/set_keys", web::post().to(validator::set_keys))
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
    .bind((args.listen.host_str().unwrap(), args.listen.port().unwrap()))?
    .run()
    .await
}
