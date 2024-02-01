//! <https://www.tencentcloud.com/document/product/1047/34916>

use crate::api::v4::common::{ActionStatus, ErrorCode, ProfileTag};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(portrait_set, PortraitSetRequest, PortraitSetResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortraitSetRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "ProfileItem")]
    profile_item: Vec<ProfileItem>,
}

impl PortraitSetRequest {
    pub fn new(from_account: String, profile_item: Vec<ProfileItem>) -> Self {
        Self {
            from_account,
            profile_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileItem {
    #[serde(rename = "Tag")]
    tag: String,

    #[serde(rename = "Value")]
    value: ProfileTag,
}

impl ProfileItem {
    pub fn new(tag: String, value: ProfileTag) -> Self {
        Self { tag, value }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortraitSetResponse {
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
mod test_portrait_set {
    use crate::api::v4::common::ProfileTag;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id",
            "ProfileItem":
            [
                {
                    "Tag":"Tag_Profile_IM_Nick",
                    "Value":"MyNickName"
                }
            ]
        });

        let req = super::PortraitSetRequest::new(
            "id".to_string(),
            vec![super::ProfileItem::new(
                ProfileTag::str_nick(),
                ProfileTag::tag_nick("MyNickName"),
            )],
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn reponse1() {
        let sample = json!({
            "ActionStatus":"FAIL",
            "ErrorCode":40001,
            "ErrorInfo":"Err_Profile_Comm_Decode_Fail",
            "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::PortraitSetResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
