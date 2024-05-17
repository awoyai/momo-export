use std::{fs::File, io::Read};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value = "./config.toml")]
    conf: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub db: DB,
    pub translate_app: TranslateAPP,
    pub log: Log,
    pub book_list: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranslateAPP {
    pub platform: String,
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DB {
    pub file_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    /// `log_level` 日志输出等级
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `file` 日志输出文件名
    pub file: String,
    /// 允许操作日志输出
    pub enable_oper_log: bool,
}

impl Config {
    pub fn init() -> Self {
        let args: Args = Args::parse();
        println!("配置文件路径{}", &args.conf);
        let mut file = match File::open(&args.conf) {
            Ok(f) => f,
            Err(e) => panic!("不存在该文件：{}, 错误信息: {}", &args.conf, e),
        };
        let mut cfg_contens = String::new();
        match file.read_to_string(&mut cfg_contens) {
            Ok(s) => s,
            Err(e) => panic!("读取文件失败, 错误信息{}", e),
        };
        toml::from_str(&cfg_contens).expect("解析文件错误")
    }
}
