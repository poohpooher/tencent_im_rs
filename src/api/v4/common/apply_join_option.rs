use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// 그룹 참가 방식
#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum ApplyJoinOption {
    FreeAccess,
    NeedPermission,
    DisableApply,
}
