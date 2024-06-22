mod config;
mod model;
mod repo;
mod service;
mod utils;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value = "show")]
    action: String,
    #[arg(long, short, default_value = "./config.toml")]
    conf: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let cfg = config::Config::init(&args.conf);
    println!("config: {:?}", cfg);
    if args.action == "export" {
        service::trasnlate(cfg).await
    } else if args.action == "show" {
        todo!()
    } else {
        todo!()
    }
}
