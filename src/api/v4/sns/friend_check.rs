//! <https://www.tencentcloud.com/ko/document/product/1047/34907>
use crate::api::v4::common::{ActionStatus, ErrorCode, FriendCheckType, FriendRelationType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_check, FriendCheckRequest, FriendCheckResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendCheckRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_account: Vec<String>,

    #[serde(rename = "CheckType")]
    check_type: FriendCheckType,
}

impl FriendCheckRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        to_account: Vec<S>,
        check_type: FriendCheckType,
    ) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
            check_type,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendCheckResponse {
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

    #[serde(rename = "Relation")]
    relation: Option<FriendRelationType>,

    #[serde(rename = "ResultCode")]
    result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}

#[cfg(test)]
mod test_friend_check {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id",
            "To_Account":["id1","id2","id3","id4","id5"],
            "CheckType":"CheckResult_Type_Both"
        });

        let req = super::FriendCheckRequest::new(
            "id",
            vec!["id1", "id2", "id3", "id4", "id5"],
            super::FriendCheckType::CheckResultTypeBoth,
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "InfoItem": [
                {
                    "To_Account": "id1",
                    "Relation": "CheckResult_Type_BothWay",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id2",
                    "Relation": "CheckResult_Type_AWithB",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id3",
                    "Relation": "CheckResult_Type_BWithA",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id4",
                    "Relation": "CheckResult_Type_NoRelation",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id5",
                    "Relation": "CheckResult_Type_NoRelation",
                    "ResultCode": 30006,
                    "ResultInfo": "Err_SNS_FriendCheck_Check_Relation_Exec_Task_Fail"
                }
            ],
            "Fail_Account": ["id5"],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::FriendCheckResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
