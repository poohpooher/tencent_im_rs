//! <https://www.tencentcloud.com/ko/document/product/1047/34955>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(account_delete, AccountDeleteRequest, AccountDeleteResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDeleteRequest {
    #[serde(rename = "DeleteItem")]
    delete_items: Vec<DeleteItem>,
}

impl AccountDeleteRequest {
    pub fn new(delete_items: Vec<DeleteItem>) -> Self {
        Self { delete_items }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteItem {
    #[serde(rename = "UserID")]
    user_id: String,
}

impl DeleteItem {
    pub fn new<S: AsRef<str>>(user_id: S) -> Self {
        Self {
            user_id: user_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDeleteResponse {
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
}

#[cfg(test)]
mod test_account_delete {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "DeleteItem":
          [
              {
                  "UserID":"UserID_1"
              },
              {
                  "UserID":"UserID_2"
              }
          ]
        });

        let req = super::AccountDeleteRequest::new(vec![
            super::DeleteItem::new("UserID_1"),
            super::DeleteItem::new("UserID_2"),
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
                    "ResultCode": 0,
                    "ResultInfo": "",
                    "UserID": "UserID_1"
                },
                {
                    "ResultCode": 70107,
                    "ResultInfo": "Err_TLS_PT_Open_Login_Account_Not_Exist",
                    "UserID": "UserID_2"
                }
            ]
        });

        let res = serde_json::from_value::<super::AccountDeleteResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
