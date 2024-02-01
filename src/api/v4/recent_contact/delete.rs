//! <https://www.tencentcloud.com/document/product/1047/43088>

use crate::api::v4::common::{ActionStatus, ConversationType, ErrorCode};
use crate::api::{bool_to_int, int_to_bool};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(delete, DeleteRequest, DeleteResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "Type")]
    conversation_type: ConversationType,

    #[serde(rename = "To_Account")]
    to_account: Option<String>,

    #[serde(rename = "ToGroupid")]
    to_group_id: Option<String>,

    #[serde(
        rename = "ClearRamble",
        serialize_with = "bool_to_int",
        deserialize_with = "int_to_bool"
    )]
    clear_ramble: Option<bool>,
}

impl DeleteRequest {
    pub fn new<S: AsRef<str>>(from_account: S, conversation_type: ConversationType) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            conversation_type,
            to_account: None,
            to_group_id: None,
            clear_ramble: None,
        }
    }

    pub fn set_to_account<S: AsRef<str>>(&mut self, to_account: Option<S>) -> &mut Self {
        self.to_account = to_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_id<S: AsRef<str>>(&mut self, to_group_id: Option<S>) -> &mut Self {
        self.to_group_id = to_group_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_clear_ramble(&mut self, clear_ramble: Option<bool>) -> &mut Self {
        self.clear_ramble = clear_ramble;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteResponse {
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
mod test_delete {
    use crate::api::v4::common::ConversationType;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"id1",
            "Type":1,
            "To_Account":"id2",
            "ClearRamble":1
        });

        let mut req = super::DeleteRequest::new("id1", ConversationType::OneToOne);
        req.set_to_account(Some("id2")).set_clear_ramble(Some(true));

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn request2() {
        let sample = json!({
            "From_Account":"id1",
            "Type":2,
            "ToGroupid":"id2",
            "ClearRamble":1
        });

        let mut req = super::DeleteRequest::new("id1", ConversationType::Group);
        req.set_group_id(Some("id2")).set_clear_ramble(Some(true));

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::DeleteResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
