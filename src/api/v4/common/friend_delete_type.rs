use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FriendDeleteType {
    #[serde(rename = "Delete_Type_Single")]
    /// 단방향 친구 삭제
    DeleteTypeSingle,
    #[serde(rename = "Delete_Type_Both")]
    /// 양방향 친구 삭제
    DeleteTypeBoth,
}
