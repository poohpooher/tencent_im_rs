//! <https://www.tencentcloud.com/document/product/1047/50295>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    get_group_ban_member,
    GetGroupBanMemberRequest,
    GetGroupBanMemberResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupBanMemberRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "Offset")]
    offset: u32,

    #[serde(rename = "Limit")]
    limit: Option<u32>,
}

impl GetGroupBanMemberRequest {
    pub fn new<S: AsRef<str>>(group_id: S, offset: u32) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            offset,
            limit: None,
        }
    }

    pub fn set_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limit = limit;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupBanMemberResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "BannedAccountList")]
    pub banned_account_list: Option<Vec<BannedAccount>>,

    #[serde(rename = "NextOffset")]
    pub next_offset: Option<u32>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BannedAccount {
    #[serde(rename = "Member_Account")]
    pub member_account: String,

    #[serde(rename = "BannedUntil")]
    pub shut_up_until: u64,
}

#[cfg(test)]
mod test_get_group_ban_member {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#aJRGC4MH6",  // Group ID
            "Limit":20, // Number of banned members to be pulled per page each time. The maximum value allowed is 100.
            "Offset": 0 // Offset. For the first request, set `Offset` to 0. For subsequent requests, set `Offset` to the value of `NextOffset` in the response packet. If `NextOffset` is 0, the entire list of banned members of the audio-video group is obtained.
        });

        let mut req = super::GetGroupBanMemberRequest::new("@TGS#aJRGC4MH6", 0);
        req.set_limit(Some(20));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "BannedAccountList": [
                {
                    "BannedUntil": 1660875916,
                    "Member_Account": "brennanli1"
                },
                {
                    "BannedUntil": 1660875916,
                    "Member_Account": "brennanli12"
                }
            ],
            "ErrorCode": 0,
            "ErrorInfo": "",
            "NextOffset": 2
        });

        let res =
            serde_json::from_value::<super::GetGroupBanMemberResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
