//! <https://www.tencentcloud.com/ko/document/product/1047/34966>

use crate::api::v4::common::action_status::ActionStatus;
use crate::api::v4::common::error_code::ErrorCode;
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    change_group_owner,
    ChangeGroupOwnerRequest,
    ChangeGroupOwnerResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeGroupOwnerRequest {
    #[serde(rename = "GroupId")]
    group_id: String,

    #[serde(rename = "NewOwner_Account")]
    new_owner_account: String,
}

impl ChangeGroupOwnerRequest {
    pub fn new<S: AsRef<str>>(group_id: S, new_owner_account: S) -> Self {
        Self {
            group_id: group_id.as_ref().to_string(),
            new_owner_account: new_owner_account.as_ref().to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeGroupOwnerResponse {
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
mod test_change_group_owner {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "GroupId": "@TGS#1NVTZEAE4",  // (Required) ID of the group whose ownership is to be transferred
            "NewOwner_Account": "peter" // (Required) ID of the new group owner
        });

        let req = super::ChangeGroupOwnerRequest::new("@TGS#1NVTZEAE4", "peter");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode":0
        });

        let res =
            serde_json::from_value::<super::ChangeGroupOwnerResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
