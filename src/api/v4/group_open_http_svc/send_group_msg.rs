//! <https://www.tencentcloud.com/ko/document/product/1047/34959>

use crate::api::v4::common::{
    ActionStatus, ErrorCode, ForbidCallback, MsgBody, MsgPriority, OfflinePush, SendMsgControl,
};
use crate::api::{bool_to_int, int_to_bool};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(send_group_msg, SendGroupMsgRequest, SendGroupMsgResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendGroupMsgRequest {
    #[serde(rename = "GroupId")]
    /// 그룹 아이디
    group_id: String,

    #[serde(rename = "Random")]
    /// 랜덤 숫자
    random: u32,

    #[serde(rename = "MsgPriority")]
    /// 메시지 우선순위
    msg_priority: Option<MsgPriority>,

    #[serde(rename = "MsgBody")]
    /// 메시지 내용
    msg_body: Vec<MsgBody>,

    #[serde(rename = "From_Account")]
    /// 발신자 아이디
    from_account: Option<String>,

    #[serde(rename = "OfflinePushInfo")]
    /// 오프라인 푸시 정보
    offline_push_info: Option<OfflinePush>,

    #[serde(rename = "ForbidCallbackControl")]
    /// 콜백 제어
    forbid_callback_control: Option<Vec<ForbidCallback>>,

    #[serde(
        rename = "OnlineOnlyFlag",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    /// 온라인만 허용
    online_only_flag: Option<bool>,

    #[serde(rename = "SendMsgControl")]
    /// 메시지 제어
    send_msg_control: Option<Vec<SendMsgControl>>,

    #[serde(rename = "CloudCustomData")]
    /// 클라우드 커스텀 데이터
    cloud_custom_data: Option<String>,

    #[serde(
        rename = "SupportMessageExtension",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    /// 메시지 확장
    support_message_extension: Option<bool>,

    #[serde(rename = "To_Account")]
    /// 수신자 아이디
    to_account: Option<Vec<String>>,

    #[serde(rename = "TopicId")]
    /// 토픽 아이디
    topic_id: Option<String>,

    #[serde(rename = "GroupAtInfo")]
    /// 그룹 멘션 정보
    group_at_info: Option<Vec<GroupAtInfo>>,
}

impl SendGroupMsgRequest {
    pub fn new<S: AsRef<str>>(group_id: S, random: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            random,
            msg_priority: None,
            msg_body,
            from_account: None,
            offline_push_info: None,
            forbid_callback_control: None,
            online_only_flag: None,
            send_msg_control: None,
            cloud_custom_data: None,
            support_message_extension: None,
            to_account: None,
            topic_id: None,
            group_at_info: None,
        }
    }

    pub fn set_msg_priority(&mut self, msg_priority: Option<MsgPriority>) -> &mut Self {
        self.msg_priority = msg_priority;
        self
    }

    pub fn set_from_account<S: AsRef<str>>(&mut self, from_account: Option<S>) -> &mut Self {
        self.from_account = from_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_offline_push_info(&mut self, offline_push_info: Option<OfflinePush>) -> &mut Self {
        self.offline_push_info = offline_push_info;
        self
    }

    pub fn set_forbid_callback_control(
        &mut self,
        forbid_callback_control: Option<Vec<ForbidCallback>>,
    ) -> &mut Self {
        self.forbid_callback_control = forbid_callback_control;
        self
    }

    pub fn set_online_only_flag(&mut self, online_only_flag: Option<bool>) -> &mut Self {
        self.online_only_flag = online_only_flag;
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
        support_message_extension: Option<bool>,
    ) -> &mut Self {
        self.support_message_extension = support_message_extension;
        self
    }

    pub fn set_to_account<S: AsRef<str>>(&mut self, to_account: Option<Vec<S>>) -> &mut Self {
        self.to_account =
            to_account.map(|s| s.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_topic_id<S: AsRef<str>>(&mut self, topic_id: Option<S>) -> &mut Self {
        self.topic_id = topic_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_at_info(&mut self, group_at_info: Option<Vec<GroupAtInfo>>) -> &mut Self {
        self.group_at_info = group_at_info;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAtInfo {
    #[serde(
        rename = "GroupAtAllFlag",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    pub group_at_all_flag: Option<bool>,

    #[serde(rename = "GroupAt_Account")]
    pub group_at_account: Option<String>,
}
impl Default for GroupAtInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupAtInfo {
    pub fn new() -> Self {
        Self {
            group_at_all_flag: None,
            group_at_account: None,
        }
    }

    pub fn set_group_at_all_flag(&mut self, group_at_all_flag: Option<bool>) -> &mut Self {
        self.group_at_all_flag = group_at_all_flag;
        self
    }

    pub fn set_group_at_account<S: AsRef<str>>(
        &mut self,
        group_at_account: Option<S>,
    ) -> &mut Self {
        self.group_at_account = group_at_account.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SendGroupMsgResponse {
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

    #[serde(rename = "MsgSeq")]
    pub msg_seq: Option<u32>,

    #[serde(rename = "MsgDropReason")]
    pub msg_random: Option<String>,
}

#[cfg(test)]
mod test_send_group_msg {
    use crate::api::v4::common::{Android, Apns, MsgBody, MsgContent, OfflinePush, SendMsgControl};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
          "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
          ],
          "CloudCustomData": "your cloud custom data",
          "SupportMessageExtension": 0,
          "OfflinePushInfo": {
              "PushFlag": 0, // Normal push
              "Desc": "Content to push offline",
              "Ext": "Passthrough content",
              "AndroidInfo": {
                  "Sound": "android.mp3"
              },
              "ApnsInfo": {
                  "Sound": "apns.mp3",
                  "BadgeMode": 1, // If this field is left as default or is set to `0`, the message is counted. If this field is set to `1`, the message is not counted, that is, the icon number in the upper-right corner does not increase.
                  "Title":"apns title", // APNs title
                  "SubTitle":"apns subtitle", // APNs subtitle
                  "Image":"www.image.com" // Image URL
              }
          }
        });

        let mut apns = Apns::new();
        apns.set_sound(Some("apns.mp3"));
        apns.set_badge_mode(Some(1));
        apns.set_title(Some("apns title"));
        apns.set_sub_title(Some("apns subtitle"));
        apns.set_image(Some("www.image.com"));

        let mut android = Android::new();
        android.set_sound(Some("android.mp3"));

        let mut offline_push_info = OfflinePush::new();
        offline_push_info.set_push_flag(Some(0));
        offline_push_info.set_desc(Some("Content to push offline"));
        offline_push_info.set_ext(Some("Passthrough content"));
        offline_push_info.set_android_info(Some(android));
        offline_push_info.set_apns_info(Some(apns));

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );
        req.set_cloud_custom_data(Some("your cloud custom data"));
        req.set_support_message_extension(Some(false));
        req.set_offline_push_info(Some(offline_push_info));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "From_Account": "leckie", // Message sender (optional)
          "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
          "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );
        req.set_from_account(Some("leckie"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "GroupId":"@TGS#12DEVUDHQ",
          "Random":2784275388u32,
          "MsgBody":[
              {
                  "MsgType":"TIMCustomElem",
                  "MsgContent":{
                      "Data":"1cddddddddq1"
                  }
              }
          ],
          "To_Account":["brennanli2", "brennanli3"] // Specify the message recipient (up to 50 recipients can be specified). If this field is used, the message is excluded from the unread count.
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#12DEVUDHQ",
            2784275388,
            vec![MsgBody::new(
                MsgContent::str_custom(),
                MsgContent::content_custom("1cddddddddq1", None, None, None),
            )],
        );

        req.set_to_account(Some(vec!["brennanli2", "brennanli3"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
          "SendMsgControl":["NoLastMsg"],// Do not trigger conversation update.
          "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );

        req.set_send_msg_control(Some(vec![SendMsgControl::NoLastMsg]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request5() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
          "MsgPriority": "High", // Message priority
          "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );

        req.set_msg_priority(Some(super::MsgPriority::High));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request6() {
        let sample = json!({
          "GroupId": "@TGS#2C5SZEAEF",
          "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
          "ForbidCallbackControl":[
                  "ForbidBeforeSendMsgCallback",
                  "ForbidAfterSendMsgCallback"], // Callback forbidding control option
          "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );

        req.set_forbid_callback_control(Some(vec![
            super::ForbidCallback::ForbidBeforeSendMsgCallback,
            super::ForbidCallback::ForbidAfterSendMsgCallback,
        ]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request7() {
        let sample = json!(    {
            "GroupId": "@TGS#2C5SZEAEF",
            "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
            "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
                {
                    "MsgType": "TIMTextElem", // Text
                    "MsgContent": {
                        "Text": "red @all @tommy @brennanli packet"
                    }
                }
            ],
            // It corresponds to @all @tommy @brennanli in the text information.
            "GroupAtInfo":[
            {
                "GroupAtAllFlag":1 // `1`: @ all ; `0`: @ a certain group member
            },
            {
                "GroupAtAllFlag":0,
                "GroupAt_Account":"tommy" // @ a specific group member
            },
            {
                "GroupAtAllFlag":0,
                "GroupAt_Account":"brennanli"
            }
         ]
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("red @all @tommy @brennanli packet"),
            )],
        );

        let mut group_at_info = Vec::new();
        let mut group_at_info1 = super::GroupAtInfo::new();
        group_at_info1.set_group_at_all_flag(Some(true));
        group_at_info.push(group_at_info1);

        let mut group_at_info2 = super::GroupAtInfo::new();
        group_at_info2.set_group_at_all_flag(Some(false));
        group_at_info2.set_group_at_account(Some("tommy"));
        group_at_info.push(group_at_info2);

        let mut group_at_info3 = super::GroupAtInfo::new();
        group_at_info3.set_group_at_all_flag(Some(false));
        group_at_info3.set_group_at_account(Some("brennanli"));
        group_at_info.push(group_at_info3);

        req.set_group_at_info(Some(group_at_info));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request8() {
        let sample = json!(  {
            "GroupId": "@TGS#2C5SZEAEF",
            "Random": 8912345, // A random number. If the random numbers of two messages are the same within five minutes, they are considered to be the same message.
            "OnlineOnlyFlag": 1, // The message is for online delivery only (only online group members will receive it), not for offline or roaming retention.
            "MsgBody": [ // Message body, which consists of an element array. For details, see the field description.
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
        });

        let mut req = super::SendGroupMsgRequest::new(
            "@TGS#2C5SZEAEF",
            8912345,
            vec![
                MsgBody::new(
                    MsgContent::str_text(),
                    MsgContent::content_text("red packet"),
                ),
                MsgBody::new(
                    MsgContent::str_face(),
                    MsgContent::content_face(6, r#"abc\u0000\u0001"#),
                ),
            ],
        );

        req.set_online_only_flag(Some(true));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "MsgTime": 1497249503,
            "MsgSeq": 1,
            "MsgDropReason" : "MsgFreqCtrl"
        });

        let res = serde_json::from_value::<super::SendGroupMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
