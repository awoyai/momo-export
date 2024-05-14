use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vocabulary {
    pub title: String,
    pub spelling: String,
    pub phonetic_uk: String,
}

impl Vocabulary {
    pub fn into_slice(self) -> Vec<String> {
        vec![self.title, self.spelling, self.phonetic_uk]
    }
}
