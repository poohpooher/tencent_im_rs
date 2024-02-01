//! <https://www.tencentcloud.com/ko/document/product/1047/34962>

use crate::api::v4::common::{ActionStatus, ErrorCode, Group};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    modify_group_base_info,
    ModifyGroupBaseInfoRequest,
    ModifyGroupBaseInfoResponse
);

pub type ModifyGroupBaseInfoRequest = Group;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyGroupBaseInfoResponse {
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
mod test_modify_group_base_info {
    use crate::api::v4::common::{ApplyJoinOption, KeyValuePascal};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#1NVTZEAE4", // Group whose basic profile you want to modify (required)
          "Name": "NewName", // Group name (optional)
          "Introduction": "NewIntroduction", // Group introduction (optional)
          "Notification": "NewNotification", // Group notice (optional)
          "FaceUrl": "http://this.is.new.face.url", // Group profile photo (optional)
          "MaxMemberNum": 500, // Maximum number of group members (optional)
          "ApplyJoinOption": "NeedPermission", // Method for applying to join the group (optional)
          "MuteAllMember": "On" // Mutes all members (optional). "On": Enable, "Off": Disable
        });

        let mut req = super::ModifyGroupBaseInfoRequest::new();
        req.set_group_id(Some("@TGS#1NVTZEAE4"))
            .set_name(Some("NewName"))
            .set_introduction(Some("NewIntroduction"))
            .set_notification(Some("NewNotification"))
            .set_face_url(Some("http://this.is.new.face.url"))
            .set_max_member_num(Some(500))
            .set_apply_join_option(Some(ApplyJoinOption::NeedPermission))
            .set_mute_all_member(Some("On"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#1NVTZEAE4", // Group whose basic profile you want to modify (required)
          "AppDefinedData": [ // Custom field (optional)
              {
                  "Key": "GroupTestData1", // Custom field key to be modified
                  "Value": "NewData"  // New value of the custom field
              }
          ]
        });

        let mut req = super::ModifyGroupBaseInfoRequest::new();
        req.set_group_id(Some("@TGS#1NVTZEAE4"))
            .set_app_defined_data(Some(vec![KeyValuePascal::new("GroupTestData1", "NewData")]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "GroupId": "@TGS#1NVTZEAE4", // Group whose basic profile you want to modify (required)
          "AppDefinedData": [  // Custom field (optional)
              {
                  "Key": "GroupTestData2",
                  "Value": "" // If this parameter is empty, the custom field is to be deleted.
              }
          ]
        });

        let mut req = super::ModifyGroupBaseInfoRequest::new();
        req.set_group_id(Some("@TGS#1NVTZEAE4"))
            .set_app_defined_data(Some(vec![KeyValuePascal::new("GroupTestData2", "")]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "GroupId": "@TGS#2J4SZEAEL", // Group whose basic profile you want to modify (required)
          "Name": "NewName", // Group name (optional)
          "Introduction": "NewIntroduction", // Group introduction (optional)
          "Notification": "NewNotification", // Group notice (optional)
          "FaceUrl": "http://this.is.new.face.url", // Group profile photo (optional)
          "MaxMemberNum": 500, // Maximum number of group members (optional)
          "ApplyJoinOption": "NeedPermission", // Method for applying to join the group (optional)
          "MuteAllMember": "On", // Mutes all members, which is optional. `On`: Enable. `Off`: Disable
          "AppDefinedData": [ // Custom field (optional)
              {
                  "Key": "GroupTestData1", // Custom field key to be modified
                  "Value": "NewData" // New value of the custom field
              },
              {
                  "Key": "GroupTestData2",
                  "Value": "" // If this parameter is empty, the custom field is to be deleted.
              }
          ]
        });

        let mut req = super::ModifyGroupBaseInfoRequest::new();
        req.set_group_id(Some("@TGS#2J4SZEAEL"))
            .set_name(Some("NewName"))
            .set_introduction(Some("NewIntroduction"))
            .set_notification(Some("NewNotification"))
            .set_face_url(Some("http://this.is.new.face.url"))
            .set_max_member_num(Some(500))
            .set_apply_join_option(Some(ApplyJoinOption::NeedPermission))
            .set_mute_all_member(Some("On"))
            .set_app_defined_data(Some(vec![
                KeyValuePascal::new("GroupTestData1", "NewData"),
                KeyValuePascal::new("GroupTestData2", ""),
            ]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res =
            serde_json::from_value::<super::ModifyGroupBaseInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
