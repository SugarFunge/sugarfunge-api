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
use subxt::ClientBuilder;
use sugarfunge_api_types::sugarfunge;

mod account;
mod asset;
mod bundle;
mod command;
mod currency;
mod dex;
mod escrow;
mod market;
mod state;
mod util;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let opt = Opt::from_args();

    let api = ClientBuilder::new()
        .set_url(opt.node_server.to_string())
        .build()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .to_runtime_api::<sugarfunge::RuntimeApi<sugarfunge::DefaultConfig>>();

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
            .route("asset/create", web::post().to(asset::create))
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/burn", web::post().to(asset::burn))
            .route("asset/balance", web::post().to(asset::balance))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
            .route("currency/issue", web::post().to(currency::issue))
            .route("currency/issuance", web::post().to(currency::issuance))
            .route("currency/mint", web::post().to(currency::mint))
            .route("currency/burn", web::post().to(currency::burn))
            .route("currency/supply", web::post().to(currency::supply))
            .route("dex/create", web::post().to(dex::create))
            .route("dex/buy_assets", web::post().to(dex::buy_assets))
            .route("dex/sell_assets", web::post().to(dex::sell_assets))
            .route("dex/add_liquidity", web::post().to(dex::add_liquidity))
            .route(
                "dex/remove_liquidity",
                web::post().to(dex::remove_liquidity),
            )
            .route("escrow/register", web::post().to(escrow::register))
            .route("escrow/create", web::post().to(escrow::create_escrow))
            .route("escrow/sweep", web::post().to(escrow::sweep_assets))
            .route("escrow/deposit", web::post().to(escrow::deposit_assets))
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
