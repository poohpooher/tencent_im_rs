//! <https://www.tencentcloud.com/ko/document/product/1047/34904>
use crate::api::v4::common::{ActionStatus, ErrorCode, SnsTag};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_update, FriendUpdateRequest, FriendUpdateResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendUpdateRequest {
    #[serde(rename = "From_Account")]
    /// 친구 추가 요청을 보낸 아이디
    from_account: String,

    #[serde(rename = "UpdateItem")]
    /// 친구 업데이트 아이템
    update_items: Vec<UpdateItem>,
}

impl FriendUpdateRequest {
    pub fn new<S: AsRef<str>>(from_account: S) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            update_items: Vec::new(),
        }
    }

    pub fn set_update_items(&mut self, update_items: Vec<UpdateItem>) -> &mut Self {
        self.update_items = update_items;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateItem {
    #[serde(rename = "To_Account")]
    /// 친구 추가 대상 아이디
    to_account: String,

    #[serde(rename = "SnsItem")]
    /// 친구 업데이트 아이템
    sns_items: Vec<SnsItem>,
}

impl UpdateItem {
    pub fn new<S: AsRef<str>>(to_account: S) -> Self {
        Self {
            to_account: to_account.as_ref().to_string(),
            sns_items: Vec::new(),
        }
    }

    pub fn set_sns_items(&mut self, sns_items: Vec<SnsItem>) -> &mut Self {
        self.sns_items = sns_items;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SnsItem {
    #[serde(rename = "Tag")]
    // 친구 아이템 태그
    tag: String,

    #[serde(rename = "Value")]
    // 친구 아이템 값
    value: SnsTag,
}

impl SnsItem {
    pub fn new<S: AsRef<str>>(tag: S, value: SnsTag) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            value,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendUpdateResponse {
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
    /// 친구 추가 실패 아이디 리스트
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
mod test_friend_update {
    use serde_json::json;

    use crate::api::v4::common::SnsTag;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"id",
          "UpdateItem":
          [
              {
                  "To_Account":"id1",
                  "SnsItem":
                  [
                      {
                          "Tag":"Tag_SNS_IM_Remark",
                          "Value":"remark1"
                      }
                  ]
              }
          ]
        });

        let mut req = super::FriendUpdateRequest::new("id");
        let mut update_items = vec![];

        let mut update_item = super::UpdateItem::new("id1");

        update_item.set_sns_items(vec![super::SnsItem {
            tag: "Tag_SNS_IM_Remark".to_string(),
            value: SnsTag::TagSnsImRemark("remark1".to_string()),
        }]);

        update_items.push(update_item);

        req.set_update_items(update_items);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "not same");
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"id",
          "UpdateItem":
          [
              {
                  "To_Account":"id1",
                  "SnsItem":
                  [
                      {
                          "Tag":"Tag_SNS_IM_Remark",
                          "Value":"remark1"
                      },
                      {
                          "Tag":"Tag_SNS_IM_Group",
                          "Value":
                          [
                              "group1",
                              "group2"
                          ]
                      },
                      {
                          "Tag":"Tag_SNS_Custom_Test",
                          "Value":"test"
                      }
                  ]
              }
          ]
        });

        let mut req = super::FriendUpdateRequest::new("id");
        let mut update_items = vec![];

        let mut update_item = super::UpdateItem::new("id1");

        update_item.set_sns_items(vec![
            super::SnsItem {
                tag: "Tag_SNS_IM_Remark".to_string(),
                value: SnsTag::TagSnsImRemark("remark1".to_string()),
            },
            super::SnsItem {
                tag: "Tag_SNS_IM_Group".to_string(),
                value: SnsTag::TagSnsImGroup(vec!["group1".to_string(), "group2".to_string()]),
            },
            super::SnsItem {
                tag: "Tag_SNS_Custom_Test".to_string(),
                value: SnsTag::TagSnsImRemark("test".to_string()),
            },
        ]);

        update_items.push(update_item);

        req.set_update_items(update_items);

        let json = serde_json::to_value(req).unwrap();

        assert_eq!(sample, json, "not same");
    }

    #[test]
    fn request3() {
        let sample = json!({
          "From_Account":"id",
          "UpdateItem":
          [
              {
                  "To_Account":"id1",
                  "SnsItem":
                  [
                      {
                          "Tag":"Tag_SNS_IM_Remark",
                          "Value":"remark1"
                      }
                  ]
              },
              {
                  "To_Account":"id2",
                  "SnsItem":
                  [
                      {
                          "Tag":"Tag_SNS_IM_Remark",
                          "Value":"remark2"
                      },
                      {
                          "Tag":"Tag_SNS_IM_Group",
                          "Value":
                          [
                              "group1",
                              "group2"
                          ]
                      }
                  ]
              },
              {
                  "To_Account":"id3",
                  "SnsItem":
                  [
                      {
                          "Tag":"Tag_SNS_IM_Remark",
                          "Value":"remark3"
                      },
                      {
                          "Tag":"Tag_SNS_IM_Group",
                          "Value":
                          [
                              "group3"
                          ]
                      },
                      {
                          "Tag":"Tag_SNS_Custom_Test",
                          "Value":"test"
                      }
                  ]
              }
          ]
        });

        let mut req = super::FriendUpdateRequest::new("id");
        let mut update_items = vec![];

        let mut update_item = super::UpdateItem::new("id1");
        update_item.set_sns_items(vec![super::SnsItem {
            tag: "Tag_SNS_IM_Remark".to_string(),
            value: SnsTag::TagSnsImRemark("remark1".to_string()),
        }]);

        update_items.push(update_item);

        let mut update_item = super::UpdateItem::new("id2");
        update_item.set_sns_items(vec![
            super::SnsItem {
                tag: "Tag_SNS_IM_Remark".to_string(),
                value: SnsTag::TagSnsImRemark("remark2".to_string()),
            },
            super::SnsItem {
                tag: "Tag_SNS_IM_Group".to_string(),
                value: SnsTag::TagSnsImGroup(vec!["group1".to_string(), "group2".to_string()]),
            },
        ]);

        update_items.push(update_item);

        let mut update_item = super::UpdateItem::new("id3");
        update_item.set_sns_items(vec![
            super::SnsItem {
                tag: "Tag_SNS_IM_Remark".to_string(),
                value: SnsTag::TagSnsImRemark("remark3".to_string()),
            },
            super::SnsItem {
                tag: "Tag_SNS_IM_Group".to_string(),
                value: SnsTag::TagSnsImGroup(vec!["group3".to_string()]),
            },
            super::SnsItem {
                tag: "Tag_SNS_Custom_Test".to_string(),
                value: SnsTag::TagSnsImRemark("test".to_string()),
            },
        ]);

        update_items.push(update_item);

        req.set_update_items(update_items);

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

        let res = serde_json::from_value::<super::FriendUpdateResponse>(sample.clone()).unwrap();

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
                  "ResultCode":30011,
                  "ResultInfo":"Err_SNS_FriendUpdate_Group_Num_Exceed_Threshold"
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

        let res = serde_json::from_value::<super::FriendUpdateResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
