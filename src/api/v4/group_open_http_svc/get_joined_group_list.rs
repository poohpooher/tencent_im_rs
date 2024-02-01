//! <https://www.tencentcloud.com/ko/document/product/1047/34925>

use crate::api::v4::common::group_type::GroupType;
use crate::api::v4::common::{ActionStatus, ErrorCode, Group};
use crate::api::v4::group_open_http_svc::get_group_info::ResponseFilter;
use crate::api::{bool_to_int, int_to_bool};
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
    #[serde(rename = "Member_Account")]
    member_account: String,

    #[serde(
        rename = "WithHugeGroups",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    with_huge_groups: Option<bool>,

    #[serde(
        rename = "WithNoActiveGroups",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    with_no_active_groups: Option<bool>,

    #[serde(rename = "Limit")]
    limit: Option<u32>,

    #[serde(rename = "Offset")]
    offset: Option<u32>,

    #[serde(rename = "GroupType")]
    group_type: Option<GroupType>,

    #[serde(rename = "ResponseFilter")]
    response_filter: Option<ResponseFilter>,

    #[serde(
        rename = "SupportTopic",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    support_topic: Option<bool>,
}

impl GetJoinedGroupListRequest {
    pub fn new<S: AsRef<str>>(member_account: S) -> Self {
        Self {
            member_account: member_account.as_ref().to_string(),
            with_huge_groups: None,
            with_no_active_groups: None,
            limit: None,
            offset: None,
            group_type: None,
            response_filter: None,
            support_topic: None,
        }
    }

    pub fn set_with_huge_groups(&mut self, with_huge_groups: Option<bool>) -> &mut Self {
        self.with_huge_groups = with_huge_groups;
        self
    }

    pub fn set_with_no_active_groups(&mut self, with_no_active_groups: Option<bool>) -> &mut Self {
        self.with_no_active_groups = with_no_active_groups;
        self
    }

    pub fn set_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_offset(&mut self, offset: Option<u32>) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn set_group_type(&mut self, group_type: Option<GroupType>) -> &mut Self {
        self.group_type = group_type;
        self
    }

    pub fn set_response_filter(&mut self, response_filter: Option<ResponseFilter>) -> &mut Self {
        self.response_filter = response_filter;
        self
    }

    pub fn set_support_topic(&mut self, support_topic: Option<bool>) -> &mut Self {
        self.support_topic = support_topic;
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

    #[serde(rename = "TotalCount")]
    pub total_count: Option<u32>,

    #[serde(rename = "GroupIdList")]
    pub group_id_list: Option<Vec<Group>>,
}

#[cfg(test)]
mod test_get_joined_group_list {
    use crate::api::v4::common::GroupType;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "Member_Account": "leckie"
        });

        let req = super::GetJoinedGroupListRequest::new("leckie");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "Member_Account": "leckie",
            "Limit": 10, // Number of groups to be pulled. If this field is not specified, all groups will be pulled.
            "Offset": 0 // Sequence number of the group starting from which information is pulled.
        });

        let mut req = super::GetJoinedGroupListRequest::new("leckie");
        req.set_limit(Some(10));
        req.set_offset(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "Member_Account": "leckie",
          "GroupType" : "Public" // Type of groups to be pulled. If this parameter is not specified, all types of groups will be pulled.
        });
        let mut req = super::GetJoinedGroupListRequest::new("leckie");
        req.set_group_type(Some(GroupType::Public));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "Member_Account": "leckie",
          "WithHugeGroups":1, // Supports pulling the information of audio-video groups (AVChatRoom).
          "WithNoActiveGroups":1,// Supports pulling the information of Private groups (same as Work groups in the new version) that the user has joined but are not activated.
          "Limit": 10, // Number of groups to be pulled. If this field is not specified, all groups will be pulled.
          "Offset": 0, // Sequence number of the group starting from which information is pulled.
          "ResponseFilter": {
              "GroupBaseInfoFilter": [ // Basic information fields to be pulled
                  "Type",
                  "Name",
                  "Introduction",
                  "Notification"
              ],
              "SelfInfoFilter": [ // Member's personal information in the group
                  "Role", // Role in the group
                  "JoinTime" // Time when the member joined the group
              ]
          }
        }
        );

        let mut fileter = super::ResponseFilter::new();
        fileter.set_group_base_info_filter(Some(vec![
            "Type",
            "Name",
            "Introduction",
            "Notification",
        ]));
        fileter.set_self_info_filter(Some(vec!["Role", "JoinTime"]));

        let mut req = super::GetJoinedGroupListRequest::new("leckie");
        req.set_with_huge_groups(Some(true));
        req.set_with_no_active_groups(Some(true));
        req.set_limit(Some(10));
        req.set_offset(Some(0));
        req.set_response_filter(Some(fileter));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request5() {
        let sample = json!({
          "Member_Account": "107867",// User account to be queried (required)
          "SupportTopic": 1// Whether the specified group type supports topics. This field is supported only by community groups.
        });

        let mut req = super::GetJoinedGroupListRequest::new("107867");
        req.set_support_topic(Some(true));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request6() {
        let sample = json!({
        "Member_Account": "leckie",
        "WithHugeGroups":1,
        "WithNoActiveGroups":1,
        "ResponseFilter": {
              "GroupBaseInfoFilter": [
                  "Type",
                  "Name",
                  "Introduction",
                  "Notification",
                  "FaceUrl",
                  "CreateTime",
                  "Owner_Account",
                  "LastInfoTime",
                  "LastMsgTime",
                  "NextMsgSeq",
                  "MemberNum",
                  "MaxMemberNum",
                  "ApplyJoinOption",
                  "MuteAllMember"
              ],
              "SelfInfoFilter": [
                  "Role",
                  "JoinTime",
                  "MsgFlag",
                  "MsgSeq"
              ]
          }
        });

        let mut req = super::GetJoinedGroupListRequest::new("leckie");
        req.set_with_huge_groups(Some(true));
        req.set_with_no_active_groups(Some(true));
        let mut fileter = super::ResponseFilter::new();
        fileter.set_group_base_info_filter(Some(vec![
            "Type",
            "Name",
            "Introduction",
            "Notification",
            "FaceUrl",
            "CreateTime",
            "Owner_Account",
            "LastInfoTime",
            "LastMsgTime",
            "NextMsgSeq",
            "MemberNum",
            "MaxMemberNum",
            "ApplyJoinOption",
            "MuteAllMember",
        ]));
        fileter.set_self_info_filter(Some(vec!["Role", "JoinTime", "MsgFlag", "MsgSeq"]));
        req.set_response_filter(Some(fileter));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "TotalCount": 2, // This value is the total number of groups that meet the conditions, regardless of the `Limit` and `Offset` settings.
          "GroupIdList": [
              {
                  "GroupId": "@TGS#2J4SZEAEL"
              },
              {
                  "GroupId": "@TGS#2C5SZEAEF"
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "TotalCount": 1,
          "GroupIdList": [
              {
                  "GroupId": "@TGS#2J4SZEAEL"
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response3() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "TotalCount": 2,
          "GroupIdList": [
              {
                  "GroupId": "@TGS#16UMONKGG",
                  "Introduction": "",
                  "Name": "d",
                  "Notification": "",
                  "SelfInfo": {
                      "JoinTime": 1588148506,
                      "Role": "Member"
                  },
                  "Type": "Private"
              },
              {
                  "GroupId": "@TGS#3FCOX2MGW",
                  "Introduction": "",
                  "Name": "TestGroup",
                  "Notification": "",
                  "SelfInfo": {
                      "JoinTime": 1588041114,
                      "Role": "Member"
                  },
                  "Type": "ChatRoom"
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response4() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "ok",
          "ErrorCode": 0,
          "TotalCount": 1,
          "GroupIdList": [
              {
                  "GroupId": "@TGS#_@TGS#cMOQ7HIM62CD",
                  "Type": "Community",
                  "SupportTopic": 1,
                  "GrossTopicNextMsgSeq": 3,
                  "SelfInfo": {
                  "GrossTopicReadSeq": 2
                  }
              }
          ]
        }
        );

        let res =
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response5() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "TotalCount": 1, // This value is the total number of groups that meet the conditions, regardless of the `Limit` and `Offset` settings.
          "GroupIdList": [
              {
                  "ApplyJoinOption": "DisableApply",
                  "CreateTime": 1585718204,
                  "FaceUrl": "",
                  "GroupId": "@TGS#16UMONKGG",
                  "Introduction": "",
                  "LastInfoTime": 1588148506,
                  "LastMsgTime": 0,
                  "MaxMemberNum": 200,
                  "MemberNum": 1,
                  "Name": "d",
                  "NextMsgSeq": 2,
                  "Notification": "",
                  "Owner_Account": "",
                  "SelfInfo": {
                      "JoinTime": 1588148506,
                      "MsgFlag": "AcceptAndNotify",
                      "Role": "Member",
                      "MsgSeq": 1
                  },
                  "MuteAllMember": "Off",
                  "Type": "Private"
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetJoinedGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
