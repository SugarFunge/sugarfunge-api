use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
    get, http, HttpRequest, HttpResponse,
};
use command::*;
use state::*;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use subxt::ClientBuilder;

#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
mod account;
mod asset;
mod bundle;
mod command;
mod currency;
mod dex;
mod escrow;
mod state;
mod util;

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

    let state = AppState {
        api: Arc::new(Mutex::new(api)),
    };

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
            
            //.wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            //.wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
            //.wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With, Origin, Accept")))
            //.wrap(middleware::DefaultHeaders::new().add(("Access-Control-Allow-Methods", "GET, OPTIONS, POST, PUT")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(Data::new(state.clone()))
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
            .route("currency/balance", web::post().to(currency::balance))
            .route("dex/create", web::post().to(dex::create))
            .route("dex/buy_assets", web::post().to(dex::buy_assets))
            .route("dex/sell_assets", web::post().to(dex::sell_assets))
            .route("dex/add_liquidity", web::post().to(dex::add_liquidity))
            .route(
                "dex/remove_liquidity",
                web::post().to(dex::remove_liquidity),
            )
            .route("escrow/create", web::post().to(escrow::create_escrow))
            .route("escrow/refund", web::post().to(escrow::refund_assets))
            .route("escrow/deposit", web::post().to(escrow::deposit_assets))
            .route("bundle/register", web::post().to(bundle::register_bundle))
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
