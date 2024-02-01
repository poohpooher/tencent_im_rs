//! <https://www.tencentcloud.com/ko/document/product/1047/34958>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    send_group_system_notification,
    SendGroupSystemNotificationRequest,
    SendGroupSystemNotificationResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendGroupSystemNotificationRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "ToMembers_Account")]
    to_members_account: Option<Vec<String>>,

    #[serde(rename = "Content")]
    content: String,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl SendGroupSystemNotificationRequest {
    pub fn new<S: AsRef<str>>(group_id: S, content: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            to_members_account: None,
            content: content.as_ref().to_string(),
            topic_id: None,
        }
    }

    pub fn set_to_members_account<S: AsRef<str>>(
        &mut self,
        to_members_account: Option<Vec<S>>,
    ) -> &mut Self {
        self.to_members_account =
            to_members_account.map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendGroupSystemNotificationResponse {
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
mod test_send_group_system_notification {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Content": "Hello World" // Content of the notification
        });

        let req = super::SendGroupSystemNotificationRequest::new("@TGS#2C5SZEAEF", "Hello World");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "ToMembers_Account": [ // List of recipients. You can leave this field empty to send the notification to all members.
              "peter",
              "leckie"
          ],
          "Content": "Hello World" // Content of the notification
        });

        let mut req =
            super::SendGroupSystemNotificationRequest::new("@TGS#2C5SZEAEF", "Hello World");
        req.set_to_members_account(Some(vec!["peter", "leckie"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res =
            serde_json::from_value::<super::SendGroupSystemNotificationResponse>(sample.clone())
                .unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
