//! <https://www.tencentcloud.com/ko/document/product/1047/37170>

use crate::api::v4::common::{ActionStatus, ErrorCode, UserAttribute};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(im_set_attr, ImSetAttrRequest, ImSetAttrResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImSetAttrRequest {
    #[serde(rename = "UserAttrs")]
    user_attrs: Vec<UserAttribute>,
}

impl ImSetAttrRequest {
    pub fn new(user_attrs: Vec<UserAttribute>) -> Self {
        Self { user_attrs }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImSetAttrResponse {
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
mod test_im_set_attr {
    use crate::api::v4::common::UserAttribute;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "UserAttrs":
            [
                {
                    "To_Account": "xiaojun012",
                    "Attrs": {
                        "sex": "attr1",
                        "city": "attr2"
                    }
                },
                {
                    "To_Account": "xiaojun013",
                    "Attrs": {
                        "city": "attr3",
                        "sex": "attr4"
                    }
                }
            ]
        });

        let req = super::ImSetAttrRequest::new(vec![
            UserAttribute::new(
                "xiaojun012",
                vec![("sex", "attr1"), ("city", "attr2")]
                    .into_iter()
                    .collect(),
            ),
            UserAttribute::new(
                "xiaojun013",
                vec![("city", "attr3"), ("sex", "attr4")]
                    .into_iter()
                    .collect(),
            ),
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

        let res = serde_json::from_value::<super::ImSetAttrResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
