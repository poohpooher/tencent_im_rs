use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAttribute {
    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "Attrs")]
    pub attrs: HashMap<String, String>,
}

impl UserAttribute {
    pub fn new<S: AsRef<str>>(to_account: S, attrs: HashMap<S, S>) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            attrs: attrs
                .iter()
                .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
                .collect(),
        }
    }

    pub fn to_account(&self) -> &str {
        &self.to_account
    }

    pub fn attrs(&self) -> &HashMap<String, String> {
        &self.attrs
    }
}
