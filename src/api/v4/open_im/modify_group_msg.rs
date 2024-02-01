//! <https://www.tencentcloud.com/document/product/1047/47948>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    modify_group_msg,
    ModifyGroupMsgRequest,
    ModifyGroupMsgResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyGroupMsgRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "MsgSeq")]
    msg_seq: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Option<Vec<MsgBody>>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl ModifyGroupMsgRequest {
    pub fn new<S: AsRef<str>>(group_id: S, msg_seq: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            msg_seq,
            msg_body: None,
            cloud_custom_data: None,
            topic_id: None,
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

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyGroupMsgResponse {
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
mod test_modify_group_msg {
    use crate::api::v4::common::{MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#1HYEP2SHC",
            "MsgSeq": 23,
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

        let mut req = super::ModifyGroupMsgRequest::new("@TGS#1HYEP2SHC", 23);
        req.set_msg_body(Some(vec![MsgBody::new(
            MsgContent::str_text(),
            MsgContent::content_text("hello"),
        )]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "GroupId": "@TGS#1HYEP2SHC",
            "MsgSeq": 23,
            "CloudCustomData": "your cloud custom data"
        }
        );

        let mut req = super::ModifyGroupMsgRequest::new("@TGS#1HYEP2SHC", 23);
        req.set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
            "GroupId": "@TGS#1HYEP2SHC",
            "MsgSeq": 23,
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

        let mut req = super::ModifyGroupMsgRequest::new("@TGS#1HYEP2SHC", 23);
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

        let res = serde_json::from_value::<super::ModifyGroupMsgResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
