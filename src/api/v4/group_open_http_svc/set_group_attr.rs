//! <https://www.tencentcloud.com/document/product/1047/44190>

use crate::api::v4::common::{ActionStatus, ErrorCode, KeyValueLower};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(set_group_attr, SetGroupAttrRequest, SetGroupAttrResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetGroupAttrRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "GroupAttr")]
    group_attribute: Vec<KeyValueLower>,
}

impl SetGroupAttrRequest {
    pub fn new<S: AsRef<str>>(group_id: S, group_attribute: Vec<KeyValueLower>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            group_attribute,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetGroupAttrResponse {
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
mod test_set_group_attr {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#aC5SZEAEF",
          "GroupAttr":[
              {
                  "key":"attr_key", //Attribute key
                  "value":"attr_val" //Attribute value
              },
              {
                  "key":"attr_key1",
                  "value":"attr_val1"
              }
          ]
        });

        let req = super::SetGroupAttrRequest::new(
            "@TGS#aC5SZEAEF",
            vec![
                super::KeyValueLower::new("attr_key", "attr_val"),
                super::KeyValueLower::new("attr_key1", "attr_val1"),
            ],
        );

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res = serde_json::from_value::<super::SetGroupAttrResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
