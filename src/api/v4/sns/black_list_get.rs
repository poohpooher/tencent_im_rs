//! <https://www.tencentcloud.com/ko/document/product/1047/34621#profile-management>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(black_list_get, BlackListGetRequest, BlackListGetResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListGetRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "StartIndex")]
    start_index: u32,

    #[serde(rename = "MaxLimited")]
    max_limited: u32,

    #[serde(rename = "LastSequence")]
    last_sequence: u32,
}

impl BlackListGetRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        start_index: u32,
        max_limited: u32,
        last_sequence: u32,
    ) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            start_index,
            max_limited,
            last_sequence,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListGetResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "BlackListItem")]
    pub black_list_items: Option<Vec<BlackListItem>>,

    #[serde(rename = "StartIndex")]
    pub start_index: Option<u32>,

    /// Tencent IM 에서 오타난 채로 전송되어 json 변환시 오타 그대로 사용
    #[serde(rename = "CurruentSequence")]
    pub current_sequence: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "AddBlackTimeStamp")]
    add_black_time_stamp: u64,
}

#[cfg(test)]
mod test_black_list_get {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account": "id",
            "StartIndex": 0,
            "MaxLimited": 30,
            "LastSequence": 12
        });

        let req = super::BlackListGetRequest::new("id", 0, 30, 12);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "BlackListItem": [
                {
                    "To_Account": "id1",
                    "AddBlackTimeStamp": 1430000001
                },
                {
                    "To_Account": "id2",
                    "AddBlackTimeStamp": 1430000002
                }
            ],
            "StartIndex": 0,
            "CurruentSequence": 13,
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::BlackListGetResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
