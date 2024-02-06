use crate::api::v4::common::SnsProfileTag;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnsProfileItem {
    #[serde(rename = "Tag")]
    tag: String,

    #[serde(rename = "Value")]
    value: SnsProfileTag,
}

impl SnsProfileItem {
    pub fn new(tag: String, value: SnsProfileTag) -> Self {
        SnsProfileItem { tag, value }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn value(&self) -> &SnsProfileTag {
        &self.value
    }
}