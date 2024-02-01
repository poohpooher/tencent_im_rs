//! <https://www.tencentcloud.com/document/product/1047/50297>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    unban_group_member,
    UnbanGroupMemberRequest,
    UnbanGroupMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnbanGroupMemberRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Members_Account")]
    members_account: Vec<String>,
}

impl UnbanGroupMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, members_account: Vec<S>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            members_account: members_account
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnbanGroupMemberResponse {
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
mod test_unban_group_member {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#aJRGC4MH6",  // ID of the group whose members are to be unbanned
            "Members_Account":["brennanli3", "brennanli12"], // Account IDs of the members to be unbanned. Up to 20 account IDs are supported per request.
        });

        let req = super::UnbanGroupMemberRequest::new(
            "@TGS#aJRGC4MH6",
            vec!["brennanli3", "brennanli12"],
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res =
            serde_json::from_value::<super::UnbanGroupMemberResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
