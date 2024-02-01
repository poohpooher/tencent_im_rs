use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SnsTag {
    TagSnsImGroup(Vec<String>),
    TagSnsImRemark(String),
    TagSnsImAddSource(String),
    TagSnsImAddWording(String),
    TagSnsImAddTime(u64),
    TagSnsCustom(String),
}

impl SnsTag {
    pub fn tag_group<S: AsRef<str>>(vec: Vec<S>) -> Self {
        Self::TagSnsImGroup(vec.iter().map(|s| s.as_ref().to_string()).collect())
    }

    pub fn tag_remark<S: AsRef<str>>(s: S) -> Self {
        Self::TagSnsImRemark(s.as_ref().to_string())
    }

    pub fn tag_add_source<S: AsRef<str>>(s: S) -> Self {
        Self::TagSnsImAddSource(s.as_ref().to_string())
    }

    pub fn tag_add_wording<S: AsRef<str>>(s: S) -> Self {
        Self::TagSnsImAddWording(s.as_ref().to_string())
    }

    pub fn tag_add_time(time: u64) -> Self {
        Self::TagSnsImAddTime(time)
    }

    pub fn tag_custom<S: AsRef<str>>(s: S) -> Self {
        Self::TagSnsCustom(s.as_ref().to_string())
    }

    pub fn str_remark() -> String {
        "Tag_SNS_IM_Remark".to_string()
    }

    pub fn str_add_source() -> String {
        "Tag_SNS_IM_AddSource".to_string()
    }

    pub fn str_add_wording() -> String {
        "Tag_SNS_IM_AddWording".to_string()
    }

    pub fn str_add_time() -> String {
        "Tag_SNS_IM_AddTime".to_string()
    }

    pub fn str_group() -> String {
        "Tag_SNS_IM_Group".to_string()
    }

    pub fn str_custom<S: AsRef<str>>(suffix: Option<S>) -> String {
        match suffix {
            Some(suffix) => format!("Tag_SNS_Custom_{}", suffix.as_ref()).to_string(),
            None => "Tag_SNS_Custom".to_string(),
        }
    }
}
