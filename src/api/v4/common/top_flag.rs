use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[serde(untagged)]
#[repr(u8)]
pub enum TopFlag {
    General = 0,
    Pinned = 1,
}
