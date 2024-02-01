use crate::api::v4::common::SnsProfileTag;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnsProfileItem {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Value")]
    pub value: SnsProfileTag,
}
