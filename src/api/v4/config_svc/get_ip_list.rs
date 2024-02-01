//! <https://www.tencentcloud.com/document/product/1047/36742>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(GetIPList, GetIpListRequest, GetIpListResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIpListRequest {}

impl Default for GetIpListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl GetIpListRequest {
    pub fn new() -> Self {
        Self {}
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetIpListResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "IPList")]
    pub ip_list: Option<Vec<String>>,
}

#[cfg(test)]
mod test_get_ip_list {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({});

        let req = super::GetIpListRequest::new();
        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "IPList": [ "127.0.0.2",   "183.192.202.0/25" ]
        });

        let res = serde_json::from_value::<super::GetIpListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
