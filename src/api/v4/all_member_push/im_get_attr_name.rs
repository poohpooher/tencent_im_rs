//! <https://www.tencentcloud.com/ko/document/product/1047/37168>

use crate::api::v4::common::{ActionStatus, AttributeName, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    im_get_attr_name,
    ImGetAttrNameRequest,
    ImGetAttrNameResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImGetAttrNameRequest {}

impl Default for ImGetAttrNameRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ImGetAttrNameRequest {
    pub fn new() -> Self {
        Self {}
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

    #[serde(rename = "AttrNames")]
    pub attr_names: Option<AttributeName>,
}

#[cfg(test)]
mod test_im_get_attr_name {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({});

        let req = super::ImGetAttrNameRequest::new();

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "AttrNames": {
                "0": "sex",
                "1": "city",
                "2": "Membership level"
            }
        });

        let res = serde_json::from_value::<super::ImGetAttrNameResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
