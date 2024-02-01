//! <https://www.tencentcloud.com/document/product/1047/53442>

use crate::api::v4::common::{ActionStatus, ContactItem, ErrorCode, GroupItem};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    search_contact_group,
    SearchContactGroupRequest,
    SearchContactGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchContactGroupRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "ContactItem")]
    contact_item: Vec<ContactItem>,
}

impl SearchContactGroupRequest {
    pub fn new<S: AsRef<str>>(from_account: S, contact_item: Vec<ContactItem>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            contact_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchContactGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ContactResultItem")]
    pub contact_result_item: Option<Vec<ContactItem>>,

    #[serde(rename = "GroupItem")]
    pub group_item: Option<Vec<GroupItem>>,
}

#[cfg(test)]
mod test_search_contact_group {
    use serde_json::json;

    use crate::api::v4::common::{ContactItem, ConversationType};

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"yahaha",
            "ContactItem":[
                {
                    "Type":1,
                    "To_Account":"xixi"
                }
            ]
        });

        let mut contact_item = ContactItem::new(ConversationType::OneToOne);
        contact_item.set_to_account(Some("xixi"));

        let req = super::SearchContactGroupRequest::new("yahaha", vec![contact_item]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ContactResultItem": [
                {
                    "Type": 1,
                    "To_Account": "xixi",
                    "StandardMark": "11111110",
                    "CustomMark": "xxyyyzzzfffffffxxx111111",
                    "ContactGroupId": [
                        1,
                        2
                    ],
                    "Timestamp": 1673500546
                }
            ],
            "GroupItem": [
                {
                    "GroupName": "test1x",
                    "GroupId": 1
                },
                {
                    "GroupName": "test10",
                    "GroupId": 2
                }
            ],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res =
            serde_json::from_value::<super::SearchContactGroupResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
