//! <https://www.tencentcloud.com/ko/document/product/1047/34908>

use crate::api::v4::common::{ActionStatus, ErrorCode, SnsTag};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_get, FriendGetRequest, FriendGetResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendGetRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "StartIndex")]
    start_index: u32,

    #[serde(rename = "StandardSequence")]
    standard_sequence: Option<u32>,

    #[serde(rename = "CustomSequence")]
    custom_sequence: Option<u32>,
}

impl FriendGetRequest {
    pub fn new<S: AsRef<str>>(from_account: S, start_index: u32) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            start_index,
            standard_sequence: None,
            custom_sequence: None,
        }
    }

    pub fn set_standard_sequence(&mut self, standard_sequence: Option<u32>) -> &mut Self {
        self.standard_sequence = standard_sequence;
        self
    }

    pub fn set_custom_sequence(&mut self, custom_sequence: Option<u32>) -> &mut Self {
        self.custom_sequence = custom_sequence;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendGetResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UserDataItem")]
    pub user_data_items: Option<Vec<UserDataItem>>,

    #[serde(rename = "StandardSequence")]
    pub standard_sequence: Option<u32>,

    #[serde(rename = "CustomSequence")]
    pub custom_sequence: Option<u32>,

    #[serde(rename = "FriendNum")]
    pub friend_num: Option<u32>,

    #[serde(rename = "CompleteFlag")]
    pub complete_flag: Option<u32>,

    #[serde(rename = "NextStartIndex")]
    pub next_start_index: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDataItem {
    #[serde(rename = "To_Account")]
    /// 친구 추가 대상 아이디
    to_account: String,

    #[serde(rename = "ValueItem")]
    value_items: Vec<ValueItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueItem {
    #[serde(rename = "Tag")]
    tag: String,

    #[serde(rename = "Value")]
    value: SnsTag,
}

impl ValueItem {
    pub fn new<S: AsRef<str>>(tag: S, value: SnsTag) -> Self {
        Self {
            tag: tag.as_ref().to_string(),
            value,
        }
    }
}

#[cfg(test)]
mod test_friend_get {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account": "id",
            "StartIndex": 0,
            "StandardSequence": 0,
            "CustomSequence": 0
        });

        let mut req = super::FriendGetRequest::new("id", 0);
        req.set_standard_sequence(Some(0));
        req.set_custom_sequence(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn reponse1() {
        let sample = json!({
            "UserDataItem": [
                {
                    "To_Account": "id1",
                    "ValueItem": [
                        {
                            "Tag": "Tag_SNS_IM_AddSource",
                            "Value": "AddSource_Type_Android"
                        },
                        {
                            "Tag": "Tag_SNS_IM_Remark",
                            "Value": "Remark1"
                        },
                        {
                            "Tag": "Tag_SNS_IM_Group",
                            "Value":["Group1","Group2"]
                        },
                        {
                            "Tag": "Tag_SNS_IM_AddTime",
                            "Value": 1563867420
                        },
                        {
                            "Tag": "Tag_SNS_Custom_Test",
                            "Value": "CustomData1"
                        }
                    ]
                },
                {
                    "To_Account": "id2",
                    "ValueItem": [
                        {
                            "Tag": "Tag_SNS_IM_AddSource",
                            "Value": "AddSource_Type_IOS"
                        },
                        {
                            "Tag": "Tag_SNS_IM_Group",
                            "Value":["Group1"]
                        },
                        {
                            "Tag": "Tag_SNS_IM_AddTime",
                            "Value": 1563867425
                        }
                    ]
                }
            ],
            "StandardSequence": 88,
            "CustomSequence": 46,
            "FriendNum": 20,
            "CompleteFlag": 1,
            "NextStartIndex": 0,
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::FriendGetResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
