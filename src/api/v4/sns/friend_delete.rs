//! <https://www.tencentcloud.com/ko/document/product/1047/34905>

use crate::api::v4::common::{ActionStatus, ErrorCode, FriendDeleteType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(friend_delete, FriendDeleteRequest, FriendDeleteResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendDeleteRequest {
    #[serde(rename = "From_Account")]
    /// 친구 삭제 요청을 보낸 아이디
    from_account: String,

    #[serde(rename = "To_Account")]
    /// 친구 삭제 대상 아이디
    to_account: Vec<String>,

    #[serde(rename = "DeleteType")]
    /// 친구 삭제 타입
    delete_type: Option<FriendDeleteType>,
}

impl FriendDeleteRequest {
    pub fn new<S: AsRef<str>>(from_account: S, to_account: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            to_account: to_account.iter().map(|s| s.as_ref().to_string()).collect(),
            delete_type: None,
        }
    }

    pub fn set_delete_type(&mut self, delete_type: Option<FriendDeleteType>) -> &mut Self {
        self.delete_type = delete_type;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendDeleteResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "ResultItem")]
    pub result_items: Option<Vec<ResultItem>>,
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "ResultCode")]
    result_code: u32,

    #[serde(rename = "ResultInfo")]
    result_info: String,
}

#[cfg(test)]
mod test_friend_delete {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"id",
          "To_Account":["id1","id2","id3"],
          "DeleteType":"Delete_Type_Single"
        });

        let mut req = super::FriendDeleteRequest::new("id", vec!["id1", "id2", "id3"]);
        req.set_delete_type(Some(super::FriendDeleteType::DeleteTypeSingle));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"id",
          "To_Account":["id1","id2","id3"],
          "DeleteType":"Delete_Type_Single"
        });

        let mut req = super::FriendDeleteRequest::new("id", vec!["id1", "id2", "id3"]);
        req.set_delete_type(Some(super::FriendDeleteType::DeleteTypeSingle));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ResultItem":
            [
                {
                    "To_Account":"id1",
                    "ResultCode":0,
                    "ResultInfo":""
                },
                {
                    "To_Account":"id2",
                    "ResultCode":0,
                    "ResultInfo":""
                },
                {
                    "To_Account":"id3",
                    "ResultCode":0,
                    "ResultInfo":""
                }
            ],
            "ActionStatus":"OK",
            "ErrorCode":0,
            "ErrorInfo":"0",
            "ErrorDisplay":""
        });

        let res = serde_json::from_value::<super::FriendDeleteResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
