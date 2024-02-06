use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupItem {
    #[serde(rename = "GroupName")]
    group_name: String,

    #[serde(rename = "GroupId")]
    group_id: u32,
}

impl GroupItem {
    pub fn new<S: AsRef<str>>(group_name: S, group_id: u32) -> Self {
        Self {
            group_name: group_name.as_ref().to_string(),
            group_id,
        }
    }

    pub fn group_name(&self) -> &str {
        &self.group_name
    }

    pub fn group_id(&self) -> u32 {
        self.group_id
    }

    pub fn set_group_name<S: AsRef<str>>(&mut self, group_name: S) -> &mut Self {
        self.group_name = group_name.as_ref().to_string();
        self
    }

    pub fn set_group_id(&mut self, group_id: u32) -> &mut Self {
        self.group_id = group_id;
        self
    }
}