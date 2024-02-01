//! <https://www.tencentcloud.com/document/product/1047/34924>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(getnospeaking, GetNoSpeakingRequest, GetNoSpeakingResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNoSpeakingRequest {
    #[serde(rename = "Get_Account")]
    get_account: String,
}

impl GetNoSpeakingRequest {
    pub fn new<S: AsRef<str>>(get_account: S) -> Self {
        Self {
            get_account: get_account.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNoSpeakingResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "C2CmsgNospeakingTime")]
    pub c2c_msg_no_speaking_time: Option<u32>,

    #[serde(rename = "GroupmsgNospeakingTime")]
    pub group_msg_no_speaking_time: Option<u32>,
}

#[cfg(test)]
mod test_get_no_speaking {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!(
                    {
            "Get_Account": "lumotuwe"
        });

        let req = super::GetNoSpeakingRequest::new("lumotuwe");

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ErrorCode": 0,
            "ErrorInfo": "",
            "C2CmsgNospeakingTime": 4294967295u32,
            "GroupmsgNospeakingTime": 7196
        });

        let res = serde_json::from_value::<super::GetNoSpeakingResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
