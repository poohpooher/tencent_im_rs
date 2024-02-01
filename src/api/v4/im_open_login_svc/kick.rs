//! <https://www.tencentcloud.com/ko/document/product/1047/34957>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(kick, KickRequest, KickResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KickRequest {
    #[serde(rename = "UserID")]
    user_id: String,
}

impl KickRequest {
    pub fn new<S: AsRef<str>>(user_id: S) -> Self {
        Self {
            user_id: user_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KickResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,
}

#[cfg(test)]
mod test_kick {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
           "UserID":"test"
        });

        let req = serde_json::to_value(super::KickRequest::new("test")).unwrap();

        assert_eq!(req, sample, "kick request serialization failed");
    }

    #[test]
    fn response1() {
        let sample = json!({
           "ActionStatus":"OK",
           "ErrorInfo":"",
           "ErrorCode":0
        });

        let res = serde_json::from_value::<super::KickResponse>(sample.clone()).unwrap();

        assert_eq!(
            serde_json::to_value(res).unwrap(),
            sample,
            "kick response deserialization failed"
        );
    }
}
