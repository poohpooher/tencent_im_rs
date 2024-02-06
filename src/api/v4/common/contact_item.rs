use crate::api::v4::common::ConversationType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactItem {
    #[serde(rename = "Type")]
    conversation_type: ConversationType,

    #[serde(rename = "To_Account")]
    to_account: Option<String>,

    #[serde(rename = "ToGroupId")]
    to_group_id: Option<String>,

    #[serde(rename = "StandardMark")]
    standard_mark: Option<String>,

    #[serde(rename = "CustomMark")]
    custom_mark: Option<String>,

    #[serde(rename = "ContactGroupId")]
    contact_group_id: Option<Vec<u32>>,

    #[serde(rename = "Timestamp")]
    timestamp: Option<u64>,
}

impl ContactItem {
    pub fn new(conversation_type: ConversationType) -> Self {
        Self {
            conversation_type,
            to_group_id: None,
            to_account: None,
            standard_mark: None,
            custom_mark: None,
            contact_group_id: None,
            timestamp: None,
        }
    }

    pub fn set_to_group_id<S: AsRef<str>>(&mut self, to_group_id: Option<S>) -> &mut Self {
        self.to_group_id = to_group_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_to_account<S: AsRef<str>>(&mut self, to_account: Option<S>) -> &mut Self {
        self.to_account = to_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_standard_mark<S: AsRef<str>>(&mut self, standard_mark: Option<S>) -> &mut Self {
        self.standard_mark = standard_mark.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_custom_mark<S: AsRef<str>>(&mut self, custom_mark: Option<S>) -> &mut Self {
        self.custom_mark = custom_mark.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_contact_group_id(&mut self, contact_group_id: Option<Vec<u32>>) -> &mut Self {
        self.contact_group_id = contact_group_id;
        self
    }

    pub fn set_timestamp(&mut self, timestamp: Option<u64>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    pub fn conversation_type(&self) -> &ConversationType {
        &self.conversation_type
    }

    pub fn to_group_id(&self) -> Option<&str> {
        self.to_group_id.as_deref()
    }

    pub fn to_account(&self) -> Option<&str> {
        self.to_account.as_deref()
    }

    pub fn standard_mark(&self) -> Option<&str> {
        self.standard_mark.as_deref()
    }

    pub fn custom_mark(&self) -> Option<&str> {
        self.custom_mark.as_deref()
    }

    pub fn contact_group_id(&self) -> Option<&Vec<u32>> {
        self.contact_group_id.as_ref()
    }

    pub fn timestamp(&self) -> Option<u64> {
        self.timestamp
    }
}
