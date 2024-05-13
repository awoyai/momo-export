use rand::Rng;
use reqwest::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Translater {
    cli: reqwest::Client,
    app_id: String,
    app_secret: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct TranslateResp {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
}

#[derive(Clone, Deserialize, Serialize)]
struct TransResult {
    src: String,
    dst: String,
}

impl Translater {
    pub fn new(app_id: String, app_secret: String) -> Self {
        let cli = reqwest::Client::new();
        Translater {
            cli,
            app_id,
            app_secret,
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String> {
        let (salt, sign) = self.generate_authorization(text);
        let mut data = HashMap::new();
        data.insert("q", text);
        data.insert("from", "en");
        data.insert("to", "zh");
        data.insert("appid", &self.app_id);
        data.insert("salt", &salt);
        data.insert("sign", &sign);

        Ok(String::new())
    }

    fn generate_authorization(&self, text: &str) -> (String, String) {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<u8>();
        let signStr = format!("{}{}{}{}", self.app_id, text, salt, self.app_secret);
        (salt.to_string(), String::new())
    }
}
