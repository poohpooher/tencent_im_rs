//! <https://www.tencentcloud.com/ko/document/product/1047/34964>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_group_muted_account,
    GetGroupMutedAccountRequest,
    GetGroupMutedAccountResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupMutedAccountRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl GetGroupMutedAccountRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
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
pub struct GetGroupMutedAccountResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,

    #[serde(rename = "MutedAccountList")]
    pub muted_account_list: Option<Vec<MutedAccount>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MutedAccount {
    #[serde(rename = "Member_Account")]
    pub member_account: String,

    #[serde(rename = "MutedUntil")]
    pub muted_until: u32,
}

#[cfg(test)]
mod test_get_group_muted_account {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
             "GroupId":"@TGS#1KGZ2RAEU"
        });

        let req = super::GetGroupMutedAccountRequest::new("@TGS#1KGZ2RAEU");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "GroupId": "@TGS#2FZNNRAEU",
            "MutedAccountList": [ // List of muted users in the group
                {
                    "Member_Account": "tommy", // User ID
                    "MutedUntil": 1458115189 // Muting stop time (UTC - Coordinated Universal Time)
                },
                {
                    "Member_Account": "peter",
                    "MutedUntil": 1458115189
                }
            ]
        });

        let res =
            serde_json::from_value::<super::GetGroupMutedAccountResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
