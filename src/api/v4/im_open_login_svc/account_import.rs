//! <https://www.tencentcloud.com/ko/document/product/1047/34953>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(account_import, AccountImportRequest, AccountImportResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountImportRequest {
    #[serde(rename = "UserID")]
    user_id: String,

    #[serde(rename = "Nick")]
    nick: Option<String>,

    #[serde(rename = "FaceUrl")]
    face_url: Option<String>,
}

impl AccountImportRequest {
    pub fn new<S: AsRef<str>>(user_id: S) -> Self {
        Self {
            user_id: user_id.as_ref().to_string(),
            nick: None,
            face_url: None,
        }
    }

    pub fn set_nick<S: AsRef<str>>(&mut self, nick: Option<S>) -> &mut Self {
        self.nick = nick.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_face_url<S: AsRef<str>>(&mut self, face_url: Option<S>) -> &mut Self {
        self.face_url = face_url.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountImportResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,
}

#[cfg(test)]
mod test_account_import {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
           "UserID":"test",
           "Nick":"test",
           "FaceUrl":"http://www.qq.com"
        });

        let mut req = super::AccountImportRequest::new("test");
        req.set_nick(Some("test"))
            .set_face_url(Some("http://www.qq.com"));

        assert_eq!(sample, serde_json::to_value(req).unwrap());
    }

    #[test]
    fn response2() {
        let sample = json!({
           "ActionStatus":"OK",
           "ErrorInfo":"",
           "ErrorCode":0
        });

        let res = serde_json::from_value::<super::AccountImportResponse>(sample.clone()).unwrap();

        assert_eq!(sample, serde_json::to_value(res).unwrap());
    }
}
