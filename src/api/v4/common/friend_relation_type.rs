use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FriendRelationType {
    #[serde(rename = "CheckResult_Type_NoRelation")]
    NoRelation,

    #[serde(rename = "CheckResult_Type_AWithB")]
    AWithB,

    #[serde(rename = "CheckResult_Type_BothWay")]
    BothWay,

    #[serde(rename = "CheckResult_Type_BWithA")]
    BWithA,
}
