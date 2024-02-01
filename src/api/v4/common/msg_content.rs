use crate::api::v4::common::{ImageFormat, ImageInfo, RelayMsg};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MsgContent {
    TIMTextElem {
        #[serde(rename = "Text")]
        text: String,
    },
    TIMLocationElem {
        #[serde(rename = "Desc")]
        desc: String,

        #[serde(rename = "Latitude")]
        latitude: f64,

        #[serde(rename = "Longitude")]
        longitude: f64,
    },
    TIMFaceElem {
        #[serde(rename = "Index")]
        index: u32,

        #[serde(rename = "Data")]
        data: String,
    },
    TIMCustomElem {
        #[serde(rename = "Data")]
        data: String,

        #[serde(rename = "Desc")]
        desc: Option<String>,

        #[serde(rename = "Ext")]
        ext: Option<String>,

        #[serde(rename = "Sound")]
        sound: Option<String>,
    },
    TIMSoundElem {
        #[serde(rename = "Url")]
        url: Option<String>,

        #[serde(rename = "UUID")]
        uuid: String,

        #[serde(rename = "Size")]
        size: u32,

        #[serde(rename = "Second")]
        second: u32,

        #[serde(rename = "Download_Flag")]
        download_flag: Option<i32>,
    },
    TIMImageElem {
        #[serde(rename = "UUID")]
        uuid: String,

        #[serde(rename = "ImageFormat")]
        image_format: ImageFormat,

        #[serde(rename = "ImageInfoArray")]
        image_info_array: Vec<ImageInfo>,
    },
    TIMFileElem {
        #[serde(rename = "Url")]
        url: Option<String>,

        #[serde(rename = "UUID")]
        uuid: String,

        #[serde(rename = "FileSize")]
        file_size: u32,

        #[serde(rename = "FileName")]
        file_name: String,

        #[serde(rename = "Download_Flag")]
        download_flag: Option<i32>,
    },
    TIMVideoFileElem {
        #[serde(rename = "VideoUrl")]
        video_url: Option<String>,

        #[serde(rename = "VideoUUID")]
        video_uuid: String,

        #[serde(rename = "VideoSize")]
        video_size: u32,

        #[serde(rename = "VideoSecond")]
        video_second: u32,

        #[serde(rename = "VideoFormat")]
        video_format: String,

        #[serde(rename = "VideoDownloadFlag")]
        video_download_flag: Option<i32>,

        #[serde(rename = "ThumbUrl")]
        thumb_url: Option<String>,

        #[serde(rename = "ThumbUUID")]
        thumb_uuid: String,

        #[serde(rename = "ThumbSize")]
        thumb_size: u32,

        #[serde(rename = "ThumbWidth")]
        thumb_width: u32,

        #[serde(rename = "ThumbHeight")]
        thumb_height: u32,

        #[serde(rename = "ThumbFormat")]
        thumb_format: String,

        #[serde(rename = "ThumbDownloadFlag")]
        thumb_download_flag: Option<i32>,
    },
    TIMRelayElem {
        #[serde(rename = "Title")]
        title: String,

        #[serde(rename = "MsgNum")]
        msg_num: u32,

        #[serde(rename = "CompatibleText")]
        compatible_text: String,

        #[serde(rename = "AbstractList")]
        abstract_list: Vec<String>,

        #[serde(rename = "MsgList")]
        msg_list: Vec<RelayMsg>,
    },
}

impl MsgContent {
    pub fn str_text() -> String {
        "TIMTextElem".to_string()
    }

    pub fn str_location() -> String {
        "TIMLocationElem".to_string()
    }

    pub fn str_face() -> String {
        "TIMFaceElem".to_string()
    }

    pub fn str_custom() -> String {
        "TIMCustomElem".to_string()
    }

    pub fn str_sound() -> String {
        "TIMSoundElem".to_string()
    }

    pub fn str_image() -> String {
        "TIMImageElem".to_string()
    }

    pub fn str_file() -> String {
        "TIMFileElem".to_string()
    }

    pub fn str_video_file() -> String {
        "TIMVideoFileElem".to_string()
    }

    pub fn str_relay() -> String {
        "TIMRelayElem".to_string()
    }

    pub fn content_text<S: AsRef<str>>(text: S) -> Self {
        Self::TIMTextElem {
            text: text.as_ref().to_string(),
        }
    }

    pub fn content_location<S: AsRef<str>>(desc: S, latitude: f64, longitude: f64) -> Self {
        Self::TIMLocationElem {
            desc: desc.as_ref().to_string(),
            latitude,
            longitude,
        }
    }

    pub fn content_face(index: u32, data: &str) -> Self {
        Self::TIMFaceElem {
            index,
            data: data.to_string(),
        }
    }

    pub fn content_custom<S: AsRef<str>>(
        data: S,
        desc: Option<S>,
        ext: Option<S>,
        sound: Option<S>,
    ) -> Self {
        Self::TIMCustomElem {
            data: data.as_ref().to_string(),
            desc: desc.map(|s| s.as_ref().to_string()),
            ext: ext.map(|s| s.as_ref().to_string()),
            sound: sound.map(|s| s.as_ref().to_string()),
        }
    }

    pub fn content_sound<S: AsRef<str>>(
        uuid: S,
        size: u32,
        second: u32,
        download_flag: Option<i32>,
        url: Option<S>,
    ) -> Self {
        Self::TIMSoundElem {
            url: url.map(|s| s.as_ref().to_string()),
            uuid: uuid.as_ref().to_string(),
            size,
            second,
            download_flag,
        }
    }

    pub fn content_image<S: AsRef<str>>(
        uuid: &str,
        image_format: ImageFormat,
        image_info_array: Vec<ImageInfo>,
    ) -> Self {
        Self::TIMImageElem {
            uuid: uuid.to_string(),
            image_format,
            image_info_array,
        }
    }

    pub fn content_file<S: AsRef<str>>(
        url: Option<S>,
        uuid: &str,
        file_size: u32,
        file_name: &str,
        download_flag: Option<i32>,
    ) -> Self {
        Self::TIMFileElem {
            url: url.map(|s| s.as_ref().to_string()),
            uuid: uuid.to_string(),
            file_size,
            file_name: file_name.to_string(),
            download_flag,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn content_video_file<S: AsRef<str>>(
        video_url: Option<S>,
        video_uuid: &str,
        video_size: u32,
        video_second: u32,
        video_format: &str,
        video_download_flag: Option<i32>,
        thumb_url: Option<S>,
        thumb_uuid: &str,
        thumb_size: u32,
        thumb_width: u32,
        thumb_height: u32,
        thumb_format: &str,
        thumb_download_flag: Option<i32>,
    ) -> Self {
        Self::TIMVideoFileElem {
            video_url: video_url.map(|s| s.as_ref().to_string()),
            video_uuid: video_uuid.to_string(),
            video_size,
            video_second,
            video_format: video_format.to_string(),
            video_download_flag,
            thumb_url: thumb_url.map(|s| s.as_ref().to_string()),
            thumb_uuid: thumb_uuid.to_string(),
            thumb_size,
            thumb_width,
            thumb_height,
            thumb_format: thumb_format.to_string(),
            thumb_download_flag,
        }
    }

    pub fn content_relay<S: AsRef<str>>(
        title: &str,
        msg_num: u32,
        compatible_text: &str,
        abstract_list: Vec<S>,
        msg_list: Vec<RelayMsg>,
    ) -> Self {
        Self::TIMRelayElem {
            title: title.to_string(),
            msg_num,
            compatible_text: compatible_text.to_string(),
            abstract_list: abstract_list
                .into_iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
            msg_list,
        }
    }
}
