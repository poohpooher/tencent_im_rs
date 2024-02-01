//! <https://www.tencentcloud.com/ko/document/product/1047/38521>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_online_member_num,
    GetOnlineMemberNumRequest,
    GetOnlineMemberNumResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOnlineMemberNumRequest {
    #[serde(rename = "GroupId")]
    group_id: String,
}

impl GetOnlineMemberNumRequest {
    pub fn new<S: AsRef<str>>(group_id: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOnlineMemberNumResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "OnlineMemberNum")]
    pub online_member_num: Option<u32>,
}

#[cfg(test)]
mod test_get_online_member_num {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId":"@TGS#a6I4ZUUGO"
        });

        let req = super::GetOnlineMemberNumRequest::new("@TGS#a6I4ZUUGO");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus":"OK",
            "ErrorInfo":"",
            "ErrorCode": 0,
            "OnlineMemberNum":1000 // Number of online users
        });

        let res =
            serde_json::from_value::<super::GetOnlineMemberNumResponse>(sample.clone()).unwrap();

        assert_eq!(res.online_member_num, Some(1000));
    }
}
