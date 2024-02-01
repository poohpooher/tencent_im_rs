use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MsgPriority {
    High,
    Normal,
    Low,
}
