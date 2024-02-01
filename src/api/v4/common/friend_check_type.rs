use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FriendCheckType {
    #[serde(rename = "CheckResult_Type_Single")]
    /// 단방향 친구 확인
    CheckResultTypeSingle,

    #[serde(rename = "CheckResult_Type_Both")]
    /// 양방향 친구 확인
    CheckResultTypeBoth,
}
