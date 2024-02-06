use crate::api::v4::common::MsgContent;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgBody {
    #[serde(rename = "MsgType")]
    msg_type: String,

    #[serde(rename = "MsgContent")]
    msg_content: MsgContent,
}

impl MsgBody {
    pub fn new<S: AsRef<str>>(msg_type: S, msg_content: MsgContent) -> Self {
        Self {
            msg_type: msg_type.as_ref().to_string(),
            msg_content,
        }
    }

    pub fn msg_type(&self) -> &str {
        &self.msg_type
    }

    pub fn msg_content(&self) -> &MsgContent {
        &self.msg_content
    }
}
