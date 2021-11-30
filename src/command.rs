use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = "sugarfunge-api")]
pub struct Opt {
    #[structopt(
        short = "s",
        long = "node-server",
        default_value = "ws://127.0.0.1:9944"
    )]
    pub node_server: Url,
    #[structopt(short = "l", long = "listen", default_value = "http://127.0.0.1:4000")]
    pub listen: Url,
    #[structopt(short = "d", long = "db-uri")]
    pub db: Option<String>,
}
