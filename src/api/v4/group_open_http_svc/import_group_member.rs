//! <https://www.tencentcloud.com/ko/document/product/1047/34969>

use crate::api::v4::common::member_role::MemberRole;
use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

tencent_api!(
    import_group_member,
    ImportGroupMemberRequest,
    ImportGroupMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupMemberRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "MemberList")]
    member_list: Vec<ImportGroupMember>,
}

impl ImportGroupMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, member_list: Vec<ImportGroupMember>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            member_list,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupMember {
    #[serde(rename = "Member_Account")]
    member_account: String,

    #[serde(rename = "Role")]
    role: Option<MemberRole>,

    #[serde(rename = "JoinTime")]
    join_time: Option<u32>,

    #[serde(rename = "UnreadMsgNum")]
    unread_msg_num: Option<u32>,

    #[serde(rename = "Result")]
    result: Option<ImportResult>,
}

impl ImportGroupMember {
    pub fn new<S: AsRef<str>>(member_account: S) -> Self {
        Self {
            member_account: member_account.as_ref().to_string(),
            role: None,
            join_time: None,
            unread_msg_num: None,
            result: None,
        }
    }

    pub fn set_role(&mut self, role: Option<MemberRole>) -> &mut Self {
        self.role = role;
        self
    }

    pub fn set_join_time(&mut self, join_time: Option<u32>) -> &mut Self {
        self.join_time = join_time;
        self
    }

    pub fn set_unread_msg_num(&mut self, unread_msg_num: Option<u32>) -> &mut Self {
        self.unread_msg_num = unread_msg_num;
        self
    }

    pub fn set_result(&mut self, result: Option<ImportResult>) -> &mut Self {
        self.result = result;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ImportResult {
    Success = 0,
    Failed = 1,
    AlreadyGroupMember = 2,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupMemberResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "MemberList")]
    pub member_list: Option<Vec<ImportGroupMember>>,
}

#[cfg(test)]
mod test_import_group_member {
    use crate::api::v4::common::member_role::MemberRole;
    use crate::api::v4::group_open_http_svc::import_group_member::ImportGroupMember;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2J4SZEAEL", // (Required) Target group
            "MemberList": [ // Up to 300 members can be added at a time.
            {
                "Member_Account": "tommy", // (Required) ID of the member to be added
                "Role":"Admin", // (Optional) Role of the group member to be imported. Currently, the only supported role is Admin.
                "JoinTime":1448357837, // (Optional) Time when the group member to be imported joined the group
                "UnreadMsgNum":5          // (Optional) Unread message count of the member

            },
            {
                "Member_Account": "jared",
                "JoinTime":1448357857,
                "UnreadMsgNum":2
            }]
        });

        let mut member_list: Vec<ImportGroupMember> = Vec::new();
        let mut import_member = ImportGroupMember::new("tommy");
        import_member
            .set_role(Some(MemberRole::Admin))
            .set_join_time(Some(1448357837))
            .set_unread_msg_num(Some(5));
        member_list.push(import_member);

        let mut import_member = ImportGroupMember::new("jared");
        import_member
            .set_join_time(Some(1448357857))
            .set_unread_msg_num(Some(2));
        member_list.push(import_member);

        let req = super::ImportGroupMemberRequest::new("@TGS#2J4SZEAEL", member_list);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "MemberList": [
            {
                 "Member_Account": "tommy",
                 "Result": 1 // Result of the import. 0: Failed; 1: Succeeded; 2: Already a group member
            },
            {
                 "Member_Account": "jared",
                 "Result": 1
            }]
        });

        let res =
            serde_json::from_value::<super::ImportGroupMemberResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
