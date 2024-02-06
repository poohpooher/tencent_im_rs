use crate::api::v4::common::MsgBody;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RelayMsg {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "MsgSeq")]
    msg_seq: u32,

    #[serde(rename = "MsgRandom")]
    msg_random: u32,

    #[serde(rename = "MsgTimeStamp")]
    msg_time_stamp: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,
}

impl RelayMsg {
    pub fn new<S: AsRef<str>>(from_account: S, group_id: S, msg_seq: u32, msg_random: u32, msg_time_stamp: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            group_id: group_id.as_ref().to_string(),
            msg_seq,
            msg_random,
            msg_time_stamp,
            msg_body,
        }
    }

    pub fn from_account(&self) -> &str {
        &self.from_account
    }

    pub fn group_id(&self) -> &str {
        &self.group_id
    }

    pub fn msg_seq(&self) -> u32 {
        self.msg_seq
    }

    pub fn msg_random(&self) -> u32 {
        self.msg_random
    }

    pub fn msg_time_stamp(&self) -> u32 {
        self.msg_time_stamp
    }

    pub fn msg_body(&self) -> &Vec<MsgBody> {
        &self.msg_body
    }
}
