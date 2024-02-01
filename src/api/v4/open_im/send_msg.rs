//! <https://www.tencentcloud.com/ko/document/product/1047/34919>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::api::v4::common::{ForbidCallback, MsgBody, OfflinePush, SendMsgControl};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(sendmsg, SendMsgRequest, SendMsgResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMsgRequest {
    #[serde(rename = "SyncOtherMachine")]
    sync_other_machine: Option<u32>,

    #[serde(rename = "From_Account")]
    from_account: Option<String>,

    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "MsgLifeTime")]
    msg_lifetime: Option<u32>,

    #[serde(rename = "MsgSeq")]
    msg_seq: Option<u32>,

    #[serde(rename = "MsgRandom")]
    msg_random: u32,

    #[serde(rename = "ForbidCallbackControl")]
    forbid_callback_control: Option<Vec<ForbidCallback>>,

    #[serde(rename = "SendMsgControl")]
    send_msg_control: Option<Vec<SendMsgControl>>,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,

    #[serde(rename = "SupportMessageExtension")]
    support_message_extension: Option<u32>,

    #[serde(rename = "OfflinePushInfo")]
    offline_push_info: Option<OfflinePush>,

    #[serde(rename = "IsNeedReadReceipt")]
    is_need_read_receipt: Option<u32>,
}

impl SendMsgRequest {
    pub fn new<S: AsRef<str>>(to_account: S, msg_random: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            sync_other_machine: None,
            from_account: None,
            to_account: to_account.as_ref().to_string(),
            msg_lifetime: None,
            msg_seq: None,
            msg_random,
            forbid_callback_control: None,
            send_msg_control: None,
            msg_body,
            cloud_custom_data: None,
            support_message_extension: None,
            offline_push_info: None,
            is_need_read_receipt: None,
        }
    }

    pub fn set_from_account<S: AsRef<str>>(&mut self, from_account: Option<S>) -> &mut Self {
        self.from_account = from_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_sync_other_machine(&mut self, sync_other_machine: Option<u32>) -> &mut Self {
        self.sync_other_machine = sync_other_machine;
        self
    }

    pub fn set_msg_lifetime(&mut self, msg_lifetime: Option<u32>) -> &mut Self {
        self.msg_lifetime = msg_lifetime;
        self
    }

    pub fn set_msg_seq(&mut self, msg_seq: Option<u32>) -> &mut Self {
        self.msg_seq = msg_seq;
        self
    }

    pub fn set_forbid_callback_control(
        &mut self,
        forbid_callback_control: Option<Vec<ForbidCallback>>,
    ) -> &mut Self {
        self.forbid_callback_control = forbid_callback_control;
        self
    }

    pub fn set_send_msg_control(
        &mut self,
        send_msg_control: Option<Vec<SendMsgControl>>,
    ) -> &mut Self {
        self.send_msg_control = send_msg_control;
        self
    }

    pub fn set_cloud_custom_data<S: AsRef<str>>(
        &mut self,
        cloud_custom_data: Option<S>,
    ) -> &mut Self {
        self.cloud_custom_data = cloud_custom_data.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_support_message_extension(
        &mut self,
        support_message_extension: Option<u32>,
    ) -> &mut Self {
        self.support_message_extension = support_message_extension;
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
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMsgResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "MsgTime")]
    pub msg_time: Option<u32>,

    #[serde(rename = "MsgKey")]
    pub msg_key: Option<String>,
}

#[cfg(test)]
mod test_send_msg {
    use crate::api::v4::common::{Android, Apns, MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "SyncOtherMachine": 2, // Do not synchronize the message to the sender.
            "To_Account": "lumotuwe2",
            "MsgLifeTime":60, // Retain the message for 60 seconds.
            "MsgSeq": 93847636,
            "MsgRandom": 1287657,
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hi, beauty"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data",
            "SupportMessageExtension": 0
        });

        let mut req = super::SendMsgRequest::new(
            "lumotuwe2",
            1287657,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );
        req.set_sync_other_machine(Some(2))
            .set_msg_lifetime(Some(60))
            .set_msg_seq(Some(93847636))
            .set_cloud_custom_data(Some("your cloud custom data"))
            .set_support_message_extension(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
            "SyncOtherMachine": 2, // Do not synchronize the message to the sender.
            "To_Account": "lumotuwe2",
            "MsgLifeTime":60, // Retain the message for 60 seconds.
            "MsgSeq": 93847636,
            "MsgRandom": 1287657,
            "ForbidCallbackControl":[
                "ForbidBeforeSendMsgCallback",
                "ForbidAfterSendMsgCallback"], // Callback forbidding control option
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hi, beauty"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::SendMsgRequest::new(
            "lumotuwe2",
            1287657,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_sync_other_machine(Some(2))
            .set_msg_lifetime(Some(60))
            .set_msg_seq(Some(93847636))
            .set_forbid_callback_control(Some(vec![
                super::ForbidCallback::ForbidBeforeSendMsgCallback,
                super::ForbidCallback::ForbidAfterSendMsgCallback,
            ]))
            .set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
            "SyncOtherMachine": 2,
            "From_Account": "lumotuwe1",
            "To_Account": "lumotuwe2",
            "MsgLifeTime":3600, // Retain the message for one hour.
            "MsgSeq": 93847636,
            "MsgRandom": 1287657,
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

        let mut req = super::SendMsgRequest::new(
            "lumotuwe2",
            1287657,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        let mut android_info = Android::new();
        android_info.set_sound(Some("android.mp3"));

        let mut apns_info = Apns::new();
        apns_info
            .set_sound(Some("apns.mp3"))
            .set_badge_mode(Some(1))
            .set_title(Some("apns title"))
            .set_sub_title(Some("apns subtitle"))
            .set_image(Some("www.image.com"));

        let mut offline_push = super::OfflinePush::new();
        offline_push
            .set_push_flag(Some(0))
            .set_desc(Some("Content to push offline"))
            .set_ext(Some("Passthrough content"))
            .set_android_info(Some(android_info))
            .set_apns_info(Some(apns_info));

        req.set_from_account(Some("lumotuwe1"))
            .set_sync_other_machine(Some(2))
            .set_msg_lifetime(Some(3600))
            .set_msg_seq(Some(93847636))
            .set_cloud_custom_data(Some("your cloud custom data"))
            .set_offline_push_info(Some(offline_push));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
            "SyncOtherMachine": 1, // Synchronize the message to the sender.
            "From_Account": "lumotuwe1",
            "To_Account": "lumotuwe2",
            "MsgSeq": 93847636,
            "MsgRandom": 1287657,
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hi, beauty"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::SendMsgRequest::new(
            "lumotuwe2",
            1287657,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("lumotuwe1"))
            .set_sync_other_machine(Some(1))
            .set_msg_seq(Some(93847636))
            .set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ActionStatus": "OK",
          "ErrorInfo": "",
          "ErrorCode": 0,
          "MsgTime": 1572870301,
          "MsgKey": "89541_2574206_1572870301"
        });

        let res = serde_json::from_value::<super::SendMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ActionStatus": "FAIL",
          "ErrorInfo": "Fail to Parse json data of body, Please check it",
          "ErrorCode": 90001
        });

        let res = serde_json::from_value::<super::SendMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
