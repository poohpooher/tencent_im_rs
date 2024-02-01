//! <https://www.tencentcloud.com/document/product/1047/34921>
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use crate::api::v4::common::Member;
use crate::tencent_api;

tencent_api!(
    add_group_member,
    AddGroupMemberRequest,
    AddGroupMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddGroupMemberRequest {
    #[serde(rename = "GroupId")]
    /// 그룹 아이디
    group_id: String,

    #[serde(rename = "Silence")]
    /// 그룹 멤버 추가 시 알림 여부
    silence: Option<u32>,

    #[serde(rename = "MemberList")]
    /// 그룹 멤버 리스트
    member_list: Vec<Member>,
}

impl AddGroupMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, member_list: Vec<Member>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            silence: None,
            member_list,
        }
    }

    pub fn set_silence(&mut self, silence: Option<u32>) -> &mut Self {
        self.silence = silence;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddGroupMemberResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "MemberList")]
    pub member_list: Option<Vec<Member>>,
}

#[cfg(test)]
mod test_add_group_member {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // (Required) Target group
          "MemberList": [ // Up to 300 members can be added at a time.
          {
              "Member_Account": "tommy" // The ID of the member to be added (required)
          },
          {
              "Member_Account": "jared"
          }]
        });

        let mut member_list = Vec::new();
        let mut member = super::Member::new();
        member.set_member_account(Some("tommy"));
        member_list.push(member);

        let mut member = super::Member::new();
        member.set_member_account(Some("jared"));
        member_list.push(member);

        let req = super::AddGroupMemberRequest::new("@TGS#2J4SZEAEL", member_list);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // (Required) Target group
          "Silence": 1, // Whether to add members silently (optional)
          "MemberList": [ // Up to 300 members can be added at a time.
          {
              "Member_Account": "tommy" // The ID of the member to be added (required)
          },
          {
              "Member_Account": "jared"
          }]
        });

        let mut member_list = Vec::new();
        let mut member = super::Member::new();
        member.set_member_account(Some("tommy"));
        member_list.push(member);

        let mut member = super::Member::new();
        member.set_member_account(Some("jared"));
        member_list.push(member);

        let mut req = super::AddGroupMemberRequest::new("@TGS#2J4SZEAEL", member_list);
        req.set_silence(Some(1));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
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
                 "Result": 1 // The result of adding the member. 0: failed. 1: successful. 2: already in the group.
            },
            {
                 "Member_Account": "jared",
                 "Result": 1
            }]
        });

        let res = serde_json::from_value::<super::AddGroupMemberResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
