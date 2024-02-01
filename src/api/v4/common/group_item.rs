use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupItem {
    #[serde(rename = "GroupName")]
    pub group_name: String,

    #[serde(rename = "GroupId")]
    pub group_id: u32,
}
