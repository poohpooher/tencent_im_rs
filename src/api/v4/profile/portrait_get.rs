//! <https://www.tencentcloud.com/document/product/1047/34917>

use crate::api::v4::common::{ActionStatus, ErrorCode, ProfileTag};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(portrait_get, PortraitGetRequest, PortraitGetResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortraitGetRequest {
    #[serde(rename = "To_Account")]
    to_account: Vec<String>,

    #[serde(rename = "TagList")]
    tag_list: Vec<String>,
}

impl PortraitGetRequest {
    pub fn new<S: AsRef<str>>(to_account: Vec<S>, tag_list: Vec<S>) -> Self {
        Self {
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
            tag_list: tag_list.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortraitGetResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UserProfileItem")]
    pub user_profile_item: Option<Vec<UserProfileItem>>,

    #[serde(rename = "Fail_Account")]
    pub fail_account: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserProfileItem {
    #[serde(rename = "To_Account")]
    pub to_account: String,

    #[serde(rename = "ProfileItem")]
    pub profile_item: Option<Vec<ProfileItem>>,

    #[serde(rename = "ResultCode")]
    pub result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    pub result_info: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileItem {
    #[serde(rename = "Tag")]
    pub tag: String,

    #[serde(rename = "Value")]
    pub value: ProfileTag,
}

#[cfg(test)]
mod test_portrait_get {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "To_Account":["id1"],
          "TagList":
          [
              "Tag_Profile_IM_Nick"
          ]
        });

        let req = super::PortraitGetRequest::new(vec!["id1"], vec!["Tag_Profile_IM_Nick"]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn request2() {
        let sample = json!({
          "To_Account":["id1"],
          "TagList":
          [
              "Tag_Profile_IM_Nick",
              "Tag_Profile_IM_AllowType",
              "Tag_Profile_IM_SelfSignature",
              "Tag_Profile_Custom_Test"
          ]
        });

        let req = super::PortraitGetRequest::new(
            vec!["id1"],
            vec![
                "Tag_Profile_IM_Nick",
                "Tag_Profile_IM_AllowType",
                "Tag_Profile_IM_SelfSignature",
                "Tag_Profile_Custom_Test",
            ],
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn request3() {
        let sample = json!({
          "To_Account":["id1","id2","id3"],
          "TagList":
          [
              "Tag_Profile_IM_Nick"
          ]
        });

        let req =
            super::PortraitGetRequest::new(vec!["id1", "id2", "id3"], vec!["Tag_Profile_IM_Nick"]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn request4() {
        let sample = json!({
          "To_Account":["id1","id2","id3","id4"],
          "TagList":
          [
              "Tag_Profile_IM_Nick",
              "Tag_Profile_IM_AllowType",
              "Tag_Profile_IM_SelfSignature",
              "Tag_Profile_Custom_Test"
          ]
        });

        let req = super::PortraitGetRequest::new(
            vec!["id1", "id2", "id3", "id4"],
            vec![
                "Tag_Profile_IM_Nick",
                "Tag_Profile_IM_AllowType",
                "Tag_Profile_IM_SelfSignature",
                "Tag_Profile_Custom_Test",
            ],
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
          "UserProfileItem":
          [
              {
                  "To_Account":"id1",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest1"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              }
          ],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::PortraitGetResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }

    #[test]
    fn response2() {
        let sample = json!({
          "UserProfileItem":
          [
              {
                  "To_Account":"id1",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest1"
                      },
                      {
                          "Tag":"Tag_Profile_IM_AllowType",
                          "Value":"AllowType_Type_NeedConfirm"
                      },
                      {
                          "Tag":"Tag_Profile_IM_SelfSignature",
                          "Value":"I'm Test1"
                      },
                      {
                          "Tag":"Tag_Profile_Custom_Test",
                          "Value":"Custom Data1"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              }
          ],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::PortraitGetResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }

    #[test]
    fn response3() {
        let sample = json!({
          "UserProfileItem":
          [
              {
                  "To_Account":"id1",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest1"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id2",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest2"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id3",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest3"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              }
          ],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::PortraitGetResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }

    #[test]
    fn response4() {
        let sample = json!({
          "UserProfileItem":
          [
              {
                  "To_Account":"id1",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest1"
                      },
                      {
                          "Tag":"Tag_Profile_IM_AllowType",
                          "Value":"AllowType_Type_NeedConfirm"
                      },
                      {
                          "Tag":"Tag_Profile_IM_SelfSignature",
                          "Value":"I'm Test1"
                      },
                      {
                          "Tag":"Tag_Profile_Custom_Test",
                          "Value":"Custom Data1"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id2",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest2"
                      },
                      {
                          "Tag":"Tag_Profile_IM_AllowType",
                          "Value":"AllowType_Type_DenyAny"
                      },
                      {
                          "Tag":"Tag_Profile_IM_SelfSignature",
                          "Value":"I'm Test2"
                      },
                      {
                          "Tag":"Tag_Profile_Custom_Test",
                          "Value":"Custom Data2"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id3",
                  "ProfileItem":
                  [
                      {
                          "Tag":"Tag_Profile_IM_Nick",
                          "Value":"NickNameTest3"
                      },
                      {
                          "Tag":"Tag_Profile_IM_AllowType",
                          "Value":"AllowType_Type_AllowAny"
                      },
                      {
                          "Tag":"Tag_Profile_IM_SelfSignature",
                          "Value":"I'm Test3"
                      },
                      {
                          "Tag":"Tag_Profile_Custom_Test",
                          "Value":"Custom Data3"
                      }
                  ],
                  "ResultCode":0,
                  "ResultInfo":""
              },
              {
                  "To_Account":"id4",
                  "ResultCode":40006,
                  "ResultInfo":"Err_Profile_PortraitGet_Read_Custom_Data_Fail"
              }
          ],
          "Fail_Account":["id4"],
          "ActionStatus":"OK",
          "ErrorCode":0,
          "ErrorInfo":"",
          "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::PortraitGetResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
