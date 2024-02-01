//! <https://www.tencentcloud.com/ko/document/product/1047/37167>

use crate::api::v4::common::{ActionStatus, AttributeName, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    im_set_attr_name,
    ImSetAttrNameRequest,
    ImSetAttrNameResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImSetAttrNameRequest {
    #[serde(rename = "AttrNames")]
    attr_names: AttributeName,
}

impl ImSetAttrNameRequest {
    pub fn new(attr_names: AttributeName) -> Self {
        Self { attr_names }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImSetAttrNameResponse {
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
mod test_im_set_attr_name {
    use crate::api::v4::common::AttributeName;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "AttrNames": {
                "0": "sex",
                "1": "city",
                "2": "country"
            }
        });

        let mut attr_names = AttributeName::new();
        attr_names.set_zero(Some("sex"));
        attr_names.set_one(Some("city"));
        attr_names.set_two(Some("country"));

        let req = super::ImSetAttrNameRequest::new(attr_names);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0
        });

        let res = serde_json::from_value::<super::ImSetAttrNameResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
