use crate::api::v4::common::{ProfileTag, SnsTag};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SnsProfileTag {
    SnsTag(SnsTag),
    ProfileTag(ProfileTag),
}
