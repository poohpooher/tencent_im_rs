//! <https://www.tencentcloud.com/ko/document/product/1047/34903>
use crate::api::v4::common::{ActionStatus, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_import, FriendImportRequest, FriendImportResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendImportRequest {
    #[serde(rename = "From_Account")]
    /// 친구 추가 요청을 보낼 아이디
    from_account: String,

    #[serde(rename = "AddFriendItem")]
    /// 친구 추가 요청을 받을 아이디 목록
    add_friend_item: Vec<AddFriendItem>,
}

impl FriendImportRequest {
    pub fn new<S: AsRef<str>>(from_account: S) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            add_friend_item: Vec::new(),
        }
    }

    pub fn set_add_friend_item(&mut self, add_friend_item: Vec<AddFriendItem>) -> &mut Self {
        self.add_friend_item = add_friend_item;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddFriendItem {
    #[serde(rename = "To_Account")]
    /// 친구 추가 대상 아이디
    to_account: String,

    #[serde(rename = "Remark")]
    /// 친구 추가 요청 시 상대방에게 보여질 메시지
    remark: Option<String>,

    #[serde(rename = "RemarkTime")]
    /// 친구 추가 요청 시간
    remark_time: Option<i64>,

    #[serde(rename = "GroupName")]
    /// 친구 추가 시 그룹 이름
    group_name: Option<Vec<String>>,

    #[serde(rename = "AddSource")]
    /// 친구 추가 요청 방식
    add_source: Option<String>,

    #[serde(rename = "AddWording")]
    /// 친구 추가 요청 메시지
    add_wording: Option<String>,

    #[serde(rename = "AddTime")]
    /// 친구 추가 시간
    add_time: Option<i64>,

    #[serde(rename = "CustomItem")]
    /// 사용자 정의 필드
    custom_item: Option<Vec<CustomItem>>,
}

impl AddFriendItem {
    pub fn new<S: AsRef<str>>(to_account: S) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            remark: None,
            remark_time: None,
            group_name: None,
            add_source: None,
            add_wording: None,
            add_time: None,
            custom_item: None,
        }
    }

    pub fn set_remark<S: AsRef<str>>(&mut self, remark: Option<S>) -> &mut Self {
        self.remark = remark.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_remark_time(&mut self, remark_time: Option<i64>) -> &mut Self {
        self.remark_time = remark_time;
        self
    }

    pub fn set_group_name<S: AsRef<str>>(&mut self, group_name: Option<Vec<S>>) -> &mut Self {
        self.group_name =
            group_name.map(|s| s.into_iter().map(|s| s.as_ref().to_string()).collect());
        self
    }

    pub fn set_add_source<S: AsRef<str>>(&mut self, add_source: Option<S>) -> &mut Self {
        self.add_source = add_source.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_add_wording<S: AsRef<str>>(&mut self, add_wording: Option<S>) -> &mut Self {
        self.add_wording = add_wording.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_add_time(&mut self, add_time: Option<i64>) -> &mut Self {
        self.add_time = add_time;
        self
    }

    pub fn set_custom_item(&mut self, custom_item: Option<Vec<CustomItem>>) -> &mut Self {
        self.custom_item = custom_item;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomItem {
    #[serde(rename = "Tag")]
    /// 사용자 정의 필드 키
    tag: String,

    #[serde(rename = "Value")]
    /// 사용자 정의 필드 값
    value: CustomItemValue,
}

impl CustomItem {
    pub fn new<S: AsRef<str>, V: Into<CustomItemValue>>(tag: S, value: V) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            value: value.into(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomItemValue {
    String(String),
    Number(u64),
}

impl From<String> for CustomItemValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<u64> for CustomItemValue {
    fn from(n: u64) -> Self {
        Self::Number(n)
    }
}

impl From<&str> for CustomItemValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendImportResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ResultItem")]
    pub result_item: Option<Vec<ResultItem>>,

    #[serde(rename = "Fail_Account")]
    pub fail_accounts: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "ResultCode")]
    result_code: u32,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}
#[cfg(test)]
mod test_friend_import {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"id",
          "AddFriendItem":
          [
              {
                  "To_Account":"id1",
                  "AddSource":"AddSource_Type_XXXXXXXX"
              }
          ]
        });

        let mut req = super::FriendImportRequest::new("id");

        let mut add_friend_items = vec![];

        let mut add_friend_item = super::AddFriendItem::new("id1");
        add_friend_item.set_add_source(Some("AddSource_Type_XXXXXXXX"));

        add_friend_items.push(add_friend_item);

        req.set_add_friend_item(add_friend_items);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "not same");
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"id",
          "AddFriendItem":
          [
              {
                  "To_Account":"id1",
                  "Remark":"remark1",
                  "RemarkTime":1420000001,
                  "GroupName":["Friends"],
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test1",
                  "AddTime":1420000001,
                  "CustomItem":
                  [
                      {
                          "Tag":"Tag_SNS_Custom_XXXX",
                          "Value":"Test"
                      },
                      {
                          "Tag":"Tag_SNS_Custom_YYYY",
                          "Value":0
                      }
                  ]
              }
          ]
        });

        let mut req = super::FriendImportRequest::new("id");

        let mut add_friend_items = vec![];

        let mut add_friend_item = super::AddFriendItem::new("id1");
        add_friend_item
            .set_remark(Some("remark1"))
            .set_remark_time(Some(1420000001))
            .set_group_name(Some(vec!["Friends"]))
            .set_add_source(Some("AddSource_Type_XXXXXXXX"))
            .set_add_wording(Some("I'm Test1"))
            .set_add_time(Some(1420000001))
            .set_custom_item(Some(vec![
                super::CustomItem::new("Tag_SNS_Custom_XXXX", "Test"),
                super::CustomItem::new("Tag_SNS_Custom_YYYY", 0),
            ]));

        add_friend_items.push(add_friend_item);

        req.set_add_friend_item(add_friend_items);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "not same");
    }

    #[test]
    fn request3() {
        let sample = json!({
          "From_Account":"id",
          "AddFriendItem":
          [
              {
                  "To_Account":"id1",
                  "AddSource":"AddSource_Type_XXXXXXXX"
              },
              {
                  "To_Account":"id2",
                  "Remark":"remark2",
                  "RemarkTime":1420000001,
                  "GroupName":["Friends"],
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test2",
                  "AddTime":1420000001
              },
              {
                  "To_Account":"id3",
                  "Remark":"remark3",
                  "RemarkTime":1420000001,
                  "GroupName":["Colleagues","Friends"],
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test3",
                  "AddTime":1420000001,
                  "CustomItem":
                  [
                      {
                          "Tag":"Tag_SNS_Custom_XXXX",
                          "Value":"Test"
                      },
                      {
                          "Tag":"Tag_SNS_Custom_YYYY",
                          "Value":0
                      }
                  ]
              }
          ]
        });

        let mut req = super::FriendImportRequest::new("id");

        let mut add_friend_items = vec![];

        let mut add_friend_item = super::AddFriendItem::new("id1");
        add_friend_item.set_add_source(Some("AddSource_Type_XXXXXXXX"));
        add_friend_items.push(add_friend_item);

        let mut add_friend_item = super::AddFriendItem::new("id2");
        add_friend_item
            .set_remark(Some("remark2"))
            .set_remark_time(Some(1420000001))
            .set_group_name(Some(vec!["Friends"]))
            .set_add_source(Some("AddSource_Type_XXXXXXXX"))
            .set_add_wording(Some("I'm Test2"))
            .set_add_time(Some(1420000001));
        add_friend_items.push(add_friend_item);

        let mut add_friend_item = super::AddFriendItem::new("id3");
        add_friend_item
            .set_remark(Some("remark3"))
            .set_remark_time(Some(1420000001))
            .set_group_name(Some(vec!["Colleagues", "Friends"]))
            .set_add_source(Some("AddSource_Type_XXXXXXXX"))
            .set_add_wording(Some("I'm Test3"))
            .set_add_time(Some(1420000001))
            .set_custom_item(Some(vec![
                super::CustomItem::new("Tag_SNS_Custom_XXXX", "Test"),
                super::CustomItem::new("Tag_SNS_Custom_YYYY", 0),
            ]));

        add_friend_items.push(add_friend_item);

        req.set_add_friend_item(add_friend_items);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "not same");
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ResultItem":
          [
              {
                  "To_Account":"id1",
                  "ResultCode":0,
                  "ResultInfo":""
              }
          ],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::FriendImportResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
    #[test]
    fn response2() {
        let sample = json!({
          "ResultItem":
          [
              {
                  "To_Account":"id1",
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id2",
                  "ResultCode":30010,
                  "ResultInfo":"Err_SNS_FriendImport_My_Friend_Num_Exceed_Threshold"
              },
              {
                  "To_Account":"id3",
                  "ResultCode":30002,
                  "ResultInfo":"Err_SNS_FriendImport_SdkAppId_Illegal"
              }
          ],
          "Fail_Account":["id2","id3"],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::FriendImportResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
