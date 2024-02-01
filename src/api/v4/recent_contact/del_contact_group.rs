//! <https://www.tencentcloud.com/document/product/1047/53441>

use crate::api::v4::common::{ActionStatus, ErrorCode, GroupItem};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    del_contact_group,
    DelContactGroupRequest,
    DelContactGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DelContactGroupRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "GroupName")]
    group_name: Vec<String>,
}

impl DelContactGroupRequest {
    pub fn new<S: AsRef<str>>(from_account: S, group_name: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            group_name: group_name.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DelContactGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupItem")]
    pub group_item: Option<Vec<GroupItem>>,
}

#[cfg(test)]
mod test_del_contact_group {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"user15",
            "GroupName":["test0"]   // Currently, only one conversation group can be deleted at a time
        });

        let req = super::DelContactGroupRequest::new("user15", vec!["test0"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "GroupItem": [
                {
                    "GroupName": "test0",
                    "GroupId": 2
                }
            ],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": ""
        });

        let res = serde_json::from_value::<super::DelContactGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
