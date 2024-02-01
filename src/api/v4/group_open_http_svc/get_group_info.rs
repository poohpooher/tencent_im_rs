//! <https://www.tencentcloud.com/document/product/1047/34961>
use crate::api::v4::common::{ActionStatus, ErrorCode, Group};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(get_group_info, GetGroupInfoRequest, GetGroupInfoResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupInfoRequest {
    #[serde(rename = "GroupIdList")]
    group_id_list: Vec<String>,

    #[serde(rename = "ResponseFilter")]
    response_filter: Option<ResponseFilter>,
}

impl GetGroupInfoRequest {
    pub fn new<S: AsRef<str>>(group_id_list: Vec<S>) -> Self {
        Self {
            group_id_list: group_id_list
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
            response_filter: None,
        }
    }

    pub fn set_response_filter(&mut self, response_filter: Option<ResponseFilter>) -> &mut Self {
        self.response_filter = response_filter;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFilter {
    #[serde(rename = "GroupBaseInfoFilter")]
    group_base_info_filter: Option<Vec<String>>,

    #[serde(rename = "MemberInfoFilter")]
    member_info_filter: Option<Vec<String>>,

    #[serde(rename = "AppDefinedDataFilter_Group")]
    app_defined_data_filter_group: Option<Vec<String>>,

    #[serde(rename = "AppDefinedDataFilter_GroupMember")]
    app_defined_data_filter_group_member: Option<Vec<String>>,

    #[serde(rename = "SelfInfoFilter")]
    self_info_filter: Option<Vec<String>>,
}

impl Default for ResponseFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseFilter {
    pub fn new() -> Self {
        Self {
            group_base_info_filter: None,
            member_info_filter: None,
            app_defined_data_filter_group: None,
            app_defined_data_filter_group_member: None,
            self_info_filter: None,
        }
    }

    pub fn set_group_base_info_filter<S: AsRef<str>>(
        &mut self,
        group_base_info_filter: Option<Vec<S>>,
    ) -> &mut Self {
        self.group_base_info_filter =
            group_base_info_filter.map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_member_info_filter<S: AsRef<str>>(
        &mut self,
        member_info_filter: Option<Vec<S>>,
    ) -> &mut Self {
        self.member_info_filter =
            member_info_filter.map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_app_defined_data_filter_group<S: AsRef<str>>(
        &mut self,
        app_defined_data_filter_group: Option<Vec<S>>,
    ) -> &mut Self {
        self.app_defined_data_filter_group = app_defined_data_filter_group
            .map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_app_defined_data_filter_group_member<S: AsRef<str>>(
        &mut self,
        app_defined_data_filter_group_member: Option<Vec<S>>,
    ) -> &mut Self {
        self.app_defined_data_filter_group_member = app_defined_data_filter_group_member
            .map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_self_info_filter<S: AsRef<str>>(
        &mut self,
        self_info_filter: Option<Vec<S>>,
    ) -> &mut Self {
        self.self_info_filter =
            self_info_filter.map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupInfoResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupInfo")]
    pub group_info: Option<Vec<Group>>,
}

#[cfg(test)]
mod test_get_group_info {
    use crate::api::v4::common::Member;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupIdList": [ // The list of group IDs specified for the query. This parameter is required.
              "@TGS#1NVTZEAE4",
              "@TGS#1CXTZEAET"
          ]
        });

        let req = super::GetGroupInfoRequest::new(vec!["@TGS#1NVTZEAE4", "@TGS#1CXTZEAET"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "GroupIdList": [ // The list of group IDs specified for the query. This parameter is required.
                "@TGS#1NVTZEAE4",
                "@TGS#1CXTZEAET"
            ],
            "ResponseFilter": { // The filters that you specify for the response.
                "GroupBaseInfoFilter": [ // Add this array if you want to query the basic information.
                    "Type",
                    "Name",
                    "Introduction",
                    "Notification"
                ],
                "MemberInfoFilter": [ // Add this array if you want to query the member information.
                    "Account", // Member ID
                    "Role"
                ],
                "AppDefinedDataFilter_Group": [ // The filter for group-specific custom fields. This filter is disabled by default.
                    "GroupTestData1",
                    "GroupTestData2"
                ],
                "AppDefinedDataFilter_GroupMember": [ // The filter for group member-specific custom fields. This filter is disabled by default.
                    "MemberDefined2",
                    "MemberDefined1"
                ]
            }
        });

        let mut filter = super::ResponseFilter::new();
        filter
            .set_group_base_info_filter(Some(vec!["Type", "Name", "Introduction", "Notification"]))
            .set_member_info_filter(Some(vec!["Account", "Role"]))
            .set_app_defined_data_filter_group(Some(vec!["GroupTestData1", "GroupTestData2"]))
            .set_app_defined_data_filter_group_member(Some(vec![
                "MemberDefined2",
                "MemberDefined1",
            ]));

        let mut req = super::GetGroupInfoRequest::new(vec!["@TGS#1NVTZEAE4", "@TGS#1CXTZEAET"]);
        req.set_response_filter(Some(filter));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn test_member_json() {
        let sample = json!({
            "Member_Account": "peter",
            "Role": "Member",
            "JoinTime": 1425976500, // Time when the member joined the group
            "MsgSeq": 1233,
            "MsgFlag": "AcceptAndNotify",
            "LastSendMsgTime": 1425976500, // Last time when the member sent a message
            "MuteUntil": 0, // The value `0` indicates that the member is not muted, and other values indicate the time when the member will be unmuted.
            "AppMemberDefinedData":[ // Custom group member fields
                {
                    "Key": "MemberDefined1",
                    "Value": "ModifyDefined1"
                },
                {
                    "Key":"MemberDefined2",
                    "Value":"ModifyDefined2"
                }
             ]
        });

        let res = serde_json::from_value::<Member>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "", // The ErrorInfo parameter here is meaningless. You need to check the ErrorInfo parameter of each group.
          "ErrorCode": 0, // The ErrorCode parameter here is meaningless. You need to check the ErrorCode parameter of each group.
          "GroupInfo": [ // A group information array is returned. Only one group is listed here for simplicity.
              {
                  "GroupId": "@TGS#2J4SZEAEL",
                  "ErrorCode": 0, // The result specific to this group
                  "ErrorInfo": "" , // The result specific to this group
                  "Type": "Public", // Group type
                  "Name": "MyFirstGroup", // Group name
                  "Appid":1400001001,// SDKAppID of the Chat app
                  "Introduction": "TestGroup", // Group introduction
                  "Notification": "TestGroup", // Group notice
                  "FaceUrl": "http://this.is.face.url", // Group profile photo
                  "Owner_Account": "leckie", // Group owner ID
                  "CreateTime": 1426976500, // Group creation time in UTC
                  "LastInfoTime": 1426976500, // The UTC time when the group information was last updated
                  "LastMsgTime": 1426976600, // The UTC time when the last message in the group was sent
                  "NextMsgSeq": 1234,
                  "MemberNum": 2, // Current number of members in the group
                  "MaxMemberNum": 50, // Maximum number of members in the group
                  "ApplyJoinOption": "FreeAccess", // Method of handling requests to join the group
                  "MuteAllMember": "On", // Whether to mute all members in the group
                  "AppDefinedData": [ // Group-specific custom fields
                      {
                          "Key": "GroupTestData1", // Key of the custom field
                          "Value": "xxxx" // Value of the custom field
                      },
                      {
                          "Key": "GroupTestData2",
                          "Value": r#"abc\u0000\u0001"# // The custom field supports binary data.
                      }
                  ],
                  "MemberList": [ // Group member list
                      {
                          "Member_Account": "leckie", // Member ID
                          "Role": "Owner", // The role of the member in the group
                          "JoinTime": 1425976500, // The UTC time when the member joined the group
                          "MsgSeq": 1233,
                          "MsgFlag": "AcceptAndNotify", // Indicates whether the member blocks group messages.
                          "LastSendMsgTime": 1425976500, // The last time in UTC when the member sent a message in the group
                          "MuteUntil": 1431069882, // UTC time when the muting period expires
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
                          "JoinTime": 1425976500, // Time when the member joined the group
                          "MsgSeq": 1233,
                          "MsgFlag": "AcceptAndNotify",
                          "LastSendMsgTime": 1425976500, // Last time when the member sent a message
                          "MuteUntil": 0, // The value `0` indicates that the member is not muted, and other values indicate the time when the member will be unmuted.
                          "AppMemberDefinedData":[ // Custom group member fields
                              {
                                  "Key": "MemberDefined1",
                                  "Value": "ModifyDefined1"
                              },
                              {
                                  "Key":"MemberDefined2",
                                  "Value":"ModifyDefined2"
                              }
                           ]
                      }
                  ]
              }
          ]
        });

        let res = serde_json::from_value::<super::GetGroupInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "", // The ErrorInfo parameter here is meaningless. You need to check the ErrorInfo parameter of each group.
          "ErrorCode": 0, // The ErrorCode parameter here is meaningless. You need to check the ErrorCode parameter of each group.
          "GroupInfo": [ // A group information array is returned. Only one group is listed here for simplicity.
              {
                  "GroupId": "@TGS#2J4SZEAEL",
                  "ErrorCode": 0, // The result specific to this group
                  "ErrorInfo":"" , // The result specific to this group
                  "Type": "Public", // Group type
                  "Name": "MyFirstGroup", // Group name
                  "Introduction": "TestGroup", // Group introduction
                  "Notification": "TestGroup", // Group notice
                  "AppDefinedData": [ // Group-specific custom fields
                      {
                          "Key": "GroupTestData1", // Key of the custom field
                          "Value": "xxxx" // Value of the custom field
                      },
                      {
                          "Key": "GroupTestData2",
                          "Value": r#"abc\u0000\u0001"# // The custom field supports binary data.
                      }
                  ],
                  "MemberList": [ // Group member list
                      {
                          "Member_Account": "leckie", // Member ID
                          "Role": "Owner", // The role of the member in the group
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
                      }
                  ]
              }
          ]
        });

        let res = serde_json::from_value::<super::GetGroupInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
