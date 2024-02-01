//! <https://www.tencentcloud.com/document/product/1047/53427>

use crate::api::v4::common::{ActionStatus, ErrorCode, GroupCounter};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    get_group_counter,
    GetGroupCounterRequest,
    GetGroupCounterResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupCounterRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "GroupCounterKeys")]
    group_counter_keys: Option<Vec<String>>,
}

impl GetGroupCounterRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            group_counter_keys: None,
        }
    }

    pub fn set_group_counter_keys<S: AsRef<str>>(
        &mut self,
        group_counter_keys: Option<Vec<S>>,
    ) -> &mut Self {
        self.group_counter_keys = group_counter_keys.map(|v| {
            v.into_iter()
                .map(|s| s.as_ref().to_string())
                .collect::<Vec<_>>()
        });
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupCounterResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupCounter")]
    pub group_counter: Option<Vec<GroupCounter>>,
}

#[cfg(test)]
mod test_get_group_counter {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#aC5SZEAEF",
            "GroupCounterKeys":[    // List of group counter keys to get. If no value is passed in, all group counter keys will be returned.
                "like",
                "unlike"
            ]
        });

        let mut req = super::GetGroupCounterRequest::new("@TGS#aC5SZEAEF");
        req.set_group_counter_keys(Some(vec!["like", "unlike"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "GroupCounter":[
                {
                    "Key":"like", // Group counter key
                    "Value":5 // Latest group counter value
                },
                {
                    "Key":"unlike",
                    "Value":1
                }
            ]
        });

        let res = serde_json::from_value::<super::GetGroupCounterResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
