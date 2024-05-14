use crypto::{digest::Digest, md5::Md5};
use rand::Rng;
use reqwest::Result;
use serde::{Deserialize, Serialize};

use crate::config;

pub struct Translater {
    cli: reqwest::Client,
    translate_app_cfg: config::TranslateAPP,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TranslateResp {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<TransResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransResult {
    pub src: String,
    pub dst: String,
}

impl Translater {
    pub fn new(translate_app_cfg: config::TranslateAPP) -> Self {
        let cli = reqwest::Client::new();
        Translater {
            cli,
            translate_app_cfg,
        }
    }

    pub async fn translate(&self, text: &str) -> Result<TranslateResp> {
        let (salt, sign) = self.generate_authorization(text);
        let resp = self
            .cli
            .get(format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}", text, "en", "zh", self.translate_app_cfg.app_id, salt, sign))
            .send()
            .await?;

        Ok(resp.json().await?)
    }

    fn generate_authorization(&self, text: &str) -> (String, String) {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<u8>();
        let sign_str = format!("{}{}{}{}", self.translate_app_cfg.app_id, text, salt, self.translate_app_cfg.app_secret);
        let mut hasher = Md5::new();
        hasher.input_str(&sign_str);
        (salt.to_string(), hasher.result_str())
    }
}
