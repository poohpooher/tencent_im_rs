use log::error;
use rand::Rng;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod group_open_http_svc;
pub mod im_open_login_svc;

pub mod all_member_push;
pub mod common;
pub mod config_svc;
pub mod group_open_attr_http_svc;
pub mod open_config_svr;
pub mod open_im;
pub mod open_msg_svc;
pub mod profile;
pub mod recent_contact;
pub mod sns;

const VERSION: &str = "v4";

pub trait TencentRequest {}

// blanket trait implementation
impl<T> TencentRequest for T {}

pub trait TencentResponse {}

// blanket trait implementation
impl<T> TencentResponse for T {}

pub enum TencentDomain {
    China,
    Singapore,
    Seoul,
    Frankfurt,
    Mumbai,
    SiliconValley,
    Jakarta,
}

impl TencentDomain {
    pub fn domain_name(&self) -> String {
        match self {
            TencentDomain::China => "https://console.tim.qq.com",
            TencentDomain::Singapore => "https://sg.api.im.qcloud.com",
            TencentDomain::Seoul => "https://kr.api.im.qcloud.com",
            TencentDomain::Frankfurt => "https://de.api.im.qcloud.com",
            TencentDomain::Mumbai => "https://in.api.im.qcloud.com",
            TencentDomain::SiliconValley => "https://us.api.im.qcloud.com",
            TencentDomain::Jakarta => "https://id.api.im.qcloud.com",
        }
        .to_string()
    }
}

pub struct TencentHelper {
    client: reqwest::Client,
    domain: String,
    sdk_app_id: u32,
    identifier: String,
    user_sig: String,
}

impl TencentHelper {
    pub fn new(domain: String, sdk_app_id: u32, identifier: String, user_sig: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            domain,
            sdk_app_id,
            identifier,
            user_sig,
        }
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn get_sdk_app_id(&self) -> u32 {
        self.sdk_app_id
    }

    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_user_sig(&self) -> String {
        self.user_sig.clone()
    }

    pub fn get_client(&mut self) -> &mut reqwest::Client {
        &mut self.client
    }
}

pub fn make_url<S: AsRef<str>>(tencent_helper: &TencentHelper, request_api: S) -> String {
    format!(
        "https://{}/{}?sdkappid={}&identifier={}&usersig={}&random={}&contenttype=json",
        tencent_helper.get_domain(),
        request_api.as_ref(),
        tencent_helper.get_sdk_app_id(),
        tencent_helper.get_identifier(),
        tencent_helper.get_user_sig(),
        rand::thread_rng().gen::<u32>()
    )
}

pub async fn send<S, R>(
    tencent_helper: &mut TencentHelper,
    url: String,
    req: &S,
) -> Result<R, anyhow::Error>
where
    S: TencentRequest + Serialize + ?Sized,
    R: TencentResponse + DeserializeOwned,
{
    match tencent_helper
        .get_client()
        .post(url)
        .json(&req)
        .send()
        .await
    {
        Ok(response) => match response.json::<R>().await {
            Ok(response) => Ok(response),
            Err(e) => {
                error!("response.json error: {}", e);
                Err(anyhow::anyhow!("response.json error: {}", e))
            }
        },
        Err(e) => {
            error!("client.post() error: {}", e);
            Err(anyhow::anyhow!("client.post() error: {}", e))
        }
    }
}

#[macro_export]
macro_rules! tencent_api {
    ($api:ident, $req:ident, $res:ident) => {
        ::paste::paste! {
            #[allow(non_snake_case)]
            pub async fn [<api_ $api>](tencent_helper: &mut $crate::api::v4::TencentHelper, req: $req) -> Result<$res, anyhow::Error> {
                let url = $crate::api::v4::make_url(
                    tencent_helper,
                    format!(
                        "{version}/{category}/{api}",
                        version = super::super::VERSION,
                        category = super::MODULE_NAME,
                        api = stringify!($api)
                    ),
                );

                $crate::api::v4::send::<$req, $res>(tencent_helper, url, &req).await
            }
        }
    };
}

#[cfg(test)]
pub mod msg {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use serde_with::skip_serializing_none;

    use crate::api::v4::common::{
        ImageFormat, ImageInfo, ImageType, MsgBody, MsgContent, OfflinePush,
    };

    #[skip_serializing_none]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct TestOfflinePushInfo {
        #[serde(rename = "OfflinePushInfo")]
        offline_push_info: OfflinePush,
    }

    #[test]
    fn test_text() {
        let sample = json!({
            "MsgType": "TIMTextElem",
            "MsgContent": {
                "Text": "hello world"
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_location() {
        let sample = json!({
            "MsgType": "TIMLocationElem",
            "MsgContent": {
                "Desc": "someinfo",
                "Latitude": 29.340656774469956,
                "Longitude": 116.77497920478824
            }
        });

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_face() {
        let sample = json!({
            "MsgType": "TIMFaceElem",
            "MsgContent": {
                "Index": 1,
                "Data": "content"
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_custom() {
        let sample = json!({
            "MsgType": "TIMCustomElem",
            "MsgContent": {
                "Data": "message",
                "Desc": "notification",
                "Ext": "url",
                "Sound": "dingdong.aiff"
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_sound1() {
        let sample = json!({
            "MsgType": "TIMSoundElem",
            "MsgContent": {
                "Url": "https://1234-5678187359-1253735226.cos.ap-shanghai.myqcloud.com/abc123/c9be9d32c05bfb77b3edafa4312c6c7d",
                "UUID": "1053D4B3D61040894AC3DE44CDF28B3EC7EB7C0F",
                "Size": 62351,
                "Second": 1,
                "Download_Flag": 2
            }
        }

                );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_sound2() {
        let sample = json!({
            "MsgType": "TIMSoundElem",
            "MsgContent": {
                "UUID": "305c0201", //음성의 유일한 식별자로 유형은 String입니다. 클라이언트에서 음성 인덱스 시 키 값으로 사용합니다. 이 필드를 통해 관련 음성을 다운로드할 수 없습니다. 음성을 가져오려면, IM SDK 버전을 4.X로 업데이트하십시오.
                "Size": 62351,      //음성 데이터 크기. 유형: Number. 단위: 바이트.
                "Second": 1         //Number 유형 음성 길이. 단위: 초.
            }
        });

        let test = MsgBody::new(
            MsgContent::str_sound(),
            MsgContent::content_sound("305c0201", 62351, 1, None, None),
        );

        assert_eq!(serde_json::to_value(test).unwrap(), sample);

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_image() {
        let sample = json!({
            "MsgType": "TIMImageElem",
            "MsgContent": {
                "UUID": "1853095_D61040894AC3DE44CDFFFB3EC7EB720F",
                "ImageFormat": 1,
                "ImageInfoArray": [
                    {
                        "Type": 1,           //원본 이미지
                        "Size": 1853095,
                        "Width": 2448,
                        "Height": 3264,
                        "URL": "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/0"
                    },
                    {
                        "Type": 2,      //큰 이미지
                        "Size": 2565240,
                        "Width": 0,
                        "Height": 0,
                        "URL": "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/720"
                    },
                    {
                        "Type": 3,   //썸네일
                        "Size": 12535,
                        "Width": 0,
                        "Height": 0,
                        "URL": "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/198"
                    }
                ]
            }
        }
        );

        let test = MsgBody::new (
            MsgContent::str_image(),
            MsgContent::TIMImageElem {
                uuid: "1853095_D61040894AC3DE44CDFFFB3EC7EB720F".to_string(),
                image_format: ImageFormat::JPG,
                image_info_array: vec![
                    ImageInfo::new(
                        ImageType::Origin,
                        1853095,
                        2448,
                        3264,
                        "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/0",
                    ),
                    ImageInfo::new(
                        ImageType::Large,
                        2565240,
                        0,
                        0,
                        "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/720",
                    ),
                    ImageInfo::new(
                        ImageType::ThumbNail,
                        12535,
                        0,
                        0,
                        "http://xxx/3200490432214177468_144115198371610486_D61040894AC3DE44CDFFFB3EC7EB720F/198",
                    ),
                ],
            },
            );

        assert_eq!(serde_json::to_value(test).unwrap(), sample);

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_file1() {
        let sample = json!({
            "MsgType": "TIMFileElem",
            "MsgContent": {
                "Url": "https://7492-5678539059-1253735326.cos.ap-shanghai.myqcloud.com/abc123/49be9d32c0fbfba7b31dafa4312c6c7d",
                "UUID": "1053D4B3D61040894AC3DE44CDF28B3EC7EB7C0F",
                "FileSize": 1773552,
                "FileName": "file:///private/var/Application/tmp/trim.B75D5F9B-1426-4913-8845-90DD46797FCD.MOV",
                "Download_Flag": 2
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_file2() {
        let sample = json!({
            "MsgType": "TIMFileElem",
            "MsgContent": {
                "UUID": "305c02010", //파일의 유일한 식별자로 유형은 String입니다. 클라이언트에서 파일 인덱스에 사용하는 키 값이며 이 필드를 통해 파일을 다운로드할 수 없습니다. 파일을 가져오려면 IM SDK 버전을 4.X로 업데이트하십시오.
                "FileSize": 1773552, //Number 유형 파일 데이터 크기. 단위: 바이트.
                "FileName": "file:///private/var/Application/tmp/trim.B75D5F9B-1426-4913-8845-90DD46797FCD.MOV" //파일 이름. 유형: String.
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_video_file1() {
        let sample = json!({
            "MsgType": "TIMVideoFileElem",
            "MsgContent": {
                "VideoUrl": "https://0345-1400187352-1256635546.cos.ap-shanghai.myqcloud.com/abcd/f7c6ad3c50af7d83e23efe0a208b90c9",
                "VideoUUID": "5da38ba89d6521011e1f6f3fd6692e35",
                "VideoSize": 1194603,
                "VideoSecond": 5,
                "VideoFormat": "mp4",
                "VideoDownloadFlag":2,
                "ThumbUrl": "https://0345-1400187352-1256635546.cos.ap-shanghai.myqcloud.com/abcd/a6c170c9c599280cb06e0523d7a1f37b",
                "ThumbUUID": "6edaffedef5150684510cf97957b7bc8",
                "ThumbSize": 13907,
                "ThumbWidth": 720,
                "ThumbHeight": 1280,
                "ThumbFormat": "JPG",
                "ThumbDownloadFlag":2
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_video_file2() {
        let sample = json!({
            "MsgType": "TIMVideoFileElem",
            "MsgContent": {
                "VideoUUID": "1400123456_dramon_34ca36be7dd214dc50a49238ef80a6b5",// 영상의 유일한 식별자로 유형은 String입니다. 클라이언트에서 영상 인덱스에 사용하는 키 값으로 이 필드를 통해 영상을 다운로드할 수 없습니다. 영상을 가져오려면 IM SDK 버전을 4.X로 업데이트하십시오.
                "VideoSize": 1194603, //영상 데이터 크기. 유형: Number. 단위: 바이트.
                "VideoSecond": 5,     //영상 길이. 유형: Number. 단위: 초.
                "VideoFormat": "mp4", //비디오 형식. 유형: String. 예: mp4.
                "ThumbUUID": "1400123456_dramon_893f5a7a4872676ae142c08acd49c18a",// 비디오 썸네일의 유일한 식별자로 유형은 String입니다. 클라이언트에서 비디오 썸네일 인덱스에 사용하는 키 값으로 이 필드를 통해 비디오 썸네일을 다운로드할 수 없습니다. 비디오 썸네일을 가져오려면 IM SDK 버전을 4.X로 업데이트하십시오.
                "ThumbSize": 13907,   //썸네일 크기. 유형: Number. 단위: 바이트.
                "ThumbWidth": 720,    //썸네일 폭. 유형: Number.
                "ThumbHeight": 1280,  //썸네일 높이. 유형: Number.
                "ThumbFormat": "JPG"  //썸네일 형식. 유형: String. 예: JPG, BMP 등.
            }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_relay() {
        let sample = json!({
          "MsgType": "TIMRelayElem",
          "MsgContent": {
            "Title": "그룹 채팅 기록",
            "MsgNum": 2,
            "CompatibleText": "SDK 버전은 결합된 메시지를 지원하지 않습니다. 최신 버전으로 업그레이드하십시오.",
            "AbstractList": [
              "A: 이에 대해 어떻게 생각하세요?",
              "B: 좋은 것 같아요"
            ],
            "MsgList": [
              {
                "From_Account": "A",
                "GroupId": "group1",
                "MsgSeq": 85,
                "MsgRandom": 3998651049u32,
                "MsgTimeStamp": 1664437702,
                "MsgBody": [
                  {
                    "MsgContent": {
                      "Text": "이에 대해 어떻게 생각하세요?"
                    },
                    "MsgType": "TIMTextElem"
                  }
                ]
              },
              {
                "From_Account": "B",
                "GroupId": "group1",
                "MsgSeq": 86,
                "MsgRandom": 965790,
                "MsgTimeStamp": 1664437703,
                "MsgBody": [
                  {
                    "MsgContent": {
                      "Text": "좋은 것 같아요"
                    },
                    "MsgType": "TIMTextElem"
                  }
                ]
              }
            ]
          }
        }
        );

        let body = serde_json::from_value::<MsgBody>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(body).unwrap(), sample);
    }

    #[test]
    fn test_offline_push() {
        let sample = json!({
            "OfflinePushInfo": {
                "PushFlag": 0,
                "Title":"푸시 제목",
                "Desc": "오프라인 푸시 콘텐츠",
                "Ext": "패스스루 콘텐츠",
                "AndroidInfo": {
                    "Sound": "android.mp3",
                    "OPPOChannelID": "test_OPPO_channel_id",
                    "VIVOClassification": 1
                },
                "ApnsInfo": {
                    "Sound": "apns.mp3",
                    "BadgeMode": 1,
                    "Title":"apns title",
                    "SubTitle":"apns subtitle",
                    "Image":"www.image.com",
                    "MutableContent": 1
                }
            }
        }
        );

        let push = serde_json::from_value::<TestOfflinePushInfo>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(push).unwrap(), sample);
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use serde_with::skip_serializing_none;

    use crate::api::v4::common::MsgBody;

    #[skip_serializing_none]
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(rename = "MsgBody")]
        msg_body: Vec<MsgBody>,

        #[serde(rename = "CloudCustomData")]
        cloud_custom_data: Option<String>,
    }

    #[test]
    fn test_text_msg() {
        let sample = json!({
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hello world"
                    }
                }
            ]
        });

        let test = serde_json::from_value::<TestStruct>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(test).unwrap(), sample);
    }

    #[test]
    fn test_combine_msg() {
        let sample = json!({
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hello"
                    }
                },
                {
                    "MsgType": "TIMFaceElem",
                    "MsgContent": {
                        "Index": 1,
                        "Data": "content"
                    }
                },
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "world"
                    }
                }
            ]
        });

        let test = serde_json::from_value::<TestStruct>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(test).unwrap(), sample);
    }

    #[test]
    fn test_cloud_custom_date() {
        let sample = json!({
            "MsgBody": [
                {
                    "MsgType": "TIMTextElem",
                    "MsgContent": {
                        "Text": "hello"
                    }
                }
            ],
            "CloudCustomData": "your cloud custom data"
        }
        );

        let test = serde_json::from_value::<TestStruct>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(test).unwrap(), sample);
    }

    #[test]
    fn test_combine_msg_with_cloud_custom_data() {
        let sample = json!({
          "MsgBody": [
              {
                  "MsgType": "TIMTextElem",
                  "MsgContent": {
                      "Text": "hello"
                  }
              },
              {
                  "MsgType": "TIMCustomElem",
                  "MsgContent": {
                      "Data": "message",
                      "Desc": "world",
                      "Ext": "https://www.example.com",
                      "Sound": "dingdong.aiff"
                  }
              }
          ]
        }
        );

        let test = serde_json::from_value::<TestStruct>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(test).unwrap(), sample);
    }
}
