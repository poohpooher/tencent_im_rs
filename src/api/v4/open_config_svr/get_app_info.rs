//! <https://www.tencentcloud.com/document/product/1047/34886>

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::{ActionStatus, AppInfo, ErrorCode, OperateField};
use crate::tencent_api;

tencent_api!(getappinfo, GetAppInfoRequest, GetAppInfoResponse);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAppInfoRequest {
    #[serde(rename = "RequestField")]
    request_field: Option<Vec<OperateField>>,
}

impl Default for GetAppInfoRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl GetAppInfoRequest {
    pub fn new() -> Self {
        Self {
            request_field: None,
        }
    }

    pub fn set_request_field(&mut self, request_field: Vec<OperateField>) {
        self.request_field = Some(request_field);
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAppInfoResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "Result")]
    pub result: Option<Vec<AppInfo>>,
}

#[cfg(test)]
mod test_get_app_info {
    use crate::api::v4::common::OperateField;
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({});

        let req = super::GetAppInfoRequest::new();

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
          "RequestField":[
              "ChainIncrease",
              "ChainDecrease"
          ]
        });

        let mut req = super::GetAppInfoRequest::new();
        req.set_request_field(vec![
            OperateField::ChainIncrease,
            OperateField::ChainDecrease,
        ]);

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
          "ErrorCode": 0,
          "ErrorInfo": "OK",
          "Result": [{
              "APNSMsgNum": "84",
              "ActiveUserNum": "2014",
              "AppId": "1104620500",
              "AppName": "Real-Time Communication Scenario Developer edition",
              "C2CAPNSMsgNum": "84",
              "C2CDownMsgNum": "11040",
              "C2CSendMsgUserNum": "9",
              "C2CUpMsgNum": "52209",
              "CallBackReq": "73069",
              "CallBackRsp": "72902",
              "ChainDecrease": "16",
              "ChainIncrease": "18",
              "Company": "Linye",
              "Date": "20160607",
              "DownMsgNum": "11869",
              "GroupAPNSMsgNum": "0",
              "GroupAllGroupNum": "41913",
              "GroupDestroyGroupNum": "35019",
              "GroupDownMsgNum": "829",
              "GroupJoinGroupTimes": "121438",
              "GroupNewGroupNum": "35904",
              "GroupQuitGroupTimes": "108292",
              "GroupSendMsgGroupNum": "5189",
              "GroupSendMsgUserNum": "12",
              "GroupUpMsgNum": "8433",
              "LoginTimes": "13708",
              "LoginUserNum": "2094",
              "MaxOnlineNum": "62",
              "RegistUserNumOneDay": "1052",
              "RegistUserNumTotal": "53091",
              "SendMsgUserNum": "19",
              "UpMsgNum": "60642",
          }]
        });

        let res = serde_json::from_value::<super::GetAppInfoResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
