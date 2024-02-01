//! <https://www.tencentcloud.com/ko/document/product/1047/34895>
use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use crate::api::v4::common::Group;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::group_type::GroupType;

use crate::tencent_api;

tencent_api!(create_group, CreateGroupRequest, CreateGroupResponse);

pub type CreateGroupRequest = Group;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// 그룹 생성 응답
pub struct CreateGroupResponse {
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

    #[serde(rename = "Type")]
    pub group_type: Option<GroupType>,

    #[serde(rename = "HugeGroupFlag")]
    pub huge_group_flag: Option<u32>,
}

#[cfg(test)]
mod test_create_group {
    use crate::api::v4::common::{ApplyJoinOption, GroupType, KeyValuePascal, Member, MemberRole};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "Owner_Account": "leckie", // UserId of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, AVChatRoom, or Community
          "Name": "TestGroup" // Group name (required)
        });

        let mut req = super::CreateGroupRequest::new();
        req.set_owner_account(Some("leckie"))
            .set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request2() {
        let sample = json!({
          "Owner_Account": "leckie", // UserId of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, AVChatRoom, or Community
          "Name": "TestGroup", // Group name (required)
          "Introduction": "This is group Introduction", // Group introduction (optional)
          "Notification": "This is group Notification", // Group notice (optional)
          "FaceUrl": "http://this.is.face.url", // Group profile photo URL (optional)
          "MaxMemberCount": 500, // Maximum number of group members (optional)
          "ApplyJoinOption": "FreeAccess"  // Method for handling requests to join the group (optional)
        });

        let mut req = super::CreateGroupRequest::new();

        req.set_owner_account(Some("leckie"))
            .set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_introduction(Some("This is group Introduction"))
            .set_notification(Some("This is group Notification"))
            .set_face_url(Some("http://this.is.face.url"))
            .set_max_member_count(Some(500))
            .set_apply_join_option(Some(ApplyJoinOption::FreeAccess));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request3() {
        let sample = json!({
          "Name": "TestGroup", // Group name (required)
          "Type": "Public", // Group type: Private, Public, ChatRoom, or Community (AVChatRoom is not supported) (required)
          "MemberList": [ // Initial group member list, which contains a maximum of 100 members (optional)
               {
                  "Member_Account": "bob", // Member (required)
                  "Role": "Admin" // Role assigned to the member (optional). Currently, only the Admin option is available.
               },
               {
                  "Member_Account": "peter"
               }
           ]
        });

        let mut member_list = vec![];
        let mut member = Member::new();
        member.set_member_account(Some("bob"));
        member.set_role(Some(MemberRole::Admin));
        member_list.push(member);
        let mut member = Member::new();
        member.set_member_account(Some("peter"));
        member_list.push(member);

        let mut req = super::CreateGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_member_list(Some(member_list));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request4() {
        let sample = json!({
          "Owner_Account": "leckie", // UserId of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, AVChatRoom, or Community (required)
          "GroupId": "MyFirstGroup", // User-defined group ID (optional)
          "Name": "TestGroup"   // Group name (required)
        });

        let mut req = super::CreateGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_owner_account(Some("leckie"))
            .set_group_id(Some("MyFirstGroup"));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request5() {
        let sample = json!({
          "Name": "TestGroup", // Group name (required)
          "Type": "Public", // Group type: Private, Public, ChatRoom, AVChatRoom, or Community (required)
          "AppDefinedData": [ // Group custom field (optional)
              {
                  "Key": "GroupTestData1", // Key of the app custom field
                  "Value": "xxxxx" // Value of the custom field
              },
              {
                  "Key": " GroupTestData2",
                  "Value": r#"abc\u0000\u0001"# // The custom field supports binary data.
              }
          ]
        });

        let mut req = super::CreateGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_app_defined_data(Some(vec![
                KeyValuePascal::new("GroupTestData1", "xxxxx"),
                KeyValuePascal::new(" GroupTestData2", r#"abc\u0000\u0001"#),
            ]));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request6() {
        let sample = json!({
          "Owner_Account": "leckie", // UserId of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, or Community (AVChatRoom is not supported) (required)
          "Name": "TestGroup", // Group name (required)
          "MemberList": [
             {
                "Member_Account":"bob",
                "AppMemberDefinedData":[ // Group member custom field (optional)
                     {
                         "Key": "MemberDefined1", // Group member custom key
                         "Value": "MemberData1" // Value of the group member custom field
                     },
                     {
                         "Key": "MemberDefined2",
                         "Value": "MemberData2"
                     }
                 ]
             },
             {
                "Member_Account":"peter",
                "AppMemberDefinedData":[
                     {
                         "Key": "MemberDefined1",
                         "Value": "MemberData1"
                     },
                     {
                         "Key": "MemberDefined2",
                         "Value": "MemberData2"
                     }
                 ]
             }
          ]
        });

        let mut member_list = vec![];

        let mut member = Member::new();
        member.set_member_account(Some("bob"));
        member.set_app_member_defined_data(Some(vec![
            KeyValuePascal::new("MemberDefined1", "MemberData1"),
            KeyValuePascal::new("MemberDefined2", "MemberData2"),
        ]));
        member_list.push(member);

        let mut member = Member::new();
        member.set_member_account(Some("peter"));

        member.set_app_member_defined_data(Some(vec![
            KeyValuePascal::new("MemberDefined1", "MemberData1"),
            KeyValuePascal::new("MemberDefined2", "MemberData2"),
        ]));

        member_list.push(member);

        let mut req = super::CreateGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_owner_account(Some("leckie"))
            .set_member_list(Some(member_list));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request7() {
        let sample = json!({
          "Type": "Community", 	// Group type (required)
          "Name": "TestCommunityGroup", // Group name (required)
          "SupportTopic": 1			// Whether the topic option is supported. Valid values: `1`: yes; `0`: no.
        });

        let mut req = super::CreateGroupRequest::new();

        req.set_group_type(Some(GroupType::Community))
            .set_name(Some("TestCommunityGroup"))
            .set_support_topic(Some(1));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn request8() {
        let sample = json!({
          "Owner_Account": "leckie", // UserId of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, or Community (AVChatRoom is not supported) (required)
          "GroupId":"MyFirstGroup", // User-defined group ID (optional)
          "Name": "TestGroup", // Group name (required)
          "Introduction": "This is group Introduction", // Group introduction (optional)
          "Notification": "This is group Notification", // Group notice (optional)
          "FaceUrl": "http://this.is.face.url", // Group profile photo URL (optional)
          "MaxMemberCount": 500, // Maximum number of group members (optional)
          "ApplyJoinOption": "FreeAccess", // Method for handling requests to join the group (optional)
          "AppDefinedData": [ // Group custom field (optional)
              {
                  "Key": "GroupTestData1", // Key of the app custom field
                  "Value": "xxxxx" // Value of the custom field
              },
              {
                  "Key": "GroupTestData2",
                  "Value": r#"abc\u0000\u0001"# // The custom field supports binary data.
              }
          ],
          "MemberList": [ // Initial group member list, which contains a maximum of 100 members (optional)
              {
                  "Member_Account": "bob", // Member (required)
                  "Role": "Admin", // Role assigned to the member. Currently, only the Admin option is available. (optional)
                  "AppMemberDefinedData":[ // Group member custom field (optional)
                     {
                         "Key":"MemberDefined1", // Group member custom key
                         "Value":"MemberData1" // Value of the group member custom field
                     },
                     {
                         "Key":"MemberDefined2",
                         "Value":"MemberData2"
                     }
                 ]
              },
              {
                  "Member_Account": "peter",
                  "AppMemberDefinedData":[
                     {
                         "Key":"MemberDefined1",
                         "Value":"MemberData1"
                     },
                     {
                         "Key":"MemberDefined2",
                         "Value":"MemberData2"
                     }
                 ]
              }
          ]
        });

        let mut member_list = vec![];
        let mut member = Member::new();
        member.set_member_account(Some("bob"));
        member
            .set_role(Some(MemberRole::Admin))
            .set_app_member_defined_data(Some(vec![
                KeyValuePascal::new("MemberDefined1", "MemberData1"),
                KeyValuePascal::new("MemberDefined2", "MemberData2"),
            ]));
        member_list.push(member);
        let mut member = Member::new();
        member.set_member_account(Some("peter"));
        member.set_app_member_defined_data(Some(vec![
            KeyValuePascal::new("MemberDefined1", "MemberData1"),
            KeyValuePascal::new("MemberDefined2", "MemberData2"),
        ]));
        member_list.push(member);

        let mut req = super::CreateGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_owner_account(Some("leckie"))
            .set_group_id(Some("MyFirstGroup"))
            .set_introduction(Some("This is group Introduction"))
            .set_notification(Some("This is group Notification"))
            .set_face_url(Some("http://this.is.face.url"))
            .set_max_member_count(Some(500))
            .set_apply_join_option(Some(ApplyJoinOption::FreeAccess))
            .set_app_defined_data(Some(vec![
                KeyValuePascal::new("GroupTestData1", "xxxxx"),
                KeyValuePascal::new("GroupTestData2", r#"abc\u0000\u0001"#),
            ]))
            .set_member_list(Some(member_list));

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "json not matched");
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "GroupId": "@TGS#2J4SZEAEL"
        });

        let res = serde_json::from_value::<super::CreateGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "GroupId": "@TGS#_@TGS#cRDH3HIM62CP",
          "HugeGroupFlag": 0,
          "Type": "Community"
        });

        let res = serde_json::from_value::<super::CreateGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response3() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "GroupId": "MyFirstGroup"
        });

        let res = serde_json::from_value::<super::CreateGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
