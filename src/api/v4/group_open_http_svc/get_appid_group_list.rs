//! <https://www.tencentcloud.com/document/product/1047/34960>

use crate::api::v4::common::{ActionStatus, ErrorCode};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::api::v4::common::group_type::GroupType;

use crate::tencent_api;

tencent_api!(
    get_appid_group_list,
    GetAppIdGroupListRequest,
    GetAppIdGroupListResponse
);

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAppIdGroupListRequest {
    #[serde(rename = "Limit")]
    /// 요청한 페이지의 최대 개수
    limit: Option<u32>,

    #[serde(rename = "Next")]
    /// 이전 요청에서 반환된 Next 필드
    next: Option<u32>,

    #[serde(rename = "GroupType")]
    /// 그룹 타입
    group_type: Option<GroupType>,
}

impl Default for GetAppIdGroupListRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl GetAppIdGroupListRequest {
    pub fn new() -> Self {
        Self {
            limit: None,
            next: None,
            group_type: None,
        }
    }

    pub fn set_limit(&mut self, limit: Option<u32>) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn set_next(&mut self, next: Option<u32>) -> &mut Self {
        self.next = next;
        self
    }

    pub fn set_group_type(&mut self, group_type: Option<GroupType>) -> &mut Self {
        self.group_type = group_type;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetAppIdGroupListResponse {
    #[serde(rename = "ActionStatus")]
    pub action_status: Option<ActionStatus>,

    #[serde(rename = "ErrorCode")]
    pub error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    pub error_info: Option<String>,

    #[serde(rename = "ErrorDisplay")]
    pub error_display: Option<String>,

    #[serde(rename = "TotalCount")]
    pub total_count: Option<u32>,

    #[serde(rename = "Next")]
    pub next: Option<u64>,

    #[serde(rename = "GroupIdList")]
    pub group_id_list: Option<Vec<GroupId>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupId {
    #[serde(rename = "GroupId")]
    pub group_id: String,
}

#[cfg(test)]
mod test_get_appid_group_list {
    use serde_json::json;

    #[test]
    fn request1() {
        let sample = json!({});

        let req = super::GetAppIdGroupListRequest::new();

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request2() {
        let sample = json!({
           "Limit": 1000,
           "Next": 0
        });

        let mut req = super::GetAppIdGroupListRequest::new();
        req.set_limit(Some(1000)).set_next(Some(0));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request3() {
        let sample = json!({
          "GroupType" : "Public" // Type of groups to be pulled. If this parameter is not specified, all types of groups will be pulled.
        });

        let mut req = super::GetAppIdGroupListRequest::new();

        req.set_group_type(Some(super::GroupType::Public));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn request4() {
        let sample = json!({
          "Limit": 1000,
          "Next": 0,
          "GroupType" : "Public" // Type of groups to be pulled. If this parameter is not specified, all types of groups will be pulled.
        });

        let mut req = super::GetAppIdGroupListRequest::new();
        req.set_limit(Some(1000))
            .set_next(Some(0))
            .set_group_type(Some(super::GroupType::Public));

        assert_eq!(serde_json::to_value(req).unwrap(), sample);
    }

    #[test]
    fn response1() {
        let sample = json!({
            "ActionStatus": "OK",
            "ErrorInfo": "",
            "ErrorCode": 0,
            "TotalCount": 2,
            "GroupIdList": [
                {
                    "GroupId": "@TGS#2J4SZEAEL"
                },
                {
                    "GroupId": "@TGS#2C5SZEAEF"
                }
            ],
            "Next": 4454685361u64
        });

        let res =
            serde_json::from_value::<super::GetAppIdGroupListResponse>(sample.clone()).unwrap();

        assert_eq!(serde_json::to_value(res).unwrap(), sample);
    }
}
