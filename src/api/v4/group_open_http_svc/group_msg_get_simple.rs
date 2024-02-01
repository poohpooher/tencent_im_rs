//! <https://www.tencentcloud.com/ko/document/product/1047/34971>

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use crate::api::{bool_to_int, int_to_bool};
use crate::tencent_api;

tencent_api!(
    group_msg_get_simple,
    GroupMsgGetSimpleRequest,
    GroupMsgGetSimpleResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMsgGetSimpleRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "ReqMsgNumber")]
    req_msg_number: u32,

    #[serde(rename = "ReqMsgSeq")]
    req_msg_seq: Option<u32>,

    #[serde(
        rename = "WithRecalledMsg",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    with_recalled_msg: Option<bool>,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl GroupMsgGetSimpleRequest {
    pub fn new<S: AsRef<str>>(group_id: S, req_msg_number: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            req_msg_number,
            req_msg_seq: None,
            with_recalled_msg: None,
            topic_id: None,
        }
    }

    pub fn set_req_msg_seq(&mut self, req_msg_seq: Option<u32>) -> &mut Self {
        self.req_msg_seq = req_msg_seq;
        self
    }

    pub fn set_with_recalled_msg(&mut self, with_recalled_msg: Option<bool>) -> &mut Self {
        self.with_recalled_msg = with_recalled_msg;
        self
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMsgGetSimpleResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,

    #[serde(rename = "IsFinished")]
    pub is_finished: Option<u32>,

    #[serde(rename = "RspMsgList")]
    pub rsp_msg_list: Option<Vec<RspMsg>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RspMsg {
    #[serde(rename = "From_Account")]
    pub from_account: String,

    #[serde(rename = "IsPlaceMsg")]
    pub is_place_msg: u32,

    #[serde(rename = "MsgBody")]
    pub msg_body: Vec<MsgBody>,

    #[serde(rename = "MsgPriority")]
    pub msg_priority: u32,

    #[serde(rename = "MsgRandom")]
    pub msg_random: u32,

    #[serde(rename = "MsgSeq")]
    pub msg_seq: u32,

    #[serde(rename = "MsgTimeStamp")]
    pub msg_time_stamp: u32,

    #[serde(rename = "IsSystemMsg")]
    pub is_system_msg: Option<u32>,
}

#[cfg(test)]
mod test_group_msg_get_simple {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#15ERQPAER",    // The ID of the group of which messages are to be pulled
          "ReqMsgNumber": 2      // The number of messages to be pulled
        });

        let req = super::GroupMsgGetSimpleRequest::new("@TGS#15ERQPAER", 2);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#15ERQPAER",
          "ReqMsgSeq": 7803321,       // The maximum seq of the requested messages. The messages with a seq less than or equal to `ReqMsgSeq` will be returned.
          "ReqMsgNumber": 2
        });

        let mut req = super::GroupMsgGetSimpleRequest::new("@TGS#15ERQPAER", 2);
        req.set_req_msg_seq(Some(7803321));

        assert_eq!(serde_json::to_value(&req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "GroupId": "@TGS#15ERQPAER",
            "IsFinished": 1,
            "RspMsgList": [
                {
                    "From_Account": "144115197276518801",
                    "IsPlaceMsg": 0,
                    "MsgBody": [
                        {
                            "MsgContent": {
                                "Data": r#"\b\u0001\u0010\u0006\u001A\u0006 MaoTong"#,
                                "Desc": "MIF",
                                "Ext": ""
                            },
                            "MsgType": "TIMCustomElem"
                        },
                        {
                            "MsgContent": {
                                "Data": "",
                                "Index": 15
                            },
                            "MsgType": "TIMFaceElem"
                        }
                    ],
                    "MsgPriority": 1,
                    "MsgRandom": 51083293,
                    "MsgSeq": 7803321,
                    "MsgTimeStamp": 1458721802
                },
                {
                    "From_Account": "144115198339527735",
                    "IsPlaceMsg": 0,
                    "MsgBody": [
                        {
                            "MsgContent": {
                                "Data": r#"\b\u0001\u0010\u0006\u001A\u000F Watermelon Girl"#,
                                "Desc": "MIF",
                                "Ext": ""
                            },
                            "MsgType": "TIMCustomElem"
                        },
                        {
                            "MsgContent": {
                                "Text": "Report"
                            },
                            "MsgType": "TIMTextElem"
                        }
                    ],
                    "MsgPriority": 1,
                    "MsgRandom": 235168582,
                    "MsgSeq": 7803320,
                    "MsgTimeStamp": 1458721797
                }
            ]
        });

        let res =
            serde_json::from_value::<super::GroupMsgGetSimpleResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
