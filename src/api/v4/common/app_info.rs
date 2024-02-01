use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppInfo {
    #[serde(rename = "AppName")]
    pub app_name: Option<String>,

    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

    #[serde(rename = "Company")]
    pub company: Option<String>,

    #[serde(rename = "ActiveUserNum")]
    pub active_user_num: Option<String>,

    #[serde(rename = "RegistUserNumOneDay")]
    pub regist_user_num_one_day: Option<String>,

    #[serde(rename = "RegistUserNumTotal")]
    pub regist_user_num_total: Option<String>,

    #[serde(rename = "LoginTimes")]
    pub login_times: Option<String>,

    #[serde(rename = "LoginUserNum")]
    pub login_user_num: Option<String>,

    #[serde(rename = "UpMsgNum")]
    pub up_msg_num: Option<String>,

    #[serde(rename = "SendMsgUserNum")]
    pub send_msg_user_num: Option<String>,

    #[serde(rename = "APNSMsgNum")]
    pub apns_msg_num: Option<String>,

    #[serde(rename = "C2CUpMsgNum")]
    pub c2c_up_msg_num: Option<String>,

    #[serde(rename = "C2CDownMsgNum")]
    pub c2c_down_msg_num: Option<String>,

    #[serde(rename = "C2CSendMsgUserNum")]
    pub c2c_send_msg_user_num: Option<String>,

    #[serde(rename = "C2CAPNSMsgNum")]
    pub c2c_apns_msg_num: Option<String>,

    #[serde(rename = "MaxOnlineNum")]
    pub max_online_num: Option<String>,

    #[serde(rename = "DownMsgNum")]
    pub down_msg_num: Option<String>,

    #[serde(rename = "ChainIncrease")]
    pub chain_increase: Option<String>,

    #[serde(rename = "ChainDecrease")]
    pub chain_decrease: Option<String>,

    #[serde(rename = "GroupUpMsgNum")]
    pub group_up_msg_num: Option<String>,

    #[serde(rename = "GroupDownMsgNum")]
    pub group_down_msg_num: Option<String>,

    #[serde(rename = "GroupSendMsgUserNum")]
    pub group_send_msg_user_num: Option<String>,

    #[serde(rename = "GroupAPNSMsgNum")]
    pub group_apns_msg_num: Option<String>,

    #[serde(rename = "GroupSendMsgGroupNum")]
    pub group_send_msg_group_num: Option<String>,

    #[serde(rename = "GroupJoinGroupTimes")]
    pub group_join_group_times: Option<String>,

    #[serde(rename = "GroupQuitGroupTimes")]
    pub group_quit_group_times: Option<String>,

    #[serde(rename = "GroupNewGroupNum")]
    pub group_new_group_num: Option<String>,

    #[serde(rename = "GroupAllGroupNum")]
    pub group_all_group_num: Option<String>,

    #[serde(rename = "GroupDestroyGroupNum")]
    pub group_destroy_group_num: Option<String>,

    #[serde(rename = "CallBackReq")]
    pub callback_req: Option<String>,

    #[serde(rename = "CallBackRsp")]
    pub callback_rsp: Option<String>,

    #[serde(rename = "Date")]
    pub date: Option<String>,
}
