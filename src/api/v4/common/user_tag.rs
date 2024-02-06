use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTag {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "Tags")]
    tags: Vec<String>,
}

impl UserTag {
    pub fn new<S: AsRef<str>>(to_account: S, tags: Vec<S>) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            tags: tags.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }

    pub fn to_account(&self) -> &str {
        &self.to_account
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}
