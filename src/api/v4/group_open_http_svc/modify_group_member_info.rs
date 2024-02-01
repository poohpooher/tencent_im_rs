//! <https://www.tencentcloud.com/ko/document/product/1047/34900>

use crate::api::v4::common::{ActionStatus, ErrorCode, KeyValuePascal, MsgFlag};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    modify_group_member_info,
    ModifyGroupMemberInfoRequest,
    ModifyGroupMemberInfoResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyGroupMemberInfoRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Member_Account")]
    member_account: String,

    #[serde(rename = "Role")]
    role: Option<String>,

    #[serde(rename = "MsgFlag")]
    msg_flag: Option<MsgFlag>,

    #[serde(rename = "NameCard")]
    name_card: Option<String>,

    #[serde(rename = "AppMemberDefinedData")]
    app_member_defined_data: Option<Vec<KeyValuePascal>>,

    #[serde(rename = "MuteTime")]
    mute_time: Option<u32>,
}

impl ModifyGroupMemberInfoRequest {
    pub fn new<S: AsRef<str>>(group_id: S, member_account: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            member_account: member_account.as_ref().to_string(),
            role: None,
            msg_flag: None,
            name_card: None,
            app_member_defined_data: None,
            mute_time: None,
        }
    }

    pub fn set_role<S: AsRef<str>>(&mut self, role: Option<S>) -> &mut Self {
        self.role = role.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_msg_flag(&mut self, msg_flag: Option<MsgFlag>) -> &mut Self {
        self.msg_flag = msg_flag;
        self
    }

    pub fn set_name_card<S: AsRef<str>>(&mut self, name_card: Option<S>) -> &mut Self {
        self.name_card = name_card.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_app_member_defined_data(
        &mut self,
        app_member_defined_data: Option<Vec<KeyValuePascal>>,
    ) -> &mut Self {
        self.app_member_defined_data = app_member_defined_data;
        self
    }

    pub fn set_mute_time(&mut self, mute_time: Option<u32>) -> &mut Self {
        self.mute_time = mute_time;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyGroupMemberInfoResponse {
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
mod test_modify_group_member_info {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "Role": "Admin" // Set as an admin
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_role(Some("Admin"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "Role": "Member" //Cancel the admin role
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_role(Some("Member"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "MsgFlag": "AcceptAndNotify" // Message blocking type, which can be AcceptAndNotify, Discard, or AcceptNotNotify
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_msg_flag(Some(super::MsgFlag::AcceptAndNotify));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "NameCard": "bob" // Group name card (optional)
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_name_card(Some("bob"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request5() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "AppMemberDefinedData": [ // Target member custom field (optional)
              {
                  "Key":"MemberDefined1", // Key of the target group member custom field
                  "Value":"ModifyData1" // Value of the key
              },
              {
                  "Key":"MemberDefined3",
                  "Value":"ModifyData3"
              }
          ]
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_app_member_defined_data(Some(vec![
            super::KeyValuePascal::new("MemberDefined1", "ModifyData1"),
            super::KeyValuePascal::new("MemberDefined3", "ModifyData3"),
        ]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request6() {
        let sample = json!({
          "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
          "Member_Account": "bob",  // Target group member (required)
          "MuteTime":86400 // Muting period for the specified user, in seconds
        });

        let mut req = super::ModifyGroupMemberInfoRequest::new("@TGS#2CLUZEAEJ", "bob");
        req.set_mute_time(Some(86400));

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
            serde_json::from_value::<super::ModifyGroupMemberInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
