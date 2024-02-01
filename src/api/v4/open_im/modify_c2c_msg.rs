//! <https://www.tencentcloud.com/ko/document/product/1047/47722>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(modify_c2c_msg, ModifyC2CMsgRequest, ModifyC2CMsgResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyC2CMsgRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "MsgKey")]
    msg_key: String,

    #[serde(rename = "MsgBody")]
    msg_body: Option<Vec<MsgBody>>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,
}

impl ModifyC2CMsgRequest {
    pub fn new<S: AsRef<str>>(from_account: S, to_account: S, msg_key: S) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_account: to_account.as_ref().to_string(),
            msg_key: msg_key.as_ref().to_string(),
            msg_body: None,
            cloud_custom_data: None,
        }
    }

    pub fn set_msg_body(&mut self, msg_body: Option<Vec<MsgBody>>) -> &mut Self {
        self.msg_body = msg_body;
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
pub struct ModifyC2CMsgResponse {
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
mod test_modify_c2c_msg {
    use crate::api::v4::common::{MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account": "dramon1",
            "To_Account": "dramon2",
            "MsgKey": "1_2_3",
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hello"
                    }
                }
            ]
        }
        );

        let mut req = super::ModifyC2CMsgRequest::new("dramon1", "dramon2", "1_2_3");
        req.set_msg_body(Some(vec![MsgBody::new(
            MsgContent::str_text(),
            MsgContent::content_text("hello"),
        )]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "From_Account": "dramon1",
            "To_Account": "dramon2",
            "MsgKey": "1_2_3",
            "CloudCustomData": "your cloud custom data"
        }
        );

        let mut req = super::ModifyC2CMsgRequest::new("dramon1", "dramon2", "1_2_3");
        req.set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
            "From_Account": "dramon1",
            "To_Account": "dramon2",
            "MsgKey": "1_2_3",
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hello"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        }
        );

        let mut req = super::ModifyC2CMsgRequest::new("dramon1", "dramon2", "1_2_3");
        req.set_msg_body(Some(vec![MsgBody::new(
            MsgContent::str_text(),
            MsgContent::content_text("hello"),
        )]));
        req.set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "succeed"
        });

        let res = serde_json::from_value::<super::ModifyC2CMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
