use crate::api::v4::common::MsgBody;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Msg {
    #[serde(rename = "From_Account")]
    pub from_account: String,

    #[serde(rename = "To_Account")]
    pub to_account: Option<String>,

    #[serde(rename = "SendTime")]
    pub send_time: u32,

    #[serde(rename = "MsgBody")]
    pub msg_body: Vec<MsgBody>,

    #[serde(rename = "Random")]
    pub random: Option<u32>,

    #[serde(rename = "TopicId")]
    pub topic_id: Option<String>,
}

impl Msg {
    pub fn new<S: AsRef<str>>(from_account: S, send_time: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_account: None,
            send_time,
            msg_body,
            random: None,
            topic_id: None,
        }
    }

    pub fn set_to_account<S: AsRef<str>>(&mut self, to_account: Option<S>) -> &mut Self {
        self.to_account = to_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_random(&mut self, random: Option<u32>) -> &mut Self {
        self.random = random;
        self
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}
