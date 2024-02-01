//! <https://www.tencentcloud.com/ko/document/product/1047/37173>

use crate::api::v4::common::{ActionStatus, ErrorCode, UserTag};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(im_add_tag, ImAddTagRequest, ImAddTagResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImAddTagRequest {
    #[serde(rename = "UserTags")]
    user_tags: Vec<UserTag>,
}

impl ImAddTagRequest {
    pub fn new(user_tags: Vec<UserTag>) -> Self {
        Self { user_tags }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImAddTagResponse {
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
mod test_im_add_tag {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "UserTags": [
                {
                    "To_Account": "xiaojun012",
                    "Tags": ["a", "b"]
                },
                {
                    "To_Account": "xiaojun013",
                    "Tags": ["a", "b"]
                }
            ]
        });

        let req = super::ImAddTagRequest::new(vec![
            super::UserTag::new("xiaojun012", vec!["a", "b"]),
            super::UserTag::new("xiaojun013", vec!["a", "b"]),
        ]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0
        });

        let res = serde_json::from_value::<super::ImAddTagResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
