//! <https://www.tencentcloud.com/ko/document/product/1047/41046>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_c2c_unread_msg_num,
    GetC2CUnreadMsgNumRequest,
    GetC2CUnreadMsgNumResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetC2CUnreadMsgNumRequest {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "Peer_Account")]
    peer_account: Option<Vec<String>>,
}

impl GetC2CUnreadMsgNumRequest {
    pub fn new<S: AsRef<str>>(to_account: S) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            peer_account: None,
        }
    }

    pub fn set_peer_account<S: AsRef<str>>(&mut self, peer_account: Option<Vec<S>>) -> &mut Self {
        self.peer_account =
            peer_account.map(|v| v.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetC2CUnreadMsgNumResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "AllC2CUnreadMsgNum")]
    pub all_c2c_unread_msg_num: Option<u32>,

    #[serde(rename = "C2CUnreadMsgNumList")]
    pub c2c_unread_msg_num_list: Option<Vec<C2CUnreadMsg>>,

    #[serde(rename = "ErrorList")]
    pub error_list: Option<Vec<C2CUnreadError>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct C2CUnreadMsg {
    #[serde(rename = "Peer_Account")]
    pub peer_account: String,

    #[serde(rename = "C2CUnreadMsgNum")]
    pub c2c_unread_msg_num: u32,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct C2CUnreadError {
    #[serde(rename = "Peer_Account")]
    pub peer_account: String,

    #[serde(rename = "ErrorCode")]
    pub error_code: ErrorCode,
}

#[cfg(test)]
mod test_get_c2c_unread_msg_num {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "To_Account":"dramon1"
        });

        let req = super::GetC2CUnreadMsgNumRequest::new("dramon1");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "To_Account":"dramon1",
            "Peer_Account":[
                "dramon2",
                "teacher"
            ]
        });

        let mut req = super::GetC2CUnreadMsgNumRequest::new("dramon1");
        req.set_peer_account(Some(vec!["dramon2", "teacher"]));

        assert_eq!(serde_json::to_value(&req).unwrap(), sample);
    }

    #[test]
    fn response() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "C2CUnreadMsgNumList": [
                {
                    "Peer_Account": "dramon2",
                    "C2CUnreadMsgNum": 12
                },
                {
                    "Peer_Account": "teacher",
                    "C2CUnreadMsgNum": 12
                }
            ]
        });

        let res =
            serde_json::from_value::<super::GetC2CUnreadMsgNumResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
