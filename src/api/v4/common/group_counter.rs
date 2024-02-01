use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCounter {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "Value")]
    pub value: u32,
}

impl GroupCounter {
    pub fn new<S: AsRef<str>>(key: S, value: u32) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value,
        }
    }

    pub fn set_key<S: AsRef<str>>(&mut self, key: S) -> &mut Self {
        self.key = key.as_ref().to_string();
        self
    }
}
