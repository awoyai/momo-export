mod repo;
mod config;
mod utils;
mod model;

#[tokio::main]
async fn main() {
    let cfg = config::Config::init();
    let translater = repo::Translater::new(cfg.translate_app);
    match translater.translate("hello world").await {
        Ok(res) => println!("result: {:?}", res.trans_result),
        Err(err) => println!("err: {}", err)
    }
}
