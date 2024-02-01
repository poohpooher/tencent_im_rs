//! <https://www.tencentcloud.com/ko/document/product/1047/34963>

use crate::api::v4::common::member_role::MemberRole;
use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_role_in_group,
    GetRoleInGroupRequest,
    GetRoleInGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRoleInGroupRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "User_Account")]
    user_account: Vec<String>,
}

impl GetRoleInGroupRequest {
    pub fn new<S: AsRef<str>>(group_id: S, user_account: Vec<S>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            user_account: user_account
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRoleInGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UserIdList")]
    pub user_id_list: Vec<UserRole>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRole {
    #[serde(rename = "Member_Account")]
    pub member_account: String,

    #[serde(rename = "Role")]
    pub role: MemberRole,
}

#[cfg(test)]
mod test_get_role_in_group {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2C5SZEAEF",
            "User_Account": [ // Up to 500 member accounts are supported
                "leckie",
                "peter",
                "wesley"
            ]
        });

        let req =
            super::GetRoleInGroupRequest::new("@TGS#2C5SZEAEF", vec!["leckie", "peter", "wesley"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "UserIdList": [ // Result
                {
                    "Member_Account": "leckie",
                    "Role": "Owner" // Member role: Owner/Admin/Member/NotMember
                },
                {
                    "Member_Account": "peter",
                    "Role": "Member"
                },
                {
                    "Member_Account": "wesley",
                    "Role": "NotMember"
                }
            ]
        });

        let res = serde_json::from_value::<super::GetRoleInGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
