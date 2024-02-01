use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BlackListVerifyType {
    #[serde(rename = "BlackCheckResult_Type_Single")]
    BlackCheckResultTypeSingle,

    #[serde(rename = "BlackCheckResult_Type_Both")]
    BlackCheckResultTypeBoth,
}
