use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MemberRole {
    Admin,
    Owner,
    Member,
    NotMember,
}
