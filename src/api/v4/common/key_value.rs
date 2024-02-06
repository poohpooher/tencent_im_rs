use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePascal {
    #[serde(rename = "Key")]
    key: String,

    #[serde(rename = "Value")]
    value: String,
}

impl KeyValuePascal {
    pub fn new<S: AsRef<str>>(key: S, value: S) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyValueLower {
    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "value")]
    value: String,
}

impl KeyValueLower {
    pub fn new<S: AsRef<str>>(key: S, value: S) -> Self {
        Self {
            key: key.as_ref().to_string(),
            value: value.as_ref().to_string(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
