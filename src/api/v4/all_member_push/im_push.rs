//! <https://www.tencentcloud.com/ko/document/product/1047/37166>

use crate::api::v4::common::{ActionStatus, Condition, ErrorCode, MsgBody, OfflinePush};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(im_push, ImPushRequest, ImPushResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImPushRequest {
    #[serde(rename = "Condition")]
    condition: Option<Condition>,

    #[serde(rename = "MsgRandom")]
    msg_random: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,

    #[serde(rename = "MsgLifeTime")]
    msg_life_time: Option<u32>,

    #[serde(rename = "From_Account")]
    from_account: Option<String>,

    #[serde(rename = "OfflinePushInfo")]
    offline_push_info: Option<OfflinePush>,
}

impl ImPushRequest {
    pub fn new(msg_random: u32, msg_body: Vec<MsgBody>) -> Self {
        Self {
            condition: None,
            msg_random,
            msg_body,
            msg_life_time: None,
            from_account: None,
            offline_push_info: None,
        }
    }

    pub fn set_msg_life_time(&mut self, msg_life_time: u32) -> &mut Self {
        self.msg_life_time = Some(msg_life_time);
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

    pub fn set_condition(&mut self, condition: Option<Condition>) -> &mut Self {
        self.condition = condition;
        self
    }

    pub fn set_msg_random(&mut self, msg_random: u32) -> &mut Self {
        self.msg_random = msg_random;
        self
    }

    pub fn set_msg_body(&mut self, msg_body: Vec<MsgBody>) -> &mut Self {
        self.msg_body = msg_body;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImPushResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "TaskId")]
    pub task_id: Option<String>,
}

#[cfg(test)]
mod test_im_push {
    use crate::api::v4::common::{Android, Apns, Condition, MsgBody, MsgContent, OfflinePush};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 56512,
          "MsgLifeTime": 120,
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            56512,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account": "xiaoming",
          "MsgRandom": 3674128,
          "MsgLifeTime": 120,
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            3674128,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("xiaoming"));
        req.set_msg_life_time(120);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "From_Account": "xiaoming",
          "MsgRandom": 3674128,
          "MsgLifeTime": 120,
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ],
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

        let mut req = super::ImPushRequest::new(
            3674128,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("xiaoming"));
        req.set_msg_life_time(120);

        let mut android_info = Android::new();
        android_info.set_sound(Some("android.mp3"));

        let mut aspns_info = Apns::new();
        aspns_info.set_sound(Some("apns.mp3"));
        aspns_info.set_badge_mode(Some(1));
        aspns_info.set_title(Some("apns title"));
        aspns_info.set_sub_title(Some("apns subtitle"));
        aspns_info.set_image(Some("www.image.com"));

        let mut offline_push_info = OfflinePush::new();
        offline_push_info.set_push_flag(Some(0));
        offline_push_info.set_desc(Some("Content to push offline"));
        offline_push_info.set_ext(Some("Passthrough content"));
        offline_push_info.set_android_info(Some(android_info));
        offline_push_info.set_apns_info(Some(aspns_info));

        req.set_offline_push_info(Some(offline_push_info));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 21302570,
          "MsgLifeTime": 120,
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            21302570,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request5() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 214,
          "MsgLifeTime": 120,
          "Condition":{
              "TagsAnd": ["Stock A","Stock B"]
          },
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            214,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);
        req.set_condition(Some(Condition::tags_and(vec!["Stock A", "Stock B"])));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request6() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 124032,
          "MsgLifeTime": 120,
          "Condition":{
              "TagsOr": ["Stock A","Stock B"]
          },
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            124032,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);
        req.set_condition(Some(Condition::tags_or(vec!["Stock A", "Stock B"])));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request7() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 389475,
          "MsgLifeTime": 120,
          "Condition":{
              "AttrsAnd": {
                  "Membership Level": "Platinum Premier members",
                  "city": "Shenzhen"
              }
          },
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            389475,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);
        req.set_condition(Some(Condition::attrs_and(
            vec![
                ("Membership Level", "Platinum Premier members"),
                ("city", "Shenzhen"),
            ]
            .into_iter()
            .collect(),
        )));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request8() {
        let sample = json!({
          "From_Account": "admin",
          "MsgRandom": 9312457,
          "MsgLifeTime": 120,
          "Condition":{
              "AttrsAnd": {
                  "Membership Level": "Platinum Premier users",
                  "city": "Shenzhen"
              }
          },
          "MsgBody":[
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent":{
                      "Text": "hi, beauty"
                  }
              }
          ]
        });

        let mut req = super::ImPushRequest::new(
            9312457,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_from_account(Some("admin"));
        req.set_msg_life_time(120);
        req.set_condition(Some(Condition::attrs_and(
            vec![
                ("Membership Level", "Platinum Premier users"),
                ("city", "Shenzhen"),
            ]
            .into_iter()
            .collect(),
        )));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "TaskId": "1400123456_144115212910570789_4155518400_15723514"
        });

        let res = serde_json::from_value::<super::ImPushResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
