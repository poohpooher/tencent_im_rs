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
