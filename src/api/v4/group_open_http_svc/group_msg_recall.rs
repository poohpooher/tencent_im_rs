//! <https://www.tencentcloud.com/ko/document/product/1047/34965>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    group_msg_recall,
    GroupMsgRecallRequest,
    GroupMsgRecallResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMsgRecallRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "MsgSeqList")]
    msg_seq_list: Vec<MsgRecall>,

    #[serde(rename = "TopicId")]
    topic_id: Option<String>,
}

impl GroupMsgRecallRequest {
    pub fn new<S: AsRef<str>>(group_id: S, msg_seq_list: Vec<MsgRecall>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            msg_seq_list,
            topic_id: None,
        }
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MsgRecall {
    #[serde(rename = "MsgSeq")]
    msg_seq: u32,

    #[serde(rename = "RetCode")]
    ret_code: Option<ErrorCode>,
}

impl MsgRecall {
    pub fn new(msg_seq: u32) -> Self {
        Self {
            msg_seq,
            ret_code: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMsgRecallResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "RecallRetList")]
    pub recall_ret_list: Vec<MsgRecall>,
}

#[cfg(test)]
mod test_group_msg_recall {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2J4SZEAEL",
            "MsgSeqList":[
                {
                    "MsgSeq":100
                },
                {
                    "MsgSeq":101
                }
            ]
        });

        let request = super::GroupMsgRecallRequest::new(
            "@TGS#2J4SZEAEL",
            vec![super::MsgRecall::new(100), super::MsgRecall::new(101)],
        );

        assert_eq!(sample, serde_json::to_value(request).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "RecallRetList":[
                {
                    "MsgSeq":100,
                    "RetCode":10030
                },
                {
                    "MsgSeq":101,
                    "RetCode":0
                }
            ]
        });

        let res = serde_json::from_value::<super::GroupMsgRecallResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
