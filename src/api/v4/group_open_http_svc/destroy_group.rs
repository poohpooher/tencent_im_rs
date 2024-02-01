//! <https://www.tencentcloud.com/document/product/1047/34896>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(destroy_group, DestroyGroupRequest, DestroyGroupResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DestroyGroupRequest {
    #[serde(rename = "GroupId")]
    group_id: String,
}

impl DestroyGroupRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DestroyGroupResponse {
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
mod test_destroy_group {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2J4SZEAEL"
        });

        let req = super::DestroyGroupRequest::new("@TGS#2J4SZEAEL");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0
        });

        let res = serde_json::from_value::<super::DestroyGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
