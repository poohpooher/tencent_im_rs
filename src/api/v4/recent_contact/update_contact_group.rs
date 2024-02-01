//! <https://www.tencentcloud.com/document/product/1047/53439>

use crate::api::v4::common::{
    ActionStatus, ContactItem, ContactOptType, ErrorCode, UpdateGroupType, UpdateType,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    update_contact_group,
    UpdateContactGroupRequest,
    UpdateContactGroupResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateContactGroupRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "UpdateType")]
    update_type: UpdateType,

    #[serde(rename = "UpdateGroup")]
    update_group: UpdateGroup,
}

impl UpdateContactGroupRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        update_type: UpdateType,
        update_group: UpdateGroup,
    ) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            update_type,
            update_group,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroup {
    #[serde(rename = "UpdateGroupType")]
    update_group_type: UpdateGroupType,

    #[serde(rename = "OldGroupName")]
    old_group_name: Option<String>,

    #[serde(rename = "NewGroupName")]
    new_group_name: Option<String>,

    #[serde(rename = "ContactUpdateItem")]
    contact_update_item: Vec<ContactUpdateItem>,
}
impl UpdateGroup {
    pub fn new(
        update_group_type: UpdateGroupType,
        contact_update_item: Vec<ContactUpdateItem>,
    ) -> Self {
        Self {
            update_group_type,
            old_group_name: None,
            new_group_name: None,
            contact_update_item,
        }
    }

    pub fn set_old_group_name<S: AsRef<str>>(&mut self, old_group_name: Option<S>) -> &mut Self {
        self.old_group_name = old_group_name.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_new_group_name<S: AsRef<str>>(&mut self, new_group_name: Option<S>) -> &mut Self {
        self.new_group_name = new_group_name.map(|s| s.as_ref().to_string());
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactUpdateItem {
    #[serde(rename = "ContactOptType")]
    contact_opt_type: ContactOptType,

    #[serde(rename = "ContactItem")]
    contact_item: ContactItem,
}

impl ContactUpdateItem {
    pub fn new(contact_opt_type: ContactOptType, contact_item: ContactItem) -> Self {
        Self {
            contact_opt_type,
            contact_item,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateContactGroupResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "UpdateType")]
    pub update_type: Option<UpdateType>,

    #[serde(rename = "UpdateGroupResult")]
    pub update_group_result: Option<UpdateGroupResult>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupResult {
    #[serde(rename = "UpdateGroupType")]
    pub update_group_type: UpdateGroupType,

    #[serde(rename = "GroupName")]
    pub group_name: String,

    #[serde(rename = "GroupId")]
    pub group_id: u32,

    #[serde(rename = "OldGroupName")]
    pub old_group_name: Option<String>,

    #[serde(rename = "ContactResultItem")]
    pub contact_result_item: Vec<ContactOptResultItem>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactOptResultItem {
    #[serde(rename = "ContactOptType")]
    pub contact_opt_type: ContactOptType,

    #[serde(rename = "ContactItem")]
    pub contact_item: ContactItem,

    #[serde(rename = "ResultCode")]
    pub result_code: ErrorCode,

    #[serde(rename = "ResultInfo")]
    pub result_info: String,
}

#[cfg(test)]
mod test_update_contact_group {
    use serde_json::json;

    use crate::api::v4::common::{ContactItem, ConversationType};

    #[test]
    fn request1() {
        let sample = json!({
            "From_Account":"user20",
            "UpdateType":1,
            "UpdateGroup":{
                "UpdateGroupType":2,
                "OldGroupName":"test1",
                "ContactUpdateItem":[
                    {
                        "ContactOptType":1,
                        "ContactItem":{
                            "Type": 1,
                            "To_Account": "user1"
                        }
                    }
                ]
            }
        });

        let mut contact_item = ContactItem::new(ConversationType::OneToOne);
        contact_item.set_to_account(Some("user1"));

        let contact_update_item =
            super::ContactUpdateItem::new(super::ContactOptType::Add, contact_item);

        let mut update_group = super::UpdateGroup::new(
            super::UpdateGroupType::Conversation,
            vec![contact_update_item],
        );
        update_group.set_old_group_name(Some("test1"));

        let req = super::UpdateContactGroupRequest::new(
            "user20",
            super::UpdateType::AddOrDelete,
            update_group,
        );

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response1() {
        let sample = json!({
            "UpdateType": 1,
            "UpdateGroupResult": {
                "UpdateGroupType": 2,
                "GroupName": "test1",
                "GroupId": 1,
                "OldGroupName": "test1",
                "ContactResultItem": [
                    {
                        "ContactOptType": 1,
                        "ContactItem": {
                            "Type": 1,
                            "To_Account": "user1"
                        },
                        "ResultCode": 0,
                        "ResultInfo": ""
                    }
                ]
            },
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": ""
        });

        let res =
            serde_json::from_value::<super::UpdateContactGroupResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
