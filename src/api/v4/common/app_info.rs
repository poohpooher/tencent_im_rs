use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppInfo {
    #[serde(rename = "AppName")]
    app_name: Option<String>,

    #[serde(rename = "AppId")]
    app_id: Option<String>,

    #[serde(rename = "Company")]
    company: Option<String>,

    #[serde(rename = "ActiveUserNum")]
    active_user_num: Option<String>,

    #[serde(rename = "RegistUserNumOneDay")]
    regist_user_num_one_day: Option<String>,

    #[serde(rename = "RegistUserNumTotal")]
    regist_user_num_total: Option<String>,

    #[serde(rename = "LoginTimes")]
    login_times: Option<String>,

    #[serde(rename = "LoginUserNum")]
    login_user_num: Option<String>,

    #[serde(rename = "UpMsgNum")]
    up_msg_num: Option<String>,

    #[serde(rename = "SendMsgUserNum")]
    send_msg_user_num: Option<String>,

    #[serde(rename = "APNSMsgNum")]
    apns_msg_num: Option<String>,

    #[serde(rename = "C2CUpMsgNum")]
    c2c_up_msg_num: Option<String>,

    #[serde(rename = "C2CDownMsgNum")]
    c2c_down_msg_num: Option<String>,

    #[serde(rename = "C2CSendMsgUserNum")]
    c2c_send_msg_user_num: Option<String>,

    #[serde(rename = "C2CAPNSMsgNum")]
    c2c_apns_msg_num: Option<String>,

    #[serde(rename = "MaxOnlineNum")]
    max_online_num: Option<String>,

    #[serde(rename = "DownMsgNum")]
    down_msg_num: Option<String>,

    #[serde(rename = "ChainIncrease")]
    chain_increase: Option<String>,

    #[serde(rename = "ChainDecrease")]
    chain_decrease: Option<String>,

    #[serde(rename = "GroupUpMsgNum")]
    group_up_msg_num: Option<String>,

    #[serde(rename = "GroupDownMsgNum")]
    group_down_msg_num: Option<String>,

    #[serde(rename = "GroupSendMsgUserNum")]
    group_send_msg_user_num: Option<String>,

    #[serde(rename = "GroupAPNSMsgNum")]
    group_apns_msg_num: Option<String>,

    #[serde(rename = "GroupSendMsgGroupNum")]
    group_send_msg_group_num: Option<String>,

    #[serde(rename = "GroupJoinGroupTimes")]
    group_join_group_times: Option<String>,

    #[serde(rename = "GroupQuitGroupTimes")]
    group_quit_group_times: Option<String>,

    #[serde(rename = "GroupNewGroupNum")]
    group_new_group_num: Option<String>,

    #[serde(rename = "GroupAllGroupNum")]
    group_all_group_num: Option<String>,

    #[serde(rename = "GroupDestroyGroupNum")]
    group_destroy_group_num: Option<String>,

    #[serde(rename = "CallBackReq")]
    callback_req: Option<String>,

    #[serde(rename = "CallBackRsp")]
    callback_rsp: Option<String>,

    #[serde(rename = "Date")]
    date: Option<String>,
}

impl AppInfo {
    pub fn new() -> Self {
        Self {
            app_name: None,
            app_id: None,
            company: None,
            active_user_num: None,
            regist_user_num_one_day: None,
            regist_user_num_total: None,
            login_times: None,
            login_user_num: None,
            up_msg_num: None,
            send_msg_user_num: None,
            apns_msg_num: None,
            c2c_up_msg_num: None,
            c2c_down_msg_num: None,
            c2c_send_msg_user_num: None,
            c2c_apns_msg_num: None,
            max_online_num: None,
            down_msg_num: None,
            chain_increase: None,
            chain_decrease: None,
            group_up_msg_num: None,
            group_down_msg_num: None,
            group_send_msg_user_num: None,
            group_apns_msg_num: None,
            group_send_msg_group_num: None,
            group_join_group_times: None,
            group_quit_group_times: None,
            group_new_group_num: None,
            group_all_group_num: None,
            group_destroy_group_num: None,
            callback_req: None,
            callback_rsp: None,
            date: None,
        }
    }

    pub fn set_app_name<S: AsRef<str>>(&mut self, app_name: Option<S>) -> &mut Self {
        self.app_name = app_name.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_app_id<S: AsRef<str>>(&mut self, app_id: Option<S>) -> &mut Self {
        self.app_id = app_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_company<S: AsRef<str>>(&mut self, company: Option<S>) -> &mut Self {
        self.company = company.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_active_user_num<S: AsRef<str>>(&mut self, active_user_num: Option<S>) -> &mut Self {
        self.active_user_num = active_user_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_regist_user_num_one_day<S: AsRef<str>>(
        &mut self,
        regist_user_num_one_day: Option<S>,
    ) -> &mut Self {
        self.regist_user_num_one_day = regist_user_num_one_day.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_regist_user_num_total<S: AsRef<str>>(
        &mut self,
        regist_user_num_total: Option<S>,
    ) -> &mut Self {
        self.regist_user_num_total = regist_user_num_total.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_login_times<S: AsRef<str>>(&mut self, login_times: Option<S>) -> &mut Self {
        self.login_times = login_times.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_login_user_num<S: AsRef<str>>(&mut self, login_user_num: Option<S>) -> &mut Self {
        self.login_user_num = login_user_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_up_msg_num<S: AsRef<str>>(&mut self, up_msg_num: Option<S>) -> &mut Self {
        self.up_msg_num = up_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_send_msg_user_num<S: AsRef<str>>(&mut self, send_msg_user_num: Option<S>) -> &mut Self {
        self.send_msg_user_num = send_msg_user_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_apns_msg_num<S: AsRef<str>>(&mut self, apns_msg_num: Option<S>) -> &mut Self {
        self.apns_msg_num = apns_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_c2c_up_msg_num<S: AsRef<str>>(&mut self, c2c_up_msg_num: Option<S>) -> &mut Self {
        self.c2c_up_msg_num = c2c_up_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_c2c_down_msg_num<S: AsRef<str>>(&mut self, c2c_down_msg_num: Option<S>) -> &mut Self {
        self.c2c_down_msg_num = c2c_down_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_c2c_send_msg_user_num<S: AsRef<str>>(
        &mut self,
        c2c_send_msg_user_num: Option<S>,
    ) -> &mut Self {
        self.c2c_send_msg_user_num = c2c_send_msg_user_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_c2c_apns_msg_num<S: AsRef<str>>(&mut self, c2c_apns_msg_num: Option<S>) -> &mut Self {
        self.c2c_apns_msg_num = c2c_apns_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_max_online_num<S: AsRef<str>>(&mut self, max_online_num: Option<S>) -> &mut Self {
        self.max_online_num = max_online_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_down_msg_num<S: AsRef<str>>(&mut self, down_msg_num: Option<S>) -> &mut Self {
        self.down_msg_num = down_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_chain_increase<S: AsRef<str>>(&mut self, chain_increase: Option<S>) -> &mut Self {
        self.chain_increase = chain_increase.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_chain_decrease<S: AsRef<str>>(&mut self, chain_decrease: Option<S>) -> &mut Self {
        self.chain_decrease = chain_decrease.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_up_msg_num<S: AsRef<str>>(&mut self, group_up_msg_num: Option<S>) -> &mut Self {
        self.group_up_msg_num = group_up_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_down_msg_num<S: AsRef<str>>(&mut self, group_down_msg_num: Option<S>) -> &mut Self {
        self.group_down_msg_num = group_down_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_send_msg_user_num<S: AsRef<str>>(
        &mut self,
        group_send_msg_user_num: Option<S>,
    ) -> &mut Self {
        self.group_send_msg_user_num = group_send_msg_user_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_apns_msg_num<S: AsRef<str>>(&mut self, group_apns_msg_num: Option<S>) -> &mut Self {
        self.group_apns_msg_num = group_apns_msg_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_send_msg_group_num<S: AsRef<str>>(
        &mut self,
        group_send_msg_group_num: Option<S>,
    ) -> &mut Self {
        self.group_send_msg_group_num = group_send_msg_group_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_join_group_times<S: AsRef<str>>(
        &mut self,
        group_join_group_times: Option<S>,
    ) -> &mut Self {
        self.group_join_group_times = group_join_group_times.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_quit_group_times<S: AsRef<str>>(
        &mut self,
        group_quit_group_times: Option<S>,
    ) -> &mut Self {
        self.group_quit_group_times = group_quit_group_times.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_new_group_num<S: AsRef<str>>(&mut self, group_new_group_num: Option<S>) -> &mut Self {
        self.group_new_group_num = group_new_group_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_all_group_num<S: AsRef<str>>(&mut self, group_all_group_num: Option<S>) -> &mut Self {
        self.group_all_group_num = group_all_group_num.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_destroy_group_num<S: AsRef<str>>(
        &mut self,
        group_destroy_group_num: Option<S>,
    ) -> &mut Self {
        self.group_destroy_group_num = group_destroy_group_num.map(|s| s.as_ref().to_string());
        self
    }

pub fn set_callback_req<S: AsRef<str>>(&mut self, callback_req: Option<S>) -> &mut Self {
        self.callback_req = callback_req.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_callback_rsp<S: AsRef<str>>(&mut self, callback_rsp: Option<S>) -> &mut Self {
        self.callback_rsp = callback_rsp.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_date<S: AsRef<str>>(&mut self, date: Option<S>) -> &mut Self {
        self.date = date.map(|s| s.as_ref().to_string());
        self
    }

    pub fn app_name(&self) -> Option<&str> {
        self.app_name.as_deref()
    }

    pub fn app_id(&self) -> Option<&str> {
        self.app_id.as_deref()
    }

    pub fn company(&self) -> Option<&str> {
        self.company.as_deref()
    }

    pub fn active_user_num(&self) -> Option<&str> {
        self.active_user_num.as_deref()
    }

    pub fn regist_user_num_one_day(&self) -> Option<&str> {
        self.regist_user_num_one_day.as_deref()
    }

    pub fn regist_user_num_total(&self) -> Option<&str> {
        self.regist_user_num_total.as_deref()
    }

    pub fn login_times(&self) -> Option<&str> {
        self.login_times.as_deref()
    }

    pub fn login_user_num(&self) -> Option<&str> {
        self.login_user_num.as_deref()
    }

    pub fn up_msg_num(&self) -> Option<&str> {
        self.up_msg_num.as_deref()
    }

    pub fn send_msg_user_num(&self) -> Option<&str> {
        self.send_msg_user_num.as_deref()
    }

    pub fn apns_msg_num(&self) -> Option<&str> {
        self.apns_msg_num.as_deref()
    }

    pub fn c2c_up_msg_num(&self) -> Option<&str> {
        self.c2c_up_msg_num.as_deref()
    }

    pub fn c2c_down_msg_num(&self) -> Option<&str> {
        self.c2c_down_msg_num.as_deref()
    }

    pub fn c2c_send_msg_user_num(&self) -> Option<&str> {
        self.c2c_send_msg_user_num.as_deref()
    }

    pub fn c2c_apns_msg_num(&self) -> Option<&str> {
        self.c2c_apns_msg_num.as_deref()
    }

    pub fn max_online_num(&self) -> Option<&str> {
        self.max_online_num.as_deref()
    }

    pub fn down_msg_num(&self) -> Option<&str> {
        self.down_msg_num.as_deref()
    }

    pub fn chain_increase(&self) -> Option<&str> {
        self.chain_increase.as_deref()
    }

    pub fn chain_decrease(&self) -> Option<&str> {
        self.chain_decrease.as_deref()
    }

    pub fn group_up_msg_num(&self) -> Option<&str> {
        self.group_up_msg_num.as_deref()
    }

    pub fn group_down_msg_num(&self) -> Option<&str> {
        self.group_down_msg_num.as_deref()
    }

    pub fn group_send_msg_user_num(&self) -> Option<&str> {
        self.group_send_msg_user_num.as_deref()
    }

    pub fn group_apns_msg_num(&self) -> Option<&str> {
        self.group_apns_msg_num.as_deref()
    }

    pub fn group_send_msg_group_num(&self) -> Option<&str> {
        self.group_send_msg_group_num.as_deref()
    }

    pub fn group_join_group_times(&self) -> Option<&str> {
        self.group_join_group_times.as_deref()
    }

    pub fn group_quit_group_times(&self) -> Option<&str> {
        self.group_quit_group_times.as_deref()
    }

    pub fn group_new_group_num(&self) -> Option<&str> {
        self.group_new_group_num.as_deref()
    }

    pub fn group_all_group_num(&self) -> Option<&str> {
        self.group_all_group_num.as_deref()
    }

    pub fn group_destroy_group_num(&self) -> Option<&str> {
        self.group_destroy_group_num.as_deref()
    }

    pub fn callback_req(&self) -> Option<&str> {
        self.callback_req.as_deref()
    }

    pub fn callback_rsp(&self) -> Option<&str> {
        self.callback_rsp.as_deref()
    }

    pub fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }
}