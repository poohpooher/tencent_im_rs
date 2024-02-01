//! <https://www.tencentcloud.com/ko/document/product/1047/34949>
use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    delete_group_member,
    DeleteGroupMemberRequest,
    DeleteGroupMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteGroupMemberRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Silence")]
    silence: Option<u32>,

    #[serde(rename = "Reason")]
    reason: Option<String>,

    #[serde(rename = "MemberToDel_Account")]
    delete_accounts: Vec<String>,
}

impl DeleteGroupMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, delete_accounts: Vec<S>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            silence: None,
            reason: None,
            delete_accounts: delete_accounts
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        }
    }

    pub fn set_silence(&mut self, silence: Option<u32>) -> &mut Self {
        self.silence = silence;
        self
    }

    pub fn set_reason<S: AsRef<str>>(&mut self, reason: Option<S>) -> &mut Self {
        self.reason = reason.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteGroupMemberResponse {
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
mod test_delete_group_member {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // Target group (required)
          "MemberToDel_Account": [ // List of group members to be deleted. A maximum of 100 members can be deleted.
              "tommy",
              "jared"
          ]
        });

        let req = super::DeleteGroupMemberRequest::new("@TGS#2J4SZEAEL", vec!["tommy", "jared"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // Target group (required)
          "Silence": 1, // Whether to delete members silently (optional)
          "MemberToDel_Account": [ // List of group members to be deleted. A maximum of 100 members can be deleted.
              "tommy",
              "jared"
          ]
        });

        let mut req =
            super::DeleteGroupMemberRequest::new("@TGS#2J4SZEAEL", vec!["tommy", "jared"]);
        req.set_silence(Some(1));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // Target group (required)
          "Reason": "kick reason", // Reason for deleting a user from a group (optional)
          "MemberToDel_Account": [ // List of group members to be deleted. A maximum of 100 members can be deleted.
              "tommy",
              "jared"
          ]
        });

        let mut req =
            super::DeleteGroupMemberRequest::new("@TGS#2J4SZEAEL", vec!["tommy", "jared"]);
        req.set_reason(Some("kick reason"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0
        });

        let res =
            serde_json::from_value::<super::DeleteGroupMemberResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
