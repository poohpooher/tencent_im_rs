//! <https://www.tencentcloud.com/ko/document/product/1047/34967>

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::{ActionStatus, ErrorCode, Group};
use crate::tencent_api;

tencent_api!(import_group, ImportGroupRequest, ImportGroupResponse);

pub type ImportGroupRequest = Group;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupResponse {
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
}

#[cfg(test)]
mod test_import_group {
    use serde_json::json;

    use crate::api::v4::common::apply_join_option::ApplyJoinOption;
    use crate::api::v4::common::group_type::GroupType;
    use crate::api::v4::common::KeyValuePascal;

    #[test]
    fn request1() {
        let sample = json!({
          "Owner_Account": "leckie", // User ID of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, or Community (required)
          "Name": "TestGroup", // Group name (required)
          "CreateTime": 1448357837 // Group creation time (optional). If this field is not specified, the default creation time is the request time.
        });

        let mut req = super::ImportGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_owner_account(Some("leckie"))
            .set_create_time(Some(1448357837));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "Type": "Community", // Group type (required)
          "Name": "test_import_group",  // Group name (required)
          "SupportTopic": 1// Whether the topic option is supported. Valid values: `1`: yes; `0`: no.
        });

        let mut req = super::ImportGroupRequest::new();
        req.set_group_type(Some(GroupType::Community))
            .set_name(Some("test_import_group"))
            .set_support_topic(Some(1));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "Owner_Account": "leckie", // User ID of the group owner (optional)
          "Type": "Public", // Group type: Private, Public, ChatRoom, or Community (required)
          "GroupId":"MyFirstGroup", // User-defined group ID for external display (optional)
          "Name": "TestGroup", // Group name (required)
          "Introduction": "This is group Introduction", // Group introduction (optional)
          "Notification": "This is group Notification", // Group notice (optional)
          "FaceUrl": "http://this.is.face.url",
          "MaxMemberCount": 500, // Maximum number of group members (optional)
          "ApplyJoinOption": "FreeAccess", // Method for handling requests to join the group (optional)
          "CreateTime": 1448357837, // Group creation time (optional). If this field is not specified, the default creation time is the request time.
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
        let mut req = super::ImportGroupRequest::new();
        req.set_group_type(Some(GroupType::Public))
            .set_name(Some("TestGroup"))
            .set_owner_account(Some("leckie"))
            .set_create_time(Some(1448357837))
            .set_group_id(Some("MyFirstGroup"))
            .set_introduction(Some("This is group Introduction"))
            .set_notification(Some("This is group Notification"))
            .set_face_url(Some("http://this.is.face.url"))
            .set_max_member_count(Some(500))
            .set_apply_join_option(Some(ApplyJoinOption::FreeAccess))
            .set_app_defined_data(Some(vec![
                KeyValuePascal::new("GroupTestData1", "xxxxx"),
                KeyValuePascal::new(" GroupTestData2", r#"abc\u0000\u0001"#),
            ]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "GroupId": "@TGS#2J4SZEAEL"
        });

        let res = serde_json::from_value::<super::ImportGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "GroupId": "@TGS#_@TGS#c4YHCIIM62CX"
        });

        let res = serde_json::from_value::<super::ImportGroupResponse>(sample.clone()).unwrap();

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

        let res = serde_json::from_value::<super::ImportGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
