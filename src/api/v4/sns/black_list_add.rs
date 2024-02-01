//! <https://www.tencentcloud.com/ko/document/product/1047/34911>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(black_list_add, BlackListAddRequest, BlackListAddResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListAddRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_accounts: Vec<String>,
}

impl BlackListAddRequest {
    pub fn new<S: AsRef<str>>(from_account: S, to_accounts: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_accounts: to_accounts.iter().map(|s| s.as_ref().to_string()).collect(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlackListAddResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ResultItem")]
    pub result_items: Option<Vec<ResultItem>>,

    #[serde(rename = "Fail_Account")]
    pub fail_accounts: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "ResultCode")]
    result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}

#[cfg(test)]
mod test_ {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id",
            "To_Account":["id1","id2","id3"]
        });

        let request = super::BlackListAddRequest::new("id", vec!["id1", "id2", "id3"]);

        assert_eq!(serde_json::to_value(request).unwrap(), sample);
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
                },
                {
                    "To_Account":"id2",
                    "ResultCode":30001,
                    "ResultInfo":"Err_SNS_BlackListAdd_Already_Exist"
                },
                {
                    "To_Account":"id3",
                    "ResultCode":30002,
                    "ResultInfo":"Err_SNS_BlackListAdd_SdkAppId_Illegal"
                }
            ],
            "Fail_Account":["id2","id3"],
            "ActionStatus":"OK",
            "ErrorCode":0,
            "ErrorInfo":"",
            "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::BlackListAddResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
