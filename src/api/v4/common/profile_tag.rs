use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileTag {
    TagProfileImNick(String),
    TagProfileImGender(String),
    TagProfileImBirthDay(u32),
    TagProfileImLocation(String),
    TagProfileImSelfSignature(String),
    TagProfileImAllowType(String),
    TagProfileImLanguage(u32),
    TagProfileImImage(String),
    TagProfileImAdminForbidType(String),
    TagProfileImLevel(u32),
    TagProfileImRole(u32),
    TagProfileCustom(String),
}

impl ProfileTag {
    pub fn tag_nick<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImNick(s.as_ref().to_string())
    }

    pub fn tag_gender<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImGender(s.as_ref().to_string())
    }

    pub fn tag_birthday(birthday: u32) -> Self {
        Self::TagProfileImBirthDay(birthday)
    }

    pub fn tag_location<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImLocation(s.as_ref().to_string())
    }

    pub fn tag_self_signature<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImSelfSignature(s.as_ref().to_string())
    }

    pub fn tag_allow_type<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImAllowType(s.as_ref().to_string())
    }

    pub fn tag_language(language: u32) -> Self {
        Self::TagProfileImLanguage(language)
    }

    pub fn tag_image<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImImage(s.as_ref().to_string())
    }

    pub fn tag_admin_forbid_type<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileImAdminForbidType(s.as_ref().to_string())
    }

    pub fn tag_level(level: u32) -> Self {
        Self::TagProfileImLevel(level)
    }

    pub fn tag_role(role: u32) -> Self {
        Self::TagProfileImRole(role)
    }

    pub fn tag_custom<S: AsRef<str>>(s: S) -> Self {
        Self::TagProfileCustom(s.as_ref().to_string())
    }

    pub fn str_nick() -> String {
        "Tag_Profile_IM_Nick".to_string()
    }

    pub fn str_gender() -> String {
        "Tag_Profile_IM_Gender".to_string()
    }

    pub fn str_birthday() -> String {
        "Tag_Profile_IM_BirthDay".to_string()
    }

    pub fn str_location() -> String {
        "Tag_Profile_IM_Location".to_string()
    }

    pub fn str_self_signature() -> String {
        "Tag_Profile_IM_SelfSignature".to_string()
    }

    pub fn str_allow_type() -> String {
        "Tag_Profile_IM_AllowType".to_string()
    }

    pub fn str_language() -> String {
        "Tag_Profile_IM_Language".to_string()
    }

    pub fn str_image() -> String {
        "Tag_Profile_IM_Image".to_string()
    }

    pub fn str_admin_forbid_type() -> String {
        "Tag_Profile_IM_AdminForbidType".to_string()
    }

    pub fn str_level() -> String {
        "Tag_Profile_IM_Level".to_string()
    }

    pub fn str_role() -> String {
        "Tag_Profile_IM_Role".to_string()
    }

    pub fn str_custom<S: AsRef<str>>(suffix: Option<S>) -> String {
        match suffix {
            Some(suffix) => format!("Tag_Profile_Custom_{}", suffix.as_ref()).to_string(),
            None => "Tag_Profile_Custom".to_string(),
        }
    }
}
