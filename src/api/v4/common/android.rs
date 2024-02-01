use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Android {
    #[serde(rename = "Sound")]
    sound: Option<String>,

    #[serde(rename = "OPPOChannelID")]
    oppo_channel_id: Option<String>,

    #[serde(rename = "XiaomiChannelID")]
    xiaomi_channel_id: Option<String>,

    #[serde(rename = "HuaweiChannelID")]
    huawei_channel_id: Option<String>,

    #[serde(rename = "GoogleChannelID")]
    google_channel_id: Option<String>,

    #[serde(rename = "VIVOClassification")]
    vivo_classification: Option<u32>,

    #[serde(rename = "HuaWeiImportance")]
    huawei_importance: Option<String>,

    #[serde(rename = "ExtAsHuaweiIntentParam")]
    ext_as_huawei_intent_param: Option<u32>,

    #[serde(rename = "HuaWeiCategory")]
    huawei_category: Option<String>,
}

impl Default for Android {
    fn default() -> Self {
        Self::new()
    }
}

impl Android {
    pub fn new() -> Self {
        Self {
            sound: None,
            oppo_channel_id: None,
            xiaomi_channel_id: None,
            huawei_channel_id: None,
            google_channel_id: None,
            vivo_classification: None,
            huawei_importance: None,
            ext_as_huawei_intent_param: None,
            huawei_category: None,
        }
    }

    pub fn set_sound<S: AsRef<str>>(&mut self, sound: Option<S>) -> &mut Self {
        self.sound = sound.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_oppo_channel_id<S: AsRef<str>>(&mut self, oppo_channel_id: Option<S>) -> &mut Self {
        self.oppo_channel_id = oppo_channel_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_xiaomi_channel_id<S: AsRef<str>>(
        &mut self,
        xiaomi_channel_id: Option<S>,
    ) -> &mut Self {
        self.xiaomi_channel_id = xiaomi_channel_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_huawei_channel_id<S: AsRef<str>>(
        &mut self,
        huawei_channel_id: Option<S>,
    ) -> &mut Self {
        self.huawei_channel_id = huawei_channel_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_google_channel_id<S: AsRef<str>>(
        &mut self,
        google_channel_id: Option<S>,
    ) -> &mut Self {
        self.google_channel_id = google_channel_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_vivo_classification(&mut self, vivo_classification: Option<u32>) -> &mut Self {
        self.vivo_classification = vivo_classification;
        self
    }

    pub fn set_huawei_importance<S: AsRef<str>>(
        &mut self,
        huawei_importance: Option<S>,
    ) -> &mut Self {
        self.huawei_importance = huawei_importance.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_ext_as_huawei_intent_param(
        &mut self,
        ext_as_huawei_intent_param: Option<u32>,
    ) -> &mut Self {
        self.ext_as_huawei_intent_param = ext_as_huawei_intent_param;
        self
    }

    pub fn set_huawei_category<S: AsRef<str>>(&mut self, huawei_category: Option<S>) -> &mut Self {
        self.huawei_category = huawei_category.map(|s| s.as_ref().to_string());
        self
    }
}
