//! <https://www.tencentcloud.com/document/product/1047/53437>

use crate::api::v4::common::{ActionStatus, ContactItem, ErrorCode, GroupItem};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(
    create_contact_group,
    CreateContactGroupRequest,
    CreateContactGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContactGroupRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "GroupContactItem")]
    group_contact_item: Vec<GroupContactItem>,
}

impl CreateContactGroupRequest {
    pub fn new<S: AsRef<str>>(from_account: S, group_contact_item: Vec<GroupContactItem>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            group_contact_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupContactItem {
    #[serde(rename = "GroupName")]
    group_name: String,

    #[serde(rename = "ContactItem")]
    contact_item: Vec<ContactItem>,
}

impl GroupContactItem {
    pub fn new<S: AsRef<str>>(group_name: S, contact_item: Vec<ContactItem>) -> Self {
        Self {
            group_name: group_name.as_ref().to_string(),
            contact_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContactGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "GroupResultItem")]
    pub group_result_item: Option<Vec<GroupResultItem>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupResultItem {
    #[serde(rename = "GroupItem")]
    pub group_item: GroupItem,

    #[serde(rename = "ResultItem")]
    pub result_item: Vec<ResultItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "ContactItem")]
    pub contact_item: ContactItem,

    #[serde(rename = "ResultCode")]
    pub result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    pub result_info: String,
}

#[cfg(test)]
mod test_create_contact_group {
    use crate::api::v4::common::ConversationType;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"user15",
            "GroupContactItem":[
                {
                    "GroupName":"test0",
                    "ContactItem": [
                        {
                            "Type": 1,
                            "To_Account": "user0"
                        }
                    ]
                }
            ]
        });

        let mut contact_item = super::ContactItem::new(ConversationType::OneToOne);
        contact_item.set_to_account(Some("user0"));

        let group_contact_item = super::GroupContactItem::new("test0", vec![contact_item]);

        let req = super::CreateContactGroupRequest::new("user15", vec![group_contact_item]);

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "GroupResultItem": [
                {
                    "GroupItem": {
                        "GroupName": "test2",
                        "GroupId": 2
                    },
                    "ResultItem": [
                        {
                            "ContactItem": {
                                "Type": 1,
                                "To_Account": "user1"
                            },
                            "ResultCode": 0,
                            "ResultInfo": ""
                        }
                    ]
                }
            ],
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res =
            serde_json::from_value::<super::CreateContactGroupResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
