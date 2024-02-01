//! <https://www.tencentcloud.com/ko/document/product/1047/34948>

use crate::api::v4::common::{ActionStatus, ErrorCode, Member};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_group_member,
    GetGroupMemberInfoRequest,
    GetGroupMemberInfoResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupMemberInfoRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Limit")]
    limit: Option<u32>,

    #[serde(rename = "Offset")]
    offset: Option<u32>,

    #[serde(rename = "Next")]
    next: Option<String>,

    #[serde(rename = "MemberInfoFilter")]
    member_info_filter: Option<Vec<String>>,

    #[serde(rename = "MemberRoleFilter")]
    member_role_filter: Option<Vec<String>>,

    #[serde(rename = "AppDefinedDataFilter_GroupMember")]
    app_defined_data_filter_group_member: Option<Vec<String>>,
}

impl GetGroupMemberInfoRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            limit: None,
            offset: None,
            next: None,
            member_info_filter: None,
            member_role_filter: None,
            app_defined_data_filter_group_member: None,
        }
    }

    pub fn set_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_offset(&mut self, offset: Option<u32>) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn set_next<S: AsRef<str>>(&mut self, next: Option<S>) -> &mut Self {
        self.next = next.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_member_info_filter<S: AsRef<str>>(
        &mut self,
        member_info_filter: Option<Vec<S>>,
    ) -> &mut Self {
        self.member_info_filter =
            member_info_filter.map(|s| s.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_member_role_filter<S: AsRef<str>>(
        &mut self,
        member_role_filter: Option<Vec<S>>,
    ) -> &mut Self {
        self.member_role_filter =
            member_role_filter.map(|s| s.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_app_defined_data_filter_group_member<S: AsRef<str>>(
        &mut self,
        app_defined_data_filter_group_member: Option<Vec<S>>,
    ) -> &mut Self {
        self.app_defined_data_filter_group_member = app_defined_data_filter_group_member
            .map(|s| s.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupMemberInfoResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "Next")]
    pub next: Option<String>,

    #[serde(rename = "MemberNum")]
    pub member_num: Option<u32>,

    #[serde(rename = "MemberList")]
    pub member_list: Option<Vec<Member>>,
}

#[cfg(test)]
mod test_get_group_member_info {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId":"@TGS#1NVTZEAE4"  // Group ID (required)
        });

        let req = super::GetGroupMemberInfoRequest::new("@TGS#1NVTZEAE4");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "GroupId":"@TGS#1NVTZEAE4", // Group ID (required)
            "Limit": 100, // Maximum number of members to pull information
            "Offset": 0 // Sequence number of the member from whom to start pulling information
        });

        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#1NVTZEAE4");
        req.set_limit(Some(100));
        req.set_offset(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
            "GroupId":"@TGS#_@TGS#cAVQXXXXXX", // Group ID (required)
            "Limit": 100, // Maximum number of members to pull information
            "Next": "" // Start pulling from the position where the last pulling ends
        });
        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#_@TGS#cAVQXXXXXX");
        req.set_limit(Some(100));
        req.set_next(Some(""));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "GroupId":"@TGS#1NVTZEAE4", // Group ID (required)
          "MemberInfoFilter": [ // Information to pull, where `Member_Account` is included by default. If this field is not specified, all group member information will be pulled.
              "Role",
              "JoinTime",
              "MsgSeq",
              "MsgFlag",
              "LastSendMsgTime",
              "MuteUntil",
              "NameCard"
          ]
        });

        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#1NVTZEAE4");
        req.set_member_info_filter(Some(vec![
            "Role",
            "JoinTime",
            "MsgSeq",
            "MsgFlag",
            "LastSendMsgTime",
            "MuteUntil",
            "NameCard",
        ]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request5() {
        let sample = json!({
          "GroupId":"@TGS#37AB3PAEC", // Group ID (required)
          "MemberRoleFilter":[ // Member role filter
              "Owner",
              "Member"
          ]
        });

        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#37AB3PAEC");
        req.set_member_role_filter(Some(vec!["Owner", "Member"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request6() {
        let sample = json!({
          "GroupId":"@TGS#37AB3PAEC", // Group ID (required)
          "AppDefinedDataFilter_GroupMember": [ // Filter for custom group member fields
              "MemberDefined2" // Key of a custom group member field
          ]
        });

        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#37AB3PAEC");
        req.set_app_defined_data_filter_group_member(Some(vec!["MemberDefined2"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request7() {
        let sample = json!({
          "GroupId":"@TGS#1NVTZEAE4", // Group ID (required)
          "MemberInfoFilter": [ // Information to pull. If this field is not specified, all group member information will be pulled.
              "Role",
              "JoinTime",
              "MsgSeq",
              "MsgFlag",
              "LastSendMsgTime",
              "MuteUntil",
              "NameCard"
          ],
         "MemberRoleFilter":[ // Member role filter
              "Owner",
              "Member"
          ],
         "AppDefinedDataFilter_GroupMember": [ // Filter for custom group member fields
              "MemberDefined2", // Key of a custom group member field
              "MemberDefined1"
          ],
          "Limit": 100, // Maximum number of members to pull information
          "Offset": 0 // Sequence number of the member from whom to start pulling information
        });

        let mut req = super::GetGroupMemberInfoRequest::new("@TGS#1NVTZEAE4");
        req.set_member_info_filter(Some(vec![
            "Role",
            "JoinTime",
            "MsgSeq",
            "MsgFlag",
            "LastSendMsgTime",
            "MuteUntil",
            "NameCard",
        ]));
        req.set_member_role_filter(Some(vec!["Owner", "Member"]));
        req.set_app_defined_data_filter_group_member(Some(vec![
            "MemberDefined2",
            "MemberDefined1",
        ]));
        req.set_limit(Some(100));
        req.set_offset(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "Next": "144115265295492787", // This field is returned only for a community group
          "MemberNum": 2, // Total number of members in the group
          "MemberList": [ // Group member list
              {
                  "Member_Account": "bob",
                  "Role": "Owner",
                  "JoinTime": 1425976500, // Time when the member joined the group
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500, // Last time when the member sent a message
                  "MuteUntil": 1431069882, // Muting end time in seconds
                  "AppMemberDefinedData": [ // Custom group member fields
                      {
                         "Key": "MemberDefined1",
                         "Value": "ModifyDefined1"
                      },
                      {
                          "Key": "MemberDefined2",
                          "Value": "ModifyDefined2"
                      }
                   ]
              },
              {
                  "Member_Account": "peter",
                  "Role": "Member",
                  "JoinTime": 1425976500,
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500,
                  "MuteUntil": 0, // `0`: the member is not muted; other values: the time when the member will be unmuted
                  "AppMemberDefinedData": [ // Custom group member fields
                      {
                         "Key": "MemberDefined1",
                         "Value": "ModifyDefined1"
                      },
                      {
                          "Key": "MemberDefined2",
                          "Value": "ModifyDefined2"
                      }
                   ]
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "MemberNum": 2, // Total number of members in the group
          "MemberList": [ // Group member list
              {
                  "Member_Account": "bob",
                  "Role": "Owner",
                  "JoinTime": 1425976500, // Time when the member joined the group
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500, // Last time when the member sent a message
                  "MuteUntil": 1431069882, // Muting end time in seconds
              },
              {
                  "Member_Account": "peter",
                  "Role": "Member",
                  "JoinTime": 1425976500,
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500,
                  "MuteUntil": 0, // `0`: the member is not muted; other values: the time when the member will be unmuted
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response3() {
        let sample = json!({
          "ActionStatus": "OK", // The request succeeded.
          "ErrorCode": 0, // Return code
          "MemberList": [
              {
                  "JoinTime": 1450680436, // Time when the member joined the group
                  "LastSendMsgTime": 0, // Last time when the member sent a message
                  "Member_Account": "Test_1", // Member account
                  "MsgFlag": "AcceptNotNotify", // Type of member messages being blocked
                  "MsgSeq": 1, // Sequence number of the member’s read message
                  "NameCard": "", // Member’s contact card
                  "Role": "Owner", // Member’s role
                  "MuteUntil": 0 // `0`: the member is not muted; other values: the time when the member will be unmuted
              },
              {
                  "JoinTime": 1450680436,
                  "LastSendMsgTime": 0,
                  "Member_Account": "Test_6",
                  "MsgFlag": "AcceptNotNotify",
                  "MsgSeq": 1,
                  "NameCard": "",
                  "Role": "Admin",
                  "MuteUntil": 0
              }
          ],
          "MemberNum": 8 // Total number of members in the group
        });

        let res =
            serde_json::from_value::<super::GetGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response4() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "MemberNum": 2, // Total number of members in the group
          "MemberList": [ // Group member list
              {
                  "Member_Account": "bob",
                  "Role": "Owner",
                  "JoinTime": 1425976500, // Time when the member joined the group
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500, // Last time when the member sent a message
                  "MuteUntil": 1431069882, // Muting end time in seconds
                   "AppMemberDefinedData": [ // Custom group member fields
                      {
                          "Key": "MemberDefined2",
                          "Value": "ModifyDefined2"
                      }
                   ]
              },
              {
                  "Member_Account": "peter",
                  "Role": "Member",
                  "JoinTime": 1425976500,
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500,
                  "MuteUntil": 0, // `0`: the member is not muted; other values: the time when the member will be unmuted
                  "AppMemberDefinedData": [ // Custom group member fields
                      {
                          "Key": "MemberDefined2",
                          "Value": "ModifyDefined2"
                      }
                   ]
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response5() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "Next": "144115265295492787", // This field is returned only for a community group
          "MemberNum": 2, // Total number of members in the group
          "MemberList": [ // Group member list
              {
                  "Member_Account": "bob",
                  "Role": "Owner",
                  "JoinTime": 1425976500, // Time when the member joined the group
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500, // Last time when the member sent a message
                  "MuteUntil": 1431069882, // Muting end time in seconds
                  "AppMemberDefinedData":[ // Custom group member fields
                      {
                         "Key":"MemberDefined1",
                         "Value":"ModifyDefined1"
                      },
                      {
                          "Key":"MemberDefined2",
                          "Value":"ModifyDefined2"
                      }
                   ]
              },
              {
                  "Member_Account": "peter",
                  "Role": "Member",
                  "JoinTime": 1425976500,
                  "MsgSeq": 1233,
                  "MsgFlag": "AcceptAndNotify",
                  "LastSendMsgTime": 1425976500,
                  "MuteUntil": 0, // `0`: the member is not muted; other values: the time when the member will be unmuted
                  "AppMemberDefinedData": [ // Custom group member fields
                      {
                         "Key": "MemberDefined1",
                         "Value": "ModifyDefined1"
                      },
                      {
                          "Key": "MemberDefined2",
                          "Value": "ModifyDefined2"
                      }
                   ]
              }
          ]
        });

        let res =
            serde_json::from_value::<super::GetGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
