//! <https://www.tencentcloud.com/ko/document/product/1047/34910>

use crate::api::v4::common::{ActionStatus, ErrorCode, SnsProfileItem};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_get_list, FriendGetListRequest, FriendGetListResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendGetListRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_accounts: Vec<String>,

    #[serde(rename = "TagList")]
    tag_list: Vec<String>,
}

impl FriendGetListRequest {
    pub fn new<S: AsRef<str>>(from_account: S, to_accounts: Vec<S>, tag_list: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_accounts: to_accounts.iter().map(|s| s.as_ref().to_string()).collect(),
            tag_list: tag_list.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendGetListResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "InfoItem")]
    pub info_items: Option<Vec<InfoItem>>,

    #[serde(rename = "Fail_Account")]
    pub fail_accounts: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "SnsProfileItem")]
    profile_items: Option<Vec<SnsProfileItem>>,

    #[serde(rename = "ResultCode")]
    result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}

#[cfg(test)]
mod test_friend_get_list {
    use serde_json::json;

    use crate::api::v4::common::{ProfileTag, SnsTag};

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"UserID_1",
          "To_Account":
          [
              "UserID_2"
          ],
          "TagList":
          [
              "Tag_Profile_Custom_Test",
              "Tag_Profile_IM_Image",
              "Tag_Profile_IM_Nick",
              "Tag_SNS_Custom_Test",
              "Tag_SNS_IM_Remark",
              "Tag_SNS_IM_Group"
          ]
        });

        let req = super::FriendGetListRequest::new(
            "UserID_1",
            vec!["UserID_2"],
            vec![
                ProfileTag::str_custom(Some("Test")).as_str(),
                ProfileTag::str_image().as_str(),
                ProfileTag::str_nick().as_str(),
                SnsTag::str_custom(Some("Test")).as_str(),
                SnsTag::str_remark().as_str(),
                SnsTag::str_group().as_str(),
            ],
        );

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"UserID_1",
          "To_Account":
          [
              "UserID_2",
              "UserID_3",
              "UserID_4"
          ],
          "TagList":
          [
              "Tag_Profile_Custom_Test",
              "Tag_Profile_IM_Image",
              "Tag_Profile_IM_Nick",
              "Tag_SNS_Custom_Test",
              "Tag_SNS_IM_Remark",
              "Tag_SNS_IM_Group"
          ]
        });

        let req = super::FriendGetListRequest::new(
            "UserID_1",
            vec!["UserID_2", "UserID_3", "UserID_4"],
            vec![
                ProfileTag::str_custom(Some("Test")).as_str(),
                ProfileTag::str_image().as_str(),
                ProfileTag::str_nick().as_str(),
                SnsTag::str_custom(Some("Test")).as_str(),
                SnsTag::str_remark().as_str(),
                SnsTag::str_group().as_str(),
            ],
        );

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "InfoItem": [
              {
                  "To_Account": "UserID_2",
                  "SnsProfileItem": [
                      {
                          "Tag": "Tag_SNS_IM_Remark",
                          "Value": "remark_2"
                      },
                      {
                          "Tag": "Tag_SNS_IM_Group",
                          "Value": ["group1","group2"]
                      },
                      {
                          "Tag": "Tag_Profile_IM_Nick",
                          "Value": "nick_2"
                      },
                      {
                          "Tag": "Tag_SNS_Custom_Test",
                          "Value": "custom_sns_2"
                      },
                      {
                          "Tag": "Tag_Profile_Custom_Test",
                          "Value": "custom_profile_2"
                      }
                  ],
                  "ResultCode": 0,
                  "ResultInfo": ""
              }
          ],
          "ActionStatus": "OK",
          "ErrorCode": 0,
          "ErrorInfo": "",
          "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::FriendGetListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
            "InfoItem": [
                {
                    "To_Account": "UserID_2",
                    "SnsProfileItem": [
                        {
                            "Tag": "Tag_SNS_IM_Remark",
                            "Value": "remark_2"
                        },
                        {
                            "Tag": "Tag_SNS_IM_Group",
                            "Value": ["group1","group2"]
                        },
                        {
                            "Tag": "Tag_Profile_IM_Nick",
                            "Value": "nick_2"
                        },
                        {
                            "Tag": "Tag_SNS_Custom_Test",
                            "Value": "custom_sns_2"
                        },
                        {
                            "Tag": "Tag_Profile_Custom_Test",
                            "Value": "custom_profile_2"
                        }
                    ],
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "UserID_3",
                    "SnsProfileItem": [
                        {
                            "Tag": "Tag_SNS_IM_Remark",
                            "Value": "remark_3"
                        },
                        {
                            "Tag": "Tag_Profile_IM_Nick",
                            "Value": "nick_3"
                        },
                        {
                            "Tag": "Tag_Profile_Custom_Test",
                            "Value": "custom_profile_3"
                        }
                    ],
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "UserID_4",
                    "ResultCode": 30001,
                    "ResultInfo": "Err_SNS_FriendGetList_Friend_Not_Exist"
                }
            ],
            "Fail_Account": [
                "UserID_4"
            ],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::FriendGetListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
