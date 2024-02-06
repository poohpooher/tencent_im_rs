use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Apns {
    #[serde(rename = "BadgeMode")]
    badge_mode: Option<u32>,

    #[serde(rename = "Title")]
    title: Option<String>,

    #[serde(rename = "SubTitle")]
    sub_title: Option<String>,

    #[serde(rename = "Image")]
    image: Option<String>,

    #[serde(rename = "MutableContent")]
    mutable_content: Option<u32>,

    #[serde(rename = "Sound")]
    sound: Option<String>,

    #[serde(rename = "SubTitle")]
    subtitle: Option<String>,
}

impl Apns {
    pub fn new() -> Self {
        Self {
            badge_mode: None,
            title: None,
            sub_title: None,
            image: None,
            mutable_content: None,
            sound: None,
            subtitle: None,
        }
    }

    pub fn set_badge_mode(&mut self, badge_mode: Option<u32>) -> &mut Self {
        self.badge_mode = badge_mode;
        self
    }

    pub fn set_title<S: AsRef<str>>(&mut self, title: Option<S>) -> &mut Self {
        self.title = title.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_sub_title<S: AsRef<str>>(&mut self, sub_title: Option<S>) -> &mut Self {
        self.sub_title = sub_title.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_image<S: AsRef<str>>(&mut self, image: Option<S>) -> &mut Self {
        self.image = image.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_mutable_content(&mut self, mutable_content: Option<u32>) -> &mut Self {
        self.mutable_content = mutable_content;
        self
    }

    pub fn set_sound<S: AsRef<str>>(&mut self, sound: Option<S>) -> &mut Self {
        self.sound = sound.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_subtitle<S: AsRef<str>>(&mut self, subtitle: Option<S>) -> &mut Self {
        self.subtitle = subtitle.map(|s| s.as_ref().to_string());
        self
    }

    pub fn badge_mode(&self) -> Option<u32> {
        self.badge_mode
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn sub_title(&self) -> Option<&str> {
        self.sub_title.as_deref()
    }


    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    pub fn mutable_content(&self) -> Option<u32> {
        self.mutable_content
    }

    pub fn sound(&self) -> Option<&str> {
        self.sound.as_deref()
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }


}
