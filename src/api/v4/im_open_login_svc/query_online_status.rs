//! <https://www.tencentcloud.com/ko/document/product/1047/35477>

use crate::api::v4::common::{ActionStatus, ConnectState, ErrorCode, Platform};
use crate::api::{bool_to_int, int_to_bool};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    query_online_status,
    QueryOnlineStatusRequest,
    QueryOnlineStatusResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryOnlineStatusRequest {
    #[serde(rename = "To_Account")]
    to_accounts: Vec<String>,

    #[serde(
        rename = "IsNeedDetail",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    is_need_detail: Option<bool>,
}

impl QueryOnlineStatusRequest {
    pub fn new<S: AsRef<str>>(to_accounts: Vec<S>) -> Self {
        Self {
            to_accounts: to_accounts.iter().map(|s| s.as_ref().to_string()).collect(),
            is_need_detail: None,
        }
    }

    pub fn set_is_need_detail(&mut self, is_need_detail: Option<bool>) -> &mut Self {
        self.is_need_detail = is_need_detail;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryOnlineStatusResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "QueryResult")]
    pub query_result: Option<Vec<QueryResult>>,

    #[serde(rename = "ErrorList")]
    pub error_list: Option<Vec<ErrorItem>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResult {
    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "State")]
    pub state: ConnectState,

    #[serde(rename = "Detail")]
    pub detail: Option<Vec<QueryResultDetail>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResultDetail {
    #[serde(rename = "Platform")]
    pub platform: Platform,

    #[serde(rename = "Status")]
    pub status: ConnectState,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorItem {
    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "ErrorCode")]
    pub error_code: ErrorCode,
}

#[cfg(test)]
mod test_query_online_status {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "To_Account":["id1","id2","id3","id4"],
        }
        );

        let req = super::QueryOnlineStatusRequest::new(vec!["id1", "id2", "id3", "id4"]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn request2() {
        let sample = json!({
            "IsNeedDetail": 1,
            "To_Account": ["id1", "id2", "id4"]
        });

        let mut req = super::QueryOnlineStatusRequest::new(vec!["id1", "id2", "id4"]);
        req.set_is_need_detail(Some(true));

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus":"OK",
            "ErrorInfo":"",
            "ErrorCode": 0,
            "QueryResult": [
                {
                    "To_Account": "id1",
                    "State": "Offline"
                },
                {
                    "To_Account": "id2",
                    "State": "Online"
                },
                {
                    "To_Account": "id3",
                    "State": "PushOnline"
                }
            ],
            "ErrorList": [
                {
                    "To_Account": "id4",
                    "ErrorCode": 70107
                }
            ]
        }
        );

        let res =
            serde_json::from_value::<super::QueryOnlineStatusResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }

    #[test]
    fn response2() {
        let sample = json!({
            "ActionStatus":"OK",
            "ErrorInfo":"",
            "ErrorCode": 0,
            "QueryResult": [
                {
                    "To_Account": "id1",
                    "State": "Online",
                    "Detail": [
                        {
                            "Platform": "IPhone",
                            "Status": "PushOnline"
                        },
                        {
                            "Platform": "Web",
                            "Status": "Online"
                        }
                    ]
                },
                {
                    "To_Account": "id2",
                    "State": "Offline",
                }
            ],
            "ErrorList": [
                {
                    "To_Account": "id4",
                    "ErrorCode": 70107
                }
            ]
        });

        let res =
            serde_json::from_value::<super::QueryOnlineStatusResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }

    #[test]
    fn response3() {
        let sample = json!({
            "ActionStatus": "FAIL",
            "ErrorInfo": "Fail to Parse json data of body, Please check it",
            "ErrorCode": 90001
        });

        let res =
            serde_json::from_value::<super::QueryOnlineStatusResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
