//! <https://www.tencentcloud.com/ko/document/product/1047/43087>

use crate::api::v4::common::{
    ActionStatus, AssistFlag, CompleteFlag, ConversationType, ErrorCode, TopFlag,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(get_list, GetListRequest, GetListResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetListRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "TimeStamp")]
    timestamp: u64,

    #[serde(rename = "StartIndex")]
    start_index: u32,

    #[serde(rename = "TopTimeStamp")]
    top_timestamp: u64,

    #[serde(rename = "TopStartIndex")]
    top_start_index: u32,

    #[serde(rename = "AssistFlags")]
    assist_flags: AssistFlag,
}

impl GetListRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        timestamp: u64,
        start_index: u32,
        top_timestamp: u64,
        top_start_index: u32,
        assist_flags: AssistFlag,
    ) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            timestamp,
            start_index,
            top_timestamp,
            top_start_index,
            assist_flags,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetListResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "SessionItem")]
    pub session_item: Option<Vec<SessionItem>>,

    #[serde(rename = "CompleteFlag")]
    pub complete_flag: Option<CompleteFlag>,

    #[serde(rename = "TimeStamp")]
    pub timestamp: Option<u64>,

    #[serde(rename = "StartIndex")]
    pub start_index: Option<u32>,

    #[serde(rename = "TopTimeStamp")]
    pub top_timestamp: Option<u64>,

    #[serde(rename = "TopStartIndex")]
    pub top_start_index: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionItem {
    #[serde(rename = "Type")]
    conversation_type: ConversationType,

    #[serde(rename = "To_Account")]
    to_account: Option<String>,

    #[serde(rename = "GroupId")]
    group_id: Option<String>,

    #[serde(rename = "MsgTime")]
    msg_time: u64,

    #[serde(rename = "TopFlag")]
    top_flag: TopFlag,
}

#[cfg(test)]
mod test_get_list {
    use crate::api::v4::common::AssistFlag;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account": "id1",
          "TimeStamp": 0,
          "StartIndex": 0,
          "TopTimeStamp": 0,
          "TopStartIndex": 0,
          "AssistFlags": 7
        });

        let req = super::GetListRequest::new(
            "id1",
            0,
            0,
            0,
            0,
            AssistFlag::PINNED | AssistFlag::EMPTY | AssistFlag::PAGINATING_PINNED,
        );

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "SessionItem": [
            {
              "Type": 1,
              "To_Account": "id2",
              "MsgTime": 1630997627,
              "TopFlag": 1
            },
            {
              "Type": 2,
              "GroupId": "id3",
              "MsgTime": 1630997628,
              "TopFlag": 1
            },
            {
              "Type": 1,
              "To_Account": "id4",
              "MsgTime": 1630997630,
              "TopFlag": 0
            },
            {
              "Type": 2,
              "GroupId": "id5",
              "MsgTime": 1630997650,
              "TopFlag": 0
            }
          ],
          "CompleteFlag": 1,
          "TimeStamp": 1631012800,
          "StartIndex": 0,
          "TopTimeStamp": 1631012800,
          "TopStartIndex": 0,
          "ActionStatus": "OK",
          "ErrorCode": 0,
          "ErrorInfo": "",
          "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::GetListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
