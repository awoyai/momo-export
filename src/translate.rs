use crypto::{digest::Digest, md5::Md5};
use rand::Rng;
use reqwest::Result;
use serde::{Deserialize, Serialize};

pub struct Translater {
    cli: reqwest::Client,
    app_id: String,
    app_secret: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct TranslateResp {
    from: String,
    to: String,
    trans_result: Vec<TransResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

    pub async fn translate(&self, text: &str) -> Result<TranslateResp> {
        let (salt, sign) = self.generate_authorization(text);
        let resp = self
            .cli
            .get(format!("http://api.fanyi.baidu.com/api/trans/vip/translate?q={}&from={}&to={}&appid={}&salt={}&sign={}", text, "en", "zh", self.app_id, salt, sign))
            .send()
            .await?;

        Ok(resp.json().await?)
    }

    fn generate_authorization(&self, text: &str) -> (String, String) {
        let mut rng = rand::thread_rng();
        let salt = rng.gen::<u8>();
        let sign_str = format!("{}{}{}{}", self.app_id, text, salt, self.app_secret);
        let mut hasher = Md5::new();
        hasher.input_str(&sign_str);
        (salt.to_string(), hasher.result_str())
    }
}
