//! <https://www.tencentcloud.com/document/product/1047/40123>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use crate::tencent_api;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

tencent_api!(group_get, GroupGetRequest, GroupGetResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupGetRequest {
    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(
        rename = "NeedFriend",
        serialize_with = "need_friend_to_string",
        deserialize_with = "need_friend_from_string"
    )]
    need_friend: Option<bool>,

    #[serde(rename = "GroupName")]
    group_names: Option<Vec<String>>,
}

impl GroupGetRequest {
    pub fn new<S: AsRef<str>>(from_account: S) -> Self {
        Self {
            from_account: from_account.as_ref().to_string(),
            need_friend: None,
            group_names: None,
        }
    }

    pub fn set_need_friend(&mut self, need_friend: Option<bool>) -> &mut Self {
        self.need_friend = need_friend;
        self
    }

    pub fn set_group_names<S: AsRef<str>>(&mut self, group_names: Option<Vec<S>>) -> &mut Self {
        self.group_names = group_names.map(|v| v.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
}

fn need_friend_to_string<S: ::serde::ser::Serializer>(
    need_friend: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match need_friend {
        Some(true) => serializer.serialize_str("Need_Friend_Type_Yes"),
        _ => serializer.serialize_none(),
    }
}

fn need_friend_from_string<'de, D: ::serde::de::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Need_Friend_Type_Yes" => Ok(Some(true)),
        _ => Ok(None),
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupGetResponse {
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

    #[serde(rename = "CurrentSequence")]
    pub current_sequence: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultItem {
    #[serde(rename = "GroupName")]
    group_name: String,

    #[serde(rename = "FriendNumber")]
    friend_number: u64,

    #[serde(rename = "To_Account")]
    to_accounts: Option<Vec<String>>,
}

#[cfg(test)]
mod test_group_get {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!(
        {
            "From_Account":"id"
        });

        let req = super::GroupGetRequest::new("id");

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!(
        {

             "From_Account":"id",

             "NeedFriend":"Need_Friend_Type_Yes",

              "GroupName": [

             "group1"

             ]

        });

        let mut req = super::GroupGetRequest::new("id");
        req.set_need_friend(Some(true));
        req.set_group_names(Some(vec!["group1"]));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response() {
        let sample = json!(
        {

          "ResultItem": [

              {

                  "GroupName": "group1",

                  "FriendNumber": 1,

                  "To_Account": ["friend1"]

              }

          ],

          "CurrentSequence": 2,

          "ActionStatus": "OK",

          "ErrorCode": 0,

          "ErrorInfo": "",

          "ErrorDisplay": ""

        });

        let res = serde_json::from_value::<super::GroupGetResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
