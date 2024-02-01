//! <https://www.tencentcloud.com/ko/document/product/1047/34621#profile-management>

use crate::api::v4::common::{ActionStatus, BlackListRelationType, BlackListVerifyType, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    black_list_check,
    BlackListCheckRequest,
    BlackListCheckResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListCheckRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_account: Vec<String>,

    #[serde(rename = "CheckType")]
    check_type: BlackListVerifyType,
}

impl BlackListCheckRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        to_account: Vec<S>,
        check_type: BlackListVerifyType,
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
pub struct BlackListCheckResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "BlackListCheckItem")]
    pub black_list_check_items: Option<Vec<BlackListInfoItem>>,

    #[serde(rename = "Fail_Account")]
    pub fail_accounts: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListInfoItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "Relation")]
    relation: BlackListRelationType,

    #[serde(rename = "ResultCode")]
    result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}

#[cfg(test)]
mod test_black_list_check {
    use crate::api::v4::common::BlackListVerifyType;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id",
            "To_Account":["id1","id2","id3","id4","id5"],
            "CheckType":"BlackCheckResult_Type_Both"
        });

        let req = super::BlackListCheckRequest::new(
            "id",
            vec!["id1", "id2", "id3", "id4", "id5"],
            BlackListVerifyType::BlackCheckResultTypeBoth,
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "BlackListCheckItem": [
                {
                    "To_Account": "id1",
                    "Relation": "BlackCheckResult_Type_BothWay",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id2",
                    "Relation": "BlackCheckResult_Type_AWithB",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id3",
                    "Relation": "BlackCheckResult_Type_BWithA",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id4",
                    "Relation": "BlackCheckResult_Type_NO",
                    "ResultCode": 0,
                    "ResultInfo": ""
                },
                {
                    "To_Account": "id5",
                    "Relation": "BlackCheckResult_Type_NO",
                    "ResultCode": 30007,
                    "ResultInfo": "Err_SNS_BlackListCheck_Check_Reverse_BlackList_Fail"
                }
            ],
            "Fail_Account": ["id5"],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::BlackListCheckResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
