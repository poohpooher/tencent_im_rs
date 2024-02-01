use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FriendAddType {
    #[serde(rename = "Add_Type_Single")]
    /// 단방향 친구 추가
    AddTypeSingle = 1,
    #[serde(rename = "Add_Type_Both")]
    /// 양방향 친구 추가
    AddTypeBoth = 2,
}
