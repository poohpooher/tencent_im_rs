//! <https://www.tencentcloud.com/ko/document/product/1047/34920>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody, OfflinePush};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(batchsendmsg, BatchSendMsgRequest, BatchSendMsgResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchSendMsgRequest {
    #[serde(rename = "SyncOtherMachine")]
    sync_other_machine: Option<u32>,

    #[serde(rename = "From_Account")]
    from_account: Option<String>,

    #[serde(rename = "To_Account")]
    to_account: Vec<String>,

    #[serde(rename = "MsgSeq")]
    msg_seq: Option<u32>,

    #[serde(rename = "MsgLifeTime")]
    msg_life_time: Option<u32>,

    #[serde(rename = "MsgRandom")]
    msg_random: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,

    #[serde(rename = "OfflinePushInfo")]
    offline_push_info: Option<OfflinePush>,

    #[serde(rename = "IsNeedReadReceipt")]
    is_need_read_receipt: Option<u32>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,
}

impl BatchSendMsgRequest {
    pub fn new<S: AsRef<str>>(to_account: Vec<S>, msg_random: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            sync_other_machine: None,
            from_account: None,
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
            msg_seq: None,
            msg_life_time: None,
            msg_random,
            msg_body,
            offline_push_info: None,
            is_need_read_receipt: None,
            cloud_custom_data: None,
        }
    }

    pub fn set_sync_other_machine(&mut self, sync_other_machine: Option<u32>) -> &mut Self {
        self.sync_other_machine = sync_other_machine;
        self
    }

    pub fn set_from_account<S: AsRef<str>>(&mut self, from_account: Option<S>) -> &mut Self {
        self.from_account = from_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_msg_seq(&mut self, msg_seq: Option<u32>) -> &mut Self {
        self.msg_seq = msg_seq;
        self
    }

    pub fn set_msg_life_time(&mut self, msg_life_time: Option<u32>) -> &mut Self {
        self.msg_life_time = msg_life_time;
        self
    }

    pub fn set_offline_push_info(&mut self, offline_push_info: Option<OfflinePush>) -> &mut Self {
        self.offline_push_info = offline_push_info;
        self
    }

    pub fn set_is_need_read_receipt(&mut self, is_need_read_receipt: Option<u32>) -> &mut Self {
        self.is_need_read_receipt = is_need_read_receipt;
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
pub struct BatchSendMsgResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "MsgKey")]
    pub msg_key: Option<String>,

    #[serde(rename = "ErrorList")]
    pub error_list: Option<Vec<ErrorItem>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorItem {
    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "ErrorCode")]
    pub error_code: ErrorCode,
}

#[cfg(test)]
mod test_batch_send_msg {
    use crate::api::v4::common::{Android, Apns, MsgBody, MsgContent, OfflinePush};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "SyncOtherMachine": 2, // Do not synchronize the message to the sender.
            "To_Account": [ // A list of target accounts
                "bonnie",
                "rong"
            ],
            "MsgSeq": 28360, // Sequence number of the message.
            "MsgRandom": 19901224, // Random number of the message
            "MsgBody": [ // Message body
                {
                    "MsgType": "TIMTextElem", // Message type. `TIMTextElem` indicates text messages.
                    "MsgContent": {
                        "Text": "hi, beauty" // Message text
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::BatchSendMsgRequest::new(
            vec!["bonnie", "rong"],
            19901224,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_sync_other_machine(Some(2))
            .set_msg_seq(Some(28360))
            .set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "SyncOtherMachine": 1, // Synchronize the message to the sender.
            "From_Account": "dave",
            "To_Account": [
                "bonnie",
                "rong"
            ],
            "MsgSeq": 28360, // Sequence number of the message.
            "MsgRandom": 19901224, // Random number of the message
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hi, beauty"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data",
            "OfflinePushInfo": {
                "PushFlag": 0,
                "Desc": "Content to push offline",
                "Ext": "Passthrough content",
                "AndroidInfo": {
                    "Sound": "android.mp3"
                },
                "ApnsInfo": {
                    "Sound": "apns.mp3",
                    "BadgeMode": 1, // If this field is left as default or is set to `0`, the message is counted. If this field is set to `1`, the message is not counted, that is, the badge counter in the upper-right corner does not increase.
                    "Title":"apns title", // APNs title
                    "SubTitle":"apns subtitle", // APNs subtitle
                    "Image":"www.image.com" // Image URL
                }
            }
        });

        let mut req = super::BatchSendMsgRequest::new(
            vec!["bonnie", "rong"],
            19901224,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        let mut android = Android::new();
        android.set_sound(Some("android.mp3"));

        let mut apns = Apns::new();
        apns.set_sound(Some("apns.mp3"))
            .set_badge_mode(Some(1))
            .set_title(Some("apns title"))
            .set_sub_title(Some("apns subtitle"))
            .set_image(Some("www.image.com"));

        let mut offline_push = OfflinePush::new();
        offline_push
            .set_push_flag(Some(0))
            .set_desc(Some("Content to push offline"))
            .set_ext(Some("Passthrough content"))
            .set_android_info(Some(android))
            .set_apns_info(Some(apns));

        req.set_sync_other_machine(Some(1))
            .set_from_account(Some("dave"))
            .set_msg_seq(Some(28360))
            .set_cloud_custom_data(Some("your cloud custom data"))
            .set_offline_push_info(Some(offline_push));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!( {
          "ErrorInfo": "",
          "ActionStatus": "OK",
          "ErrorCode": 0,
          "MsgKey": "128493_903762_1572870301"
        });

        let res = serde_json::from_value::<super::BatchSendMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "SomeError",
          "ErrorCode": 0,
          "ErrorInfo": "",
          "MsgKey": "4852_28135_1579678877",
          "ErrorList": [ // List of the accounts to which the message was not sent
              {
                  "To_Account": "rong", // Account to which the message was not sent
                  "ErrorCode":  70107 // Error code. `70107` indicates that the account does not exist.
              }
          ]
        });

        let res = serde_json::from_value::<super::BatchSendMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response3() {
        let sample = json!({
          "ActionStatus": "FAIL",
          "ErrorInfo": "invalid To_Account",
          "ErrorCode": 90012
        });

        let res = serde_json::from_value::<super::BatchSendMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
