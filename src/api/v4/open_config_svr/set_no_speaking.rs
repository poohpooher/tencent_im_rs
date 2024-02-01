//! <https://www.tencentcloud.com/document/product/1047/34923>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(setnospeaking, SetNoSpeakingRequest, SetNoSpeakingResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetNoSpeakingRequest {
    #[serde(rename = "Set_Account")]
    set_account: String,

    #[serde(rename = "C2CmsgNospeakingTime")]
    c2c_msg_no_speaking_time: Option<u32>,

    #[serde(rename = "GroupmsgNospeakingTime")]
    group_msg_no_speaking_time: Option<u32>,
}

impl SetNoSpeakingRequest {
    pub fn new<S: AsRef<str>>(set_account: S) -> Self {
        Self {
            set_account: set_account.as_ref().to_string(),
            c2c_msg_no_speaking_time: None,
            group_msg_no_speaking_time: None,
        }
    }

    pub fn set_c2c_msg_no_speaking_time(
        &mut self,
        c2c_msg_no_speaking_time: Option<u32>,
    ) -> &mut Self {
        self.c2c_msg_no_speaking_time = c2c_msg_no_speaking_time;
        self
    }

    pub fn set_group_msg_no_speaking_time(
        &mut self,
        group_msg_no_speaking_time: Option<u32>,
    ) -> &mut Self {
        self.group_msg_no_speaking_time = group_msg_no_speaking_time;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetNoSpeakingResponse {
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
mod test_set_no_speaking {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "Set_Account": "lumotuwe",
            "C2CmsgNospeakingTime": 4294967295u32, // `C2CmsgNospeakingTime` and `GroupmsgNospeakingTime` are optional, but either of them must be specified.
            "GroupmsgNospeakingTime": 7200
        });

        let mut req = super::SetNoSpeakingRequest::new("lumotuwe");
        req.set_c2c_msg_no_speaking_time(Some(4294967295));
        req.set_group_msg_no_speaking_time(Some(7200));

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response2() {
        let sample = json!({
            "ErrorCode": 0,
            "ErrorInfo": "",
        });

        let res = serde_json::from_value::<super::SetNoSpeakingResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
