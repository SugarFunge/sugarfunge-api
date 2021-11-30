use actix_web::{middleware, web, App, HttpServer};
use command::*;
use state::*;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use subxt::ClientBuilder;

#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
mod account;
mod command;
mod nft;
mod state;
mod token;
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
            .route("token/issue", web::post().to(token::issue))
            .route("token/issuance", web::post().to(token::issuance))
            .route("token/mint", web::post().to(token::mint))
            .route("token/balance", web::post().to(token::balance))
            .route("token/transfer_from", web::post().to(token::transfer_from))
            .route("nft/create", web::post().to(nft::create))
            .route("nft/mint", web::post().to(nft::mint))
            .route("nft/collections", web::post().to(nft::collections))
            .route("nft/tokens", web::post().to(nft::tokens))
            .route("nft/owner", web::post().to(nft::owner))
            .route("nft/transfer", web::post().to(nft::transfer))
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .run()
    .await
}
