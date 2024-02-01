//! <https://www.tencentcloud.com/ko/document/product/1047/34968>

use crate::api::v4::common::msg::Msg;
use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::api::{bool_to_int, int_to_bool};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    import_group_msg,
    ImportGroupMsgRequest,
    ImportGroupMsgResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupMsgRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(
        rename = "RecentContactFlag",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    recent_contact_flag: Option<bool>,

    #[serde(rename = "MsgList")]
    msg_list: Vec<Msg>,
}

impl ImportGroupMsgRequest {
    pub fn new<S: AsRef<str>>(group_id: S, msg_list: Vec<Msg>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            recent_contact_flag: None,
            msg_list,
        }
    }

    pub fn set_recent_contact_flag(&mut self, recent_contact_flag: Option<bool>) -> &mut Self {
        self.recent_contact_flag = recent_contact_flag;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportGroupMsgResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ImportMsgResult")]
    pub import_msg_result: Vec<ImportMsg>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportMsg {
    #[serde(rename = "MsgSeq")]
    pub msg_seq: u32,

    #[serde(rename = "MsgTime")]
    pub msg_time: u32,

    #[serde(rename = "Result")]
    pub msg_result: ErrorCode,
}

#[cfg(test)]
mod test_import_group_msg {
    use crate::api::v4::common::{Msg, MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2C5SZEAEF",
            "RecentContactFlag":1,// Means to trigger conversation update (This field is not supported by AVChatRoom groups.)
            "MsgList": [
                {
                    "From_Account": "leckie", // Message sender
                    "SendTime":1620808101,
                    "Random": 8912345, // Random number of the message (optional)
                    "MsgBody": [ // Message body, which consists of an element array. For details, see the `TIMMessage` message object.
                        {
                            "MsgType": "TIMTextElem", // Text
                            "MsgContent": {
                                "Text": "red packet"
                            }
                        },
                        {
                            "MsgType": "TIMFaceElem", // Emoji
                            "MsgContent": {
                                "Index": 6,
                                "Data": r#"abc\u0000\u0001"#
                            }
                        }
                    ]
                },
                {
                    "From_Account": "peter", // Message sender
                    "SendTime":1620892821,
                    "MsgBody": [ // Message body, which consists of an element array. For details, see the `TIMMessage` message object.
                        {
                            "MsgType": "TIMTextElem", // Text
                            "MsgContent": {
                                "Text": "red packet"
                            }
                        }
                    ]
                }

            ]
        });

        let mut msg_list = vec![];
        let body_list = vec![
            MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("red packet"),
            ),
            MsgBody::new(
                MsgContent::str_face(),
                MsgContent::content_face(6, r#"abc\u0000\u0001"#),
            ),
        ];

        let mut msg = Msg::new("leckie", 1620808101, body_list);
        msg.set_random(Some(8912345));

        msg_list.push(msg);

        let body_list = vec![MsgBody::new(
            MsgContent::str_text(),
            MsgContent::content_text("red packet"),
        )];

        let msg = Msg::new("peter", 1620892821, body_list);

        msg_list.push(msg);

        let mut req = super::ImportGroupMsgRequest::new("@TGS#2C5SZEAEF", msg_list);
        req.set_recent_contact_flag(Some(true));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "ImportMsgResult": [
                {
                    "MsgSeq": 1,
                    "MsgTime": 1620808101,
                    "Result": 0
                },
                {
                    "MsgSeq": 2,
                    "MsgTime": 1620892821,
                    "Result": 0
                },
            ]
        });

        let res = serde_json::from_value::<super::ImportGroupMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
