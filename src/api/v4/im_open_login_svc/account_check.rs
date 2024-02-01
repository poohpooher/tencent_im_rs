//! <https://www.tencentcloud.com/ko/document/product/1047/34956>

use crate::api::v4::common::{AccountStatus, ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(account_check, AccountCheckRequest, AccountCheckResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCheckRequest {
    #[serde(rename = "CheckItem")]
    check_items: Vec<CheckItem>,
}

impl AccountCheckRequest {
    pub fn new(check_items: Vec<CheckItem>) -> Self {
        Self { check_items }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckItem {
    #[serde(rename = "UserID")]
    user_id: String,
}

impl CheckItem {
    pub fn new<S: AsRef<str>>(user_id: S) -> Self {
        Self {
            user_id: user_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountCheckResponse {
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
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "UserID")]
    pub user_id: String,

    #[serde(rename = "ResultCode")]
    pub result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    pub result_info: String,

    #[serde(rename = "AccountStatus")]
    pub account_status: AccountStatus,
}

#[cfg(test)]
mod test_account_check {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "CheckItem":
          [
              {
                  "UserID":"UserID_1"
              },
              {
                  "UserID":"UserID_2"
              }
          ]
        });

        let req = super::AccountCheckRequest::new(vec![
            super::CheckItem::new("UserID_1"),
            super::CheckItem::new("UserID_2"),
        ]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ResultItem": [
                {
                    "UserID": "UserID_1",
                    "ResultCode": 0,
                    "ResultInfo": "",
                    "AccountStatus": "Imported"
                },
                {
                    "UserID": "UserID_2",
                    "ResultCode": 0,
                    "ResultInfo": "",
                    "AccountStatus": "Imported"
                }
            ]
        });

        let res = serde_json::from_value::<super::AccountCheckResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
