//! <https://www.tencentcloud.com/document/product/1047/34950>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::tencent_api;

tencent_api!(group_add, GroupAddRequest, GroupAddResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAddRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "GroupName")]
    group_names: Vec<String>,

    #[serde(rename = "To_Account")]
    to_accounts: Option<Vec<String>>,
}

impl GroupAddRequest {
    pub fn new<S: AsRef<str>>(from_account: S, group_names: Vec<S>) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            group_names: group_names.iter().map(|s| s.as_ref().to_string()).collect(),
            to_accounts: None,
        }
    }

    pub fn set_to_accounts<S: AsRef<str>>(&mut self, to_accounts: Option<Vec<S>>) -> &mut Self {
        self.to_accounts = to_accounts.map(|v| v.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAddResponse {
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

    #[serde(rename = "Fail_Account")]
    pub fail_accounts: Option<Vec<String>>,

    #[serde(rename = "CurrentSequence")]
    pub current_sequence: Option<u64>,
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
mod test_group_add {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "From_Account":"id",
          "GroupName":["group1","group2","group3"]
        });

        let req = super::GroupAddRequest::new("id", vec!["group1", "group2", "group3"]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "From_Account":"id",
          "GroupName":["group1","group2","group3"],
          "To_Account":["id1","id2","id3"]
        });

        let mut req = super::GroupAddRequest::new("id", vec!["group1", "group2", "group3"]);
        req.set_to_accounts(Some(vec!["id1", "id2", "id3"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "CurrentSequence": 2,
          "ActionStatus": "OK",
          "ErrorCode": 0,
          "ErrorInfo": "",
          "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::GroupAddResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }

    #[test]
    fn response2() {
        let sample = json!({
          "ResultItem":
          [
              {
                  "To_Account": "id1",
                  "ResultCode": 0,
                  "ResultInfo": ""
              },
              {
                  "To_Account": "id2",
                  "ResultCode": 32216,
                  "ResultInfo": "Err_SNS_GroupAdd_ToTinyId_Not_Friend"
              },
              {
                  "To_Account": "id3",
                  "ResultCode": 30002,
                  "ResultInfo": "ERR_SDKAPPID_ILLEGAL"
              }
          ],
          "Fail_Account":["id2","id3"],
          "CurrentSequence": 3,
          "ActionStatus": "OK",
          "ErrorCode": 0,
          "ErrorInfo": "",
          "ErrorDisplay": ""
        });

        let res = serde_json::from_value::<super::GroupAddResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
