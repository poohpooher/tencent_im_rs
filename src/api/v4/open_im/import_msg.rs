//! <https://www.tencentcloud.com/ko/document/product/1047/35014>

use crate::api::v4::common::{ActionStatus, ErrorCode, MsgBody};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportMsgRequest {
    #[serde(rename = "SyncFromOldSystem")]
    sync_from_old_system: Option<u32>,

    #[serde(rename = "From_Account")]
    from_account: String,

    #[serde(rename = "To_Account")]
    to_account: String,

    #[serde(rename = "MsgSeq")]
    msg_seq: Option<u32>,

    #[serde(rename = "MsgRandom")]
    msg_random: u32,

    #[serde(rename = "MsgTimeStamp")]
    msg_time_stamp: u32,

    #[serde(rename = "MsgBody")]
    msg_body: Vec<MsgBody>,

    #[serde(rename = "CloudCustomData")]
    cloud_custom_data: Option<String>,
}

impl ImportMsgRequest {
    pub fn new<S: AsRef<str>>(
        from_account: S,
        to_account: S,
        msg_random: u32,
        msg_time_stamp: u32,
        msg_body: Vec<MsgBody>,
    ) -> Self {
        Self {
            sync_from_old_system: None,
            from_account: from_account.as_ref().to_string(),
            to_account: to_account.as_ref().to_string(),
            msg_seq: None,
            msg_random,
            msg_time_stamp,
            msg_body,
            cloud_custom_data: None,
        }
    }

    pub fn set_sync_from_old_system(&mut self, sync_from_old_system: Option<u32>) -> &mut Self {
        self.sync_from_old_system = sync_from_old_system;
        self
    }

    pub fn set_msg_seq(&mut self, msg_seq: Option<u32>) -> &mut Self {
        self.msg_seq = msg_seq;
        self
    }

    pub fn set_cloud_custom_data<S: AsRef<str>>(
        &mut self,
        cloud_custom_data: Option<S>,
    ) -> &mut Self {
        self.cloud_custom_data = cloud_custom_data.map(|s| s.as_ref().to_string());
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportMsgResponse {
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
mod test_import_msg {
    use crate::api::v4::common::{MsgBody, MsgContent};
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({
          "SyncFromOldSystem": 5, // Imports real-time messages, marks them as unread and pushes them to the client.
          "From_Account": "lumotuwe1", // Account of the sender
          "To_Account": "lumotuwe2", // Account of the recipient
          "MsgSeq": 827092, // Sequence number of the message
          "MsgRandom": 1287657, // A random number assigned to the message
          "MsgTimeStamp": 1556178721, // UNIX timestamp in seconds
          "MsgBody": [ // Message body. This is a text message
              {
                  "MsgType": "TIMTextElem", // Text message element
                  "MsgContent": {
                      "Text": "hi, beauty"
                  }
              }
          ],
          "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::ImportMsgRequest::new(
            "lumotuwe1",
            "lumotuwe2",
            1287657,
            1556178721,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_sync_from_old_system(Some(5))
            .set_msg_seq(Some(827092))
            .set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!( {
          "SyncFromOldSystem": 2, // Imports historical messages, marks them as read and does not push them to the client.
          "From_Account": "lumotuwe1", // Account of the sender
          "To_Account": "lumotuwe2", // Account of the recipient
          "MsgSeq": 827092, // Sequence number of the message
          "MsgRandom": 1287657, // A random number assigned to the message
          "MsgTimeStamp": 1556178721, // UNIX timestamp in seconds
          "MsgBody": [ // Message body. This is a text message
              {
                  "MsgType": "TIMTextElem", // Text message element
                  "MsgContent": {
                      "Text": "hi, beauty" // Message content
                  }
              }
          ],
          "CloudCustomData": "your cloud custom data"
        });

        let mut req = super::ImportMsgRequest::new(
            "lumotuwe1",
            "lumotuwe2",
            1287657,
            1556178721,
            vec![MsgBody::new(
                MsgContent::str_text(),
                MsgContent::content_text("hi, beauty"),
            )],
        );

        req.set_sync_from_old_system(Some(2))
            .set_msg_seq(Some(827092))
            .set_cloud_custom_data(Some("your cloud custom data"));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
        "ActionStatus" : "OK",
        "ErrorInfo" : "",
        "ErrorCode" : 0
        }
        );

        let res = serde_json::from_value::<super::ImportMsgResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
