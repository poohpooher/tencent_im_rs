//! <https://www.tencentcloud.com/ko/document/product/1047/44187>

use crate::api::v4::common::{ActionStatus, ErrorCode, KeyValueLower};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(get_group_att, GetGroupAttRequest, GetGroupAttResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupAttRequest {
    #[serde(rename = "GroupId")]
    group_id: String,
}

impl GetGroupAttRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupAttResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupAttrAry")]
    pub group_attr_ary: Option<Vec<KeyValueLower>>,
}

#[cfg(test)]
mod test_get_group_attr {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#aC5SZEAEF"
        });

        let req = super::GetGroupAttRequest::new("@TGS#aC5SZEAEF");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "GroupAttrAry": [
                {
                    "key": "attr_key1",
                    "value": "attr_val1"
                },
                {
                    "key": "attr_key2",
                    "value": "attr_val2"
                }
            ]
        });

        let res = serde_json::from_value::<super::GetGroupAttResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
