//! <https://www.tencentcloud.com/ko/document/product/1047/37169>

use crate::api::v4::common::{ActionStatus, ErrorCode, UserAttribute};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    im_get_attr_name,
    ImGetAttrNameRequest,
    ImGetAttrNameResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImGetAttrNameRequest {
    #[serde(rename = "To_Account")]
    to_account: Vec<String>,
}

impl ImGetAttrNameRequest {
    pub fn new<S: AsRef<str>>(to_account: Vec<S>) -> Self {
        Self {
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImGetAttrNameResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UserAttrs")]
    pub user_attrs: Option<Vec<UserAttribute>>,
}

#[cfg(test)]
mod test_im_get_attr {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "To_Account": [
                "Zhang Xiaohong",
                "Chen Xiaoming",
                "abc"
            ]
        });

        let req = super::ImGetAttrNameRequest::new(vec!["Zhang Xiaohong", "Chen Xiaoming", "abc"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "UserAttrs": [
                {
                    "To_Account": "Zhang Xiaohong",
                    "Attrs": {
                        "sex" : "Female",
                        "city": "New York"
                    }
                },
                {
                    "To_Account": "abc",
                    "Attrs": {}
                },
                {
                    "To_Account": "Chen Xiaoming",
                    "Attrs": {
                        "sex": "M",
                        "city": "Shenzhen"
                    }
                }
            ]
        });

        let res = serde_json::from_value::<super::ImGetAttrNameResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
