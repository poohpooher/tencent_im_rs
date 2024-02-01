//! <https://www.tencentcloud.com/ko/document/product/1047/38996>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    admin_set_msg_read,
    AdminSetMsgReadRequest,
    AdminSetMsgReadResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminSetMsgReadRequest {
    #[serde(rename = "Report_Account")]
    report_account: String,

    #[serde(rename = "Peer_Account")]
    peer_account: String,

    #[serde(rename = "MsgReadedTime")]
    msg_readed_time: Option<String>,
}

impl AdminSetMsgReadRequest {
    pub fn new<S: AsRef<str>>(report_account: S, peer_account: S) -> Self {
        Self {
            report_account: report_account.as_ref().to_string(),
            peer_account: peer_account.as_ref().to_string(),
            msg_readed_time: None,
        }
    }

    pub fn set_msg_readed_time<S: AsRef<str>>(&mut self, msg_readed_time: S) -> &mut Self {
        self.msg_readed_time = Some(msg_readed_time.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminSetMsgReadResponse {
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
mod test_admin_set_msg_read {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "Report_Account":"dramon1",
            "Peer_Account":"dramon2"
        });

        let req = super::AdminSetMsgReadRequest::new("dramon1", "dramon2");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0
        });

        let req = serde_json::from_value::<super::AdminSetMsgReadResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }
}
