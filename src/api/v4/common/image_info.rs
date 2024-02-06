use crate::api::v4::common::ImageType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    #[serde(rename = "Type")]
    image_type: ImageType,

    #[serde(rename = "Size")]
    size: u32,

    #[serde(rename = "Width")]
    width: u32,

    #[serde(rename = "Height")]
    height: u32,

    #[serde(rename = "URL")]
    url: String,
}

impl ImageInfo {
    pub fn new<S: AsRef<str>>(
        image_type: ImageType,
        size: u32,
        width: u32,
        height: u32,
        url: S,
    ) -> Self {
        Self {
            image_type,
            size,
            width,
            height,
            url: url.as_ref().to_string(),
        }
    }

    pub fn image_type(&self) -> &ImageType {
        &self.image_type
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_image_type(&mut self, image_type: ImageType) -> &mut Self {
        self.image_type = image_type;
        self
    }

    pub fn set_size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_url<S: AsRef<str>>(&mut self, url: S) -> &mut Self {
        self.url = url.as_ref().to_string();
        self
    }

}
