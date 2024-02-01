//! <https://www.tencentcloud.com/document/product/1047/50296>

use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    ban_group_member,
    BanGroupMemberRequest,
    BanGroupMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BanGroupMemberRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Members_Account")]
    members_account: Vec<String>,

    #[serde(rename = "Duration")]
    duration: u32,

    #[serde(rename = "Description")]
    description: Option<String>,
}

impl BanGroupMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, members_account: Vec<String>, duration: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            members_account,
            duration,
            description: None,
        }
    }

    pub fn set_description<S: AsRef<str>>(&mut self, description: Option<S>) -> &mut Self {
        self.description = description.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BanGroupMemberResponse {
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
mod test_ban_group_member {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#aJRGC4MH6",  // ID of the group whose members are to be banned
            "Members_Account":["brennanli3", "brennanli12"], // Account IDs of the members to be banned. Up to 20 account IDs are supported per request.
            "Duration":3600, // Ban duration, in seconds
            "Description": "you are banned because of irregularities" // Ban information, which can be up to 1,000 bytes in length
        });

        let mut req = super::BanGroupMemberRequest::new(
            "@TGS#aJRGC4MH6",
            vec!["brennanli3".to_string(), "brennanli12".to_string()],
            3600,
        );
        req.set_description(Some("you are banned because of irregularities"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res = serde_json::from_value::<super::BanGroupMemberResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
