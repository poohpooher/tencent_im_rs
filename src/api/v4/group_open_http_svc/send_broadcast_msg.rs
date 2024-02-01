//! <https://www.tencentcloud.com/document/product/1047/49440>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    send_broadcast_msg,
    SendBroadcastMsgRequest,
    SendBroadcastMsgResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendBroadcastMsgRequest {
    #[serde(rename = "From_Account")]
    from_account: Option<String>,

    #[serde(rename = "Random")]
    random: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,
}

impl SendBroadcastMsgRequest {
    pub fn new(random: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            from_account: None,
            random,
            msg_body,
            cloud_custom_data: None,
        }
    }

    pub fn set_from_account<S: AsRef<str>>(&mut self, from_account: Option<S>) -> &mut Self {
        self.from_account = from_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_cloud_custom_data<S: AsRef<str>>(
        &mut self,
        cloud_custom_data: Option<S>,
    ) -> &mut Self {
        self.cloud_custom_data = cloud_custom_data.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendBroadcastMsgResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "MsgSeq")]
    pub msg_seq: Option<u32>,
}

#[cfg(test)]
mod test_send_broadcast_msg {
    use crate::api::v4::common::{MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account": "test",  // Specify the message sender (optional)
            "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
            "MsgBody": [
                {
                    "MsgType": "TIMCustomElem", // Custom message
                    "MsgContent": {
                        "Data": "{ \"type\":1, \"content\":\"hello world\"}"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::SendBroadcastMsgRequest::new(
            8912345,
            vec![MsgBody::new(
                MsgContent::str_custom(),
                MsgContent::content_custom(
                    "{ \"type\":1, \"content\":\"hello world\"}",
                    None,
                    None,
                    None,
                ),
            )],
        );
        req.set_from_account(Some("test"));
        req.set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "MsgSeq": 1283
        });

        let res =
            serde_json::from_value::<super::SendBroadcastMsgResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
