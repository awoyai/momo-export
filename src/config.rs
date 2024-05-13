use serde::{Deserialize, Serialize};
#[derive(Deserialize, Clone, Serialize)]
pub struct Config {
    app_id: String,
    app_secret: String,
}
