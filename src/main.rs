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
}

#[tokio::main]
async fn main() {
    let cfg = config::Config::init();
    println!("config: {:?}", cfg);
    let args: Args = Args::parse();
    if args.action == "export" {
        service::trasnlate(cfg).await
    } else if args.action == "show" {
        todo!()
    } else {
        todo!()
    }
}
