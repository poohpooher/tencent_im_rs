use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlackListRelationType {
    #[serde(rename = "BlackCheckResult_Type_BothWay")]
    BlackCheckResultTypeBothWay,

    #[serde(rename = "BlackCheckResult_Type_AWithB")]
    BlackCheckResultTypeAwithB,

    #[serde(rename = "BlackCheckResult_Type_BWithA")]
    BlackCheckResultTypeBwithA,

    #[serde(rename = "BlackCheckResult_Type_NO")]
    BlackCheckResultTypeNo,
}
