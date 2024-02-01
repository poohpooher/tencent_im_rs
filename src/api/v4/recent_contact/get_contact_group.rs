//! <https://www.tencentcloud.com/document/product/1047/53440>

use crate::api::v4::common::{ActionStatus, ContactItem, ErrorCode, GroupItem};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    get_contact_group,
    GetContactGroupRequest,
    GetContactGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetContactGroupRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "StartIndex")]
    start_index: u32,
}

impl GetContactGroupRequest {
    pub fn new<S: AsRef<str>>(from_account: S, start_index: u32) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            start_index,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetContactGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ContactItem")]
    pub contact_item: Option<Vec<ContactItem>>,

    #[serde(rename = "GroupItem")]
    pub group_item: Option<Vec<GroupItem>>,

    #[serde(rename = "CompleteFlag")]
    pub complete_flag: Option<u32>,

    #[serde(rename = "NextStartIndex")]
    pub next_start_index: Option<u32>,
}

#[cfg(test)]
mod test_get_contact_group {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"user0",
            "StartIndex":0
        });

        let req = super::GetContactGroupRequest::new("user0", 0);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ContactItem": [
                {
                    "Type": 1,
                    "To_Account": "teacher4",
                    "StandardMark": "111001",
                    "ContactGroupId": [
                        1,
                        2
                    ],
                    "Timestamp": 1670843110
                },
                {
                    "Type": 2,
                    "ToGroupId": "@TGS#1N3RSUYG2",
                    "StandardMark": "111001",
                    "CustomMark": "abcd",
                    "Timestamp": 1672998266
                }
            ],
            "GroupItem": [
                {
                    "GroupName": "test1x",
                    "GroupId": 1
                },
                {
                    "GroupName": "test10",
                    "GroupId": 2
                }
            ],
            "CompleteFlag": 1,
            "NextStartIndex": 0,
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": ""
        });

        let res = serde_json::from_value::<super::GetContactGroupResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
