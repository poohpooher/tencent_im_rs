//! <https://www.tencentcloud.com/ko/document/product/1047/34970>

use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    delete_group_msg_by_sender,
    DeleteGroupMsgBySenderRequest,
    DeleteGroupMsgBySenderResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteGroupMsgBySenderRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Sender_Account")]
    sender_account: String,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl DeleteGroupMsgBySenderRequest {
    pub fn new<S: AsRef<str>>(group_id: S, sender_account: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            sender_account: sender_account.as_ref().to_string(),
            topic_id: None,
        }
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteGroupMsgBySenderResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,
}

#[cfg(test)]
mod test_delete_group_msg_by_sender {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2C5SZEAEF",    // Required
            "Sender_Account": "leckie"      // Required
        });

        let req = super::DeleteGroupMsgBySenderRequest::new("@TGS#2C5SZEAEF", "leckie");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res = serde_json::from_value::<super::DeleteGroupMsgBySenderResponse>(sample.clone())
            .unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
