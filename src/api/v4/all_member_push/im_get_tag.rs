//! <https://www.tencentcloud.com/ko/document/product/1047/37172>

use crate::api::v4::common::{ActionStatus, ErrorCode, UserTag};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(im_get_attr, ImGetTagRequest, ImGetTagResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImGetTagRequest {
    #[serde(rename = "To_Account")]
    to_account: Vec<String>,
}

impl ImGetTagRequest {
    pub fn new<S: AsRef<str>>(to_account: Vec<S>) -> Self {
        Self {
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImGetTagResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UserTags")]
    pub user_tags: Option<Vec<UserTag>>,
}

#[cfg(test)]
mod test_im_get_tag {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "To_Account": [
                "xiaojun012",
                "xiaojun013"
            ]
        });

        let req = super::ImGetTagRequest::new(vec!["xiaojun012", "xiaojun013"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "UserTags": [
                {
                    "To_Account": "xiaojun012",
                    "Tags": ["a", "b"]
                },
                {
                    "To_Account": "xiaojun013",
                    "Tags": ["a", "c"]
                }
            ]
        });

        let res = serde_json::from_value::<super::ImGetTagResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
