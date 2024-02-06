use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCounter {
    #[serde(rename = "Key")]
    key: String,

    #[serde(rename = "Value")]
    value: u32,
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

    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value = value;
        self
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
