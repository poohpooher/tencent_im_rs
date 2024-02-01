//! <https://www.tencentcloud.com/ko/document/product/1047/35015>

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;

tencent_api!(
    admin_msgwithdraw,
    AdminMsgWithdrawRequest,
    AdminMsgWithdrawResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminMsgWithdrawRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "MsgKey")]
    msg_key: String,
}

impl AdminMsgWithdrawRequest {
    pub fn new<S: AsRef<str>>(from_account: S, to_account: S, msg_key: S) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_account: to_account.as_ref().to_string(),
            msg_key: msg_key.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminMsgWithdrawResponse {
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
mod test_admin_msg_withdraw {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account": "vinson",
            "To_Account": "dramon",
            "MsgKey": "31906_833502_1572869830"
        });

        let req =
            super::AdminMsgWithdrawRequest::new("vinson", "dramon", "31906_833502_1572869830");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0
        });

        let res =
            serde_json::from_value::<super::AdminMsgWithdrawResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "FAIL",
          "ErrorInfo": "Fail to Parse json data of body, Please check it",
          "ErrorCode": 90001
        });

        let res =
            serde_json::from_value::<super::AdminMsgWithdrawResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
