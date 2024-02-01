//! <https://www.tencentcloud.com/ko/document/product/1047/34951>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_joined_group_list,
    GetJoinedGroupListRequest,
    GetJoinedGroupListResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetJoinedGroupListRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Members_Account")]
    members_account: Vec<String>,

    #[serde(rename = "MuteTime")]
    mute_time: u32,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl GetJoinedGroupListRequest {
    pub fn new<S: AsRef<str>>(group_id: S, members_account: Vec<S>, mute_time: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            members_account: members_account
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
            mute_time,
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
pub struct GetJoinedGroupListResponse {
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
mod test_forbid_send_msg {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Members_Account": [ // Up to 500 member accounts
              "peter",
              "leckie"
          ],
          "MuteTime": 60 // Muting period, in seconds
        });

        let req =
            super::GetJoinedGroupListRequest::new("@TGS#2C5SZEAEF", vec!["peter", "leckie"], 60);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Members_Account": [ // Up to 500 member accounts
              "peter",
              "leckie"
          ],
          "MuteTime": 0 // 0 indicates to unmute members.
        });

        let req =
            super::GetJoinedGroupListRequest::new("@TGS#2C5SZEAEF", vec!["peter", "leckie"], 0);

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
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
