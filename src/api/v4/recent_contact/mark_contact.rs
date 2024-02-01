//! <https://www.tencentcloud.com/document/product/1047/53438>

use crate::api::v4::common::{ActionStatus, ContactItem, ErrorCode, MarkOptType};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(mark_contact, MarkContactRequest, MarkContactResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkContactRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "MarkItem")]
    mark_item: Vec<MarkItem>,
}

impl MarkContactRequest {
    pub fn new<S: AsRef<str>>(from_account: S, mark_item: Vec<MarkItem>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            mark_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkItem {
    #[serde(rename = "OptType")]
    mark_opt_type: MarkOptType,

    #[serde(rename = "ContactItem")]
    contact_item: ContactItem,

    #[serde(rename = "ClearMark")]
    clear_mark: Option<Vec<u32>>,

    #[serde(rename = "SetMark")]
    set_mark: Option<Vec<u32>>,

    #[serde(rename = "CustomMark")]
    custom_mark: Option<String>,
}

impl MarkItem {
    pub fn new(mark_opt_type: MarkOptType, contact_item: ContactItem) -> Self {
        Self {
            mark_opt_type,
            contact_item,
            clear_mark: None,
            set_mark: None,
            custom_mark: None,
        }
    }

    pub fn set_clear_mark(&mut self, clear_mark: Option<Vec<u32>>) -> &mut Self {
        self.clear_mark = clear_mark;
        self
    }

    pub fn set_set_mark(&mut self, set_mark: Option<Vec<u32>>) -> &mut Self {
        self.set_mark = set_mark;
        self
    }

    pub fn set_custom_mark<S: AsRef<str>>(&mut self, custom_mark: Option<S>) -> &mut Self {
        self.custom_mark = custom_mark.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkContactResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ResultItem")]
    pub result_item: Option<Vec<ResultItem>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "OptType")]
    pub mark_opt_type: MarkOptType,

    #[serde(rename = "ContactItem")]
    pub contact_item: ContactItem,

    #[serde(rename = "ResultCode")]
    pub result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    pub result_info: String,
}

#[cfg(test)]
mod test_mark_contact {
    use crate::api::v4::common::{ContactItem, ConversationType};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"user0",
            "MarkItem":[
                {
                    "OptType":3,
                    "ContactItem":{
                        "Type":1,
                        "To_Account":"user1"
                    },
                    "SetMark":[1,2,3],
                    "CustomMark":"abcd"
                }
            ]
        });

        let mut contact_item = ContactItem::new(ConversationType::OneToOne);
        contact_item.set_to_account(Some("user1"));

        let mut mark_item =
            super::MarkItem::new(super::MarkOptType::StandardAndCustom, contact_item);
        mark_item
            .set_set_mark(Some(vec![1, 2, 3]))
            .set_custom_mark(Some("abcd"));

        let req = super::MarkContactRequest::new("user0", vec![mark_item]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ResultItem": [
                {
                    "OptType":3,
                    "ContactItem": {
                        "Type": 1,
                        "To_Account": "user1"
                    },
                    "ResultCode": 0,
                    "ResultInfo": ""
                }
            ],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::MarkContactResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
