//! <https://www.tencentcloud.com/document/product/1047/34885>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(gethistory, GetHistoryRequest, GetHistoryResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetHistoryRequest {
    #[serde(rename = "ChatType")]
    pub chat_type: String,

    #[serde(rename = "MsgTime")]
    pub msg_time: String,
}

impl GetHistoryRequest {
    pub fn new<S: AsRef<str>>(chat_type: S, msg_time: S) -> Self {
        Self {
            chat_type: chat_type.as_ref().to_string(),
            msg_time: msg_time.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetHistoryResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "File")]
    pub file: Option<Vec<File>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "URL")]
    pub url: String,

    #[serde(rename = "ExpireTime")]
    pub expire_time: String,

    #[serde(rename = "FileSize")]
    pub file_size: u64,

    #[serde(rename = "FileMD5")]
    pub file_md5: String,

    #[serde(rename = "GzipSize")]
    pub gzip_size: u64,

    #[serde(rename = "GzipMD5")]
    pub gzip_md5: String,
}

#[cfg(test)]
mod test_get_history {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "ChatType": "C2C",
            "MsgTime": "2015120121"
        });

        let req = super::GetHistoryRequest::new("C2C", "2015120121");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "File": [
                {
                    "URL": "https://download.tim.qq.com/msg_history/2/9b8f8f063b73f61698ce11e58207e89ade40.gz",
                    "ExpireTime": "2015-12-02 16:45:23",
                    "FileSize": 65207,
                    "FileMD5": "cceece008bb7f469a47cf8c4b7acb84e",
                    "GzipSize": 1815,
                    "GzipMD5": "c3a0269dde393fd7a8bb18bfdeaeee2e"
                }
            ],
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res = serde_json::from_value::<super::GetHistoryResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
