//! <https://www.tencentcloud.com/document/product/1047/34926>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(group_delete, GroupDeleteRequest, GroupDeleteResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupDeleteRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "GroupName")]
    group_names: Vec<String>,
}

impl GroupDeleteRequest {
    pub fn new<S: AsRef<str>>(from_account: S, group_names: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            group_names: group_names.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupDeleteResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "CurrentSequence")]
    pub current_sequence: Option<u64>,
}

#[cfg(test)]
mod test_group_delete {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id",
            "GroupName":["group1","group2","group3"]
        });

        let req = super::GroupDeleteRequest::new("id", vec!["group1", "group2", "group3"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "CurrentSequence": 4,
            "ActionStatus":"OK",
            "ErrorCode":0,
            "ErrorInfo":"0",
            "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::GroupDeleteResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
