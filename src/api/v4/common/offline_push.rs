use crate::api::v4::common::{Android, Apns};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OfflinePush {
    #[serde(rename = "PushFlag")]
    push_flag: Option<i32>,

    #[serde(rename = "Title")]
    title: Option<String>,

    #[serde(rename = "Desc")]
    desc: Option<String>,

    #[serde(rename = "Ext")]
    ext: Option<String>,

    #[serde(rename = "AndroidInfo")]
    android_info: Option<Android>,

    #[serde(rename = "ApnsInfo")]
    apns_info: Option<Apns>,
}

impl Default for OfflinePush {
    fn default() -> Self {
        Self::new()
    }
}

impl OfflinePush {
    pub fn new() -> Self {
        Self {
            push_flag: None,
            title: None,
            desc: None,
            ext: None,
            android_info: None,
            apns_info: None,
        }
    }

    pub fn set_push_flag(&mut self, push_flag: Option<i32>) -> &mut Self {
        self.push_flag = push_flag;
        self
    }

    pub fn set_title<S: AsRef<str>>(&mut self, title: Option<S>) -> &mut Self {
        self.title = title.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_desc<S: AsRef<str>>(&mut self, desc: Option<S>) -> &mut Self {
        self.desc = desc.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_ext<S: AsRef<str>>(&mut self, ext: Option<S>) -> &mut Self {
        self.ext = ext.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_android_info(&mut self, android_info: Option<Android>) -> &mut Self {
        self.android_info = android_info;
        self
    }

    pub fn set_apns_info(&mut self, apns_info: Option<Apns>) -> &mut Self {
        self.apns_info = apns_info;
        self
    }
}
