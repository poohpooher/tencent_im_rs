use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// 텐센트 채팅 그룹 타입
#[derive(Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum GroupType {
    Private,
    Public,
    ChatRoom,
    AVChatRoom,
    Community,
}
