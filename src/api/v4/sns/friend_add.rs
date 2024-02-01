//! <https://www.tencentcloud.com/ko/document/product/1047/34902>
use crate::api::v4::common::{ActionStatus, ErrorCode, FriendAddType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_add, FriendAddRequest, FriendAddResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendAddRequest {
    #[serde(rename = "From_Account")]
    /// 요청을 보내는 사용자 아이디
    from_account: String,

    #[serde(rename = "AddFriendItem")]
    /// 친구 추가 요청 리스트
    add_friend_item: Vec<AddFriendItem>,

    #[serde(rename = "AddType")]
    /// 친구 추가 방식
    add_type: Option<FriendAddType>,

    #[serde(rename = "ForceAddFlags")]
    /// 0: 관리자에 의한 강제 추가, 1: 일반적인 추가
    force_add_flags: Option<u32>,
}

impl FriendAddRequest {
    pub fn new<S: AsRef<str>>(from_account: S, add_friend_item: Vec<AddFriendItem>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            add_friend_item,
            add_type: None,
            force_add_flags: None,
        }
    }

    pub fn set_add_type(&mut self, add_type: Option<FriendAddType>) -> &mut Self {
        self.add_type = add_type;
        self
    }

    pub fn set_force_add_flags(&mut self, force_add_flags: Option<u32>) -> &mut Self {
        self.force_add_flags = force_add_flags;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddFriendItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "Remark")]
    remark: Option<String>,

    #[serde(rename = "GroupName")]
    group_name: Option<String>,

    #[serde(rename = "AddSource")]
    add_source: Option<String>,

    #[serde(rename = "AddWording")]
    add_wording: Option<String>,
}

impl AddFriendItem {
    pub fn new<S: AsRef<str>>(to_account: S) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            remark: None,
            group_name: None,
            add_source: None,
            add_wording: None,
        }
    }

    pub fn set_remark<S: AsRef<str>>(&mut self, remark: Option<S>) -> &mut Self {
        self.remark = remark.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_name<S: AsRef<str>>(&mut self, group_name: Option<S>) -> &mut Self {
        self.group_name = group_name.map(|s| s.as_ref().to_string());
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
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendAddResponse {
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
mod test_friend_add {
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

        let mut add_friend_item = super::AddFriendItem::new("id1");
        add_friend_item.set_add_source(Some("AddSource_Type_XXXXXXXX"));

        let req = super::FriendAddRequest::new("id", vec![add_friend_item]);

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
                  "GroupName":"Classmates",
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test1"
              }
          ],
          "AddType":"Add_Type_Both",
          "ForceAddFlags":1
        });

        let mut add_friend_item: Vec<super::AddFriendItem> = Vec::new();
        let mut friend = super::AddFriendItem::new("id1");
        friend
            .set_remark(Some("remark1"))
            .set_group_name(Some("Classmates"))
            .set_add_source(Some("AddSource_Type_XXXXXXXX".to_string()))
            .set_add_wording(Some("I'm Test1"));
        add_friend_item.push(friend);

        let mut req = super::FriendAddRequest::new("id", add_friend_item);
        req.set_add_type(Some(super::FriendAddType::AddTypeBoth));
        req.set_force_add_flags(Some(1));

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
                  "GroupName":"Classmates", // Each user can only be assigned to one friend list when the user is added as a friend. Therefore, we can use `String` as the data type.
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test2"
              },
              {
                  "To_Account":"id3",
                  "Remark":"remark3",
                  "GroupName":"Colleagues", // Each user can only be assigned to one friend list when the user is added as a friend. Therefore, we can use `String` as the data type.
                  "AddSource":"AddSource_Type_XXXXXXXX",
                  "AddWording":"I'm Test3"
              }
          ],
          "AddType":"Add_Type_Both",
          "ForceAddFlags":1
        });

        let mut add_friend_item: Vec<super::AddFriendItem> = Vec::new();
        let mut friend = super::AddFriendItem::new("id1");
        friend.set_add_source(Some("AddSource_Type_XXXXXXXX".to_string()));
        add_friend_item.push(friend);

        let mut friend = super::AddFriendItem::new("id2");
        friend
            .set_remark(Some("remark2"))
            .set_group_name(Some("Classmates"))
            .set_add_source(Some("AddSource_Type_XXXXXXXX".to_string()))
            .set_add_wording(Some("I'm Test2"));
        add_friend_item.push(friend);

        let mut friend = super::AddFriendItem::new("id3");
        friend
            .set_remark(Some("remark3"))
            .set_group_name(Some("Colleagues"))
            .set_add_source(Some("AddSource_Type_XXXXXXXX".to_string()))
            .set_add_wording(Some("I'm Test3"));
        add_friend_item.push(friend);

        let mut req = super::FriendAddRequest::new("id", add_friend_item);
        req.set_add_type(Some(super::FriendAddType::AddTypeBoth));
        req.set_force_add_flags(Some(1));

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

        let res = serde_json::from_value::<super::FriendAddResponse>(sample.clone()).unwrap();

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
                  "ResultCode":30006,
                  "ResultInfo":"Err_SNS_FriendAdd_Unpack_Profile_Data_Fail"
              },
              {
                  "To_Account":"id3",
                  "ResultCode":30002,
                  "ResultInfo":"Err_SNS_FriendAdd_SdkAppId_Illegal"
              }
          ],
          "Fail_Account":["id2","id3"],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::FriendAddResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
