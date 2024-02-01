use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionStatus {
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "SomeError")]
    SomeError,
}
