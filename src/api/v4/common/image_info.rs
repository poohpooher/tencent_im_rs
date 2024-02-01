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
}
