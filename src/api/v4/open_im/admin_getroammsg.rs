//! <https://www.tencentcloud.com/ko/document/product/1047/35478>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    admin_getroammsg,
    AdminGetRoamMsgRequest,
    AdminGetRoamMsgResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminGetRoamMsgRequest {
    #[serde(rename = "Operator_Account")]
    operator_account: String,

    #[serde(rename = "Peer_Account")]
    peer_account: String,

    #[serde(rename = "MaxCnt")]
    max_cnt: u32,

    #[serde(rename = "MinTime")]
    min_time: u32,

    #[serde(rename = "MaxTime")]
    max_time: u32,

    #[serde(rename = "LastMsgKey")]
    last_msg_key: Option<String>,
}

impl AdminGetRoamMsgRequest {
    pub fn new<S: AsRef<str>>(
        operator_account: S,
        peer_account: S,
        max_cnt: u32,
        min_time: u32,
        max_time: u32,
    ) -> Self {
        Self {
            operator_account: operator_account.as_ref().to_string(),
            peer_account: peer_account.as_ref().to_string(),
            max_cnt,
            min_time,
            max_time,
            last_msg_key: None,
        }
    }

    pub fn set_last_msg_key<S: AsRef<str>>(&mut self, last_msg_key: S) -> &mut Self {
        self.last_msg_key = Some(last_msg_key.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminGetRoamMsgResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "Complete")]
    pub complete: Option<u32>,

    #[serde(rename = "MsgCnt")]
    pub msg_cnt: Option<u32>,

    #[serde(rename = "LastMsgTime")]
    pub last_msg_time: Option<u32>,

    #[serde(rename = "LastMsgKey")]
    pub last_msg_key: Option<String>,

    #[serde(rename = "MsgList")]
    pub msg_list: Option<Vec<RoamMsg>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoamMsg {
    #[serde(rename = "From_Account")]
    pub from_account: String,

    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "MsgSeq")]
    pub msg_seq: Option<u32>,

    #[serde(rename = "MsgRandom")]
    pub msg_random: u32,

    #[serde(rename = "MsgTimeStamp")]
    pub msg_time_stamp: u32,

    #[serde(rename = "MsgFlagBits")]
    pub msg_flag_bits: u32,

    #[serde(rename = "IsPeerRead")]
    pub is_peer_read: u32,

    #[serde(rename = "MsgKey")]
    pub msg_key: String,

    #[serde(rename = "MsgBody")]
    pub msg_body: Vec<MsgBody>,

    #[serde(rename = "CloudCustomData")]
    pub cloud_custom_data: Option<String>,
}

#[cfg(test)]
mod test_admin_get_roam_msg {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "Operator_Account":"user2",
            "Peer_Account":"user1",
            "MaxCnt":100,
            "MinTime":1584669600,
            "MaxTime":1584669680,
            "LastMsgKey": "549396494_2578554_1584669680"
        });

        let mut req =
            super::AdminGetRoamMsgRequest::new("user2", "user1", 100, 1584669600, 1584669680);
        req.set_last_msg_key("549396494_2578554_1584669680");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "Complete": 1,
            "MsgCnt": 5, // Five messages were returned for the pull.
            "LastMsgTime": 1584669601,
            "LastMsgKey": "1456_23287_1584669601",
            "MsgList": [
                {
                    "From_Account": "user1",
                    "To_Account": "user2",
                    "MsgSeq": 1456,
                    "MsgRandom": 23287,
                    "MsgTimeStamp": 1584669601,
                    "MsgFlagBits": 0,
                    "IsPeerRead": 1,
                    "MsgKey": "1456_23287_1584669601",
                    "MsgBody": [
                        {
                            "MsgType": "TIMTextElem",
                            "MsgContent": {
                                "Text": "msg 13"
                            }
                        }
                    ],
                    "CloudCustomData": "your cloud custom data"
                },
                {
                    "From_Account": "user2",
                    "To_Account": "user1",
                    "MsgSeq": 9806,
                    "MsgRandom": 14,
                    "MsgTimeStamp": 1584669602,
                    "MsgFlagBits": 0,
                    "IsPeerRead": 1,
                    "MsgKey": "9806_14_1584669602",
                    "MsgBody": [
                        {
                            "MsgType": "TIMTextElem",
                            "MsgContent": {
                                "Text": "msg 14"
                            }
                        }
                    ],
                    "CloudCustomData": "your cloud custom data"
                },
            ]
        });

        let res = serde_json::from_value::<super::AdminGetRoamMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "Complete": 1,
          "MsgCnt": 1,
          "LastMsgTime": 1584669680,
          "LastMsgKey": "549396494_2578554_1584669680",
          "MsgList": [
              {
                  "From_Account": "user1",
                  "To_Account": "user2",
                  "MsgSeq": 549396494,
                  "MsgRandom": 2578554,
                  "MsgTimeStamp": 1584669680,
                  "MsgFlagBits": 0,
                  "IsPeerRead": 0,
                  "MsgKey": "549396494_2578554_1584669680",
                  "MsgBody": [
                      {
                          "MsgType": "TIMTextElem",
                          "MsgContent": {
                              "Text": "1"
                          }
                      }
                  ],
                  "CloudCustomData": "your cloud custom data"
              }
          ]
        });

        let res = serde_json::from_value::<super::AdminGetRoamMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
