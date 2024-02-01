//! <https://www.tencentcloud.com/ko/document/product/1047/34909>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    set_unread_msg_num,
    SetUnreadMsgNumRequest,
    SetUnreadMsgNumResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetUnreadMsgNumRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Member_Account")]
    member_account: String,

    #[serde(rename = "UnreadMsgNum")]
    unread_msg_num: u32,
}

impl SetUnreadMsgNumRequest {
    pub fn new<S: AsRef<str>>(group_id: S, member_account: S, unread_msg_num: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            member_account: member_account.as_ref().to_string(),
            unread_msg_num,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SetUnreadMsgNumResponse {
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
mod test_set_unread_msg_num {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#2CLUZEAEJ",  // Target group (required)
            "Member_Account": "bob",  // Target group member (required)
            "UnreadMsgNum":5          // Unread message count of the target member
        });

        let req = super::SetUnreadMsgNumRequest::new("@TGS#2CLUZEAEJ", "bob", 5);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0
        });

        let res = serde_json::from_value::<super::SetUnreadMsgNumResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
