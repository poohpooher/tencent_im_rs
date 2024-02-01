//! <https://www.tencentcloud.com/ko/document/product/1047/37175>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    im_remove_all_tags,
    ImRemoveAllTagsRequest,
    ImRemoveAllTagsResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImRemoveAllTagsRequest {
    #[serde(rename = "To_Account")]
    to_account: Vec<String>,
}

impl ImRemoveAllTagsRequest {
    pub fn new<S: AsRef<str>>(to_account: Vec<S>) -> Self {
        Self {
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImRemoveAllTagsResponse {
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
mod test_im_remove_all_tags {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "To_Account": [
                "xiaojun012",
                "xiaojun013"
            ]
        });

        let req = super::ImRemoveAllTagsRequest::new(vec!["xiaojun012", "xiaojun013"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res = serde_json::from_value::<super::ImRemoveAllTagsResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
