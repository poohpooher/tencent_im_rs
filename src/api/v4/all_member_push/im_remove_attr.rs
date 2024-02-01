//! <https://www.tencentcloud.com/ko/document/product/1047/37171>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(im_remove_attr, ImRemoveAttrRequest, ImRemoveAttrResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImRemoveAttrRequest {
    #[serde(rename = "UserAttrs")]
    user_attrs: Vec<RemoveAttribute>,
}

impl ImRemoveAttrRequest {
    pub fn new(user_attrs: Vec<RemoveAttribute>) -> Self {
        Self { user_attrs }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveAttribute {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "Attrs")]
    attrs: Vec<String>,
}
impl RemoveAttribute {
    pub fn new<S: AsRef<str>>(to_account: S, attrs: Vec<S>) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            attrs: attrs.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImRemoveAttrResponse {
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
mod test_im_remove_attr {
    use serde_json::json;

    use crate::api::v4::all_member_push::im_remove_attr::RemoveAttribute;

    #[test]
    fn request1() {
        let sample = json!({
            "UserAttrs": [
                {
                    "To_Account": "xiaojun013",
                    "Attrs": [
                        "sex",
                        "city"
                    ]
                },
                {
                    "To_Account": "xiaojun012",
                    "Attrs": [
                        "sex",
                        "city"
                    ]
                }
            ]
        });

        let req = super::ImRemoveAttrRequest::new(vec![
            RemoveAttribute::new("xiaojun013", vec!["sex", "city"]),
            RemoveAttribute::new("xiaojun012", vec!["sex", "city"]),
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

        let res = serde_json::from_value::<super::ImRemoveAttrResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
