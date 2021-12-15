use actix_web::{middleware, web, App, HttpServer};
use command::*;
use state::*;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use subxt::ClientBuilder;

#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
mod account;
mod asset;
mod command;
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
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .data(state.clone())
            .route("account/create", web::post().to(account::create))
            .route("account/fund", web::post().to(account::fund))
            .route("account/balance", web::post().to(account::balance))
            .route("asset/create_class", web::post().to(asset::create_class))
            .route("asset/create", web::post().to(asset::create))
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/balance", web::post().to(asset::balance))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
