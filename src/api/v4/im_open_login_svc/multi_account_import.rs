//! <https://www.tencentcloud.com/ko/document/product/1047/34954>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    multiaccount_import,
    MultiAccountImportRequest,
    MultiAccountImportResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiAccountImportRequest {
    #[serde(rename = "Accounts")]
    /// 멤버 아이디
    accounts: Vec<String>,
}

impl MultiAccountImportRequest {
    pub fn new<S: AsRef<str>>(accounts: Vec<S>) -> Self {
        let mut accounts_vec = Vec::new();
        for account in accounts {
            accounts_vec.push(account.as_ref().to_string());
        }

        Self {
            accounts: accounts_vec,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiAccountImportResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "FailAccounts")]
    pub fail_accounts: Option<Vec<String>>,
}

#[cfg(test)]
mod test_multi_account_import {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
        "Accounts":["test1","test2","test3","test4","test5"]
        });

        let req = super::MultiAccountImportRequest::new(vec![
            "test1", "test2", "test3", "test4", "test5",
        ]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "FailAccounts": [
                "test3",
                "test4"
            ]
        });

        let res =
            serde_json::from_value::<super::MultiAccountImportResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
