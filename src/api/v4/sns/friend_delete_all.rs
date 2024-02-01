//! <https://www.tencentcloud.com/document/product/1047/34906>

use crate::api::v4::common::{ActionStatus, ErrorCode, FriendDeleteType};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(
    friend_delete_all,
    FriendDeleteAllRequest,
    FriendDeleteAllResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendDeleteAllRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "DeleteType")]
    delete_type: Option<FriendDeleteType>,
}

impl FriendDeleteAllRequest {
    pub fn new(from_account: String) -> Self {
        Self {
            from_account,
            delete_type: None,
        }
    }

    pub fn set_delete_type(&mut self, delete_type: Option<FriendDeleteType>) {
        self.delete_type = delete_type;
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendDeleteAllResponse {
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
mod test_friend_delete_all {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"id"
        });

        let req = super::FriendDeleteAllRequest::new("id".to_string());

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"id",
          "DeleteType":"Delete_Type_Both"
        });

        let mut req = super::FriendDeleteAllRequest::new("id".to_string());
        req.set_delete_type(Some(super::FriendDeleteType::DeleteTypeBoth));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorCode": 0,
            "ErrorInfo": "",
            "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::FriendDeleteAllResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
