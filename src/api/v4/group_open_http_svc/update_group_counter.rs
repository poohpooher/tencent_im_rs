//! <https://www.tencentcloud.com/document/product/1047/53428>

use crate::api::v4::common::{ActionStatus, ErrorCode, GroupCounter, GroupCounterMode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    update_group_counter,
    UpdateGroupCounterRequest,
    UpdateGroupCounterResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupCounterRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "GroupCounter")]
    group_counter: Vec<GroupCounter>,

    #[serde(rename = "Mode")]
    mode: Option<GroupCounterMode>,
}

impl UpdateGroupCounterRequest {
    pub fn new<S: AsRef<str>>(group_id: S, group_counter: Vec<GroupCounter>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            group_counter,
            mode: None,
        }
    }

    pub fn set_mode(&mut self, mode: Option<GroupCounterMode>) -> &mut Self {
        self.mode = mode;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupCounterResponse {
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
mod test_update_group_counter {
    use crate::api::v4::common::GroupCounter;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#aC5SZEAEF",
            "GroupCounter":[
                {
                    "Key":"like", // Group counter key
                    "Value":5 // Group counter value
                },
                {
                    "Key":"unlike",
                    "Value":1
                }
            ],
            "Mode": "Set"  // `Set`, `Increase`, `Decrease`
        });

        let mut req = super::UpdateGroupCounterRequest::new(
            "@TGS#aC5SZEAEF",
            vec![GroupCounter::new("like", 5), GroupCounter::new("unlike", 1)],
        );
        req.set_mode(Some(super::GroupCounterMode::Set));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "GroupCounter":[  // List of the latest group counter key-value pairs updated
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

        let res =
            serde_json::from_value::<super::UpdateGroupCounterResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
