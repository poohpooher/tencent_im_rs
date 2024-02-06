use crate::api::v4::common::{ApplyJoinOption, ErrorCode, GroupType, KeyValuePascal, Member};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "Type")]
    group_type: Option<GroupType>,

    #[serde(rename = "Name")]
    name: Option<String>,

    #[serde(rename = "Introduction")]
    introduction: Option<String>,

    #[serde(rename = "Notification")]
    notification: Option<String>,

    #[serde(rename = "FaceUrl")]
    face_url: Option<String>,

    #[serde(rename = "Owner_Account")]
    owner_account: Option<String>,

    #[serde(rename = "GroupId")]
    group_id: Option<String>,

    #[serde(rename = "MaxMemberCount")]
    max_member_count: Option<u32>,

    #[serde(rename = "ApplyJoinOption")]
    apply_join_option: Option<ApplyJoinOption>,

    #[serde(rename = "AppDefinedData")]
    app_defined_data: Option<Vec<KeyValuePascal>>,

    #[serde(rename = "MemberList")]
    member_list: Option<Vec<Member>>,

    #[serde(rename = "AppMemberDefinedData")]
    app_member_defined_data: Option<Vec<KeyValuePascal>>,

    #[serde(rename = "SupportTopic")]
    support_topic: Option<u32>,

    #[serde(rename = "Appid")]
    app_id: Option<u32>,

    #[serde(rename = "CreateTime")]
    create_time: Option<u64>,

    #[serde(rename = "LastInfoTime")]
    last_info_time: Option<u64>,

    #[serde(rename = "LastMsgTime")]
    last_msg_time: Option<u64>,

    #[serde(rename = "MaxMemberNum")]
    max_member_num: Option<u32>,

    #[serde(rename = "MemberNum")]
    member_num: Option<u32>,

    #[serde(rename = "MuteAllMember")]
    mute_all_member: Option<String>,

    #[serde(rename = "NextMsgSeq")]
    next_msg_seq: Option<u32>,

    #[serde(rename = "SelfInfo")]
    self_info: Option<Member>,

    #[serde(rename = "GrossTopicNextMsgSeq")]
    gross_topic_next_msg_seq: Option<u32>,

    #[serde(rename = "ErrorCode")]
    error_code: Option<ErrorCode>,

    #[serde(rename = "ErrorInfo")]
    error_info: Option<String>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            group_type: None,
            name: None,
            introduction: None,
            notification: None,
            face_url: None,
            owner_account: None,
            group_id: None,
            max_member_count: None,
            apply_join_option: None,
            app_defined_data: None,
            member_list: None,
            app_member_defined_data: None,
            support_topic: None,
            app_id: None,
            create_time: None,
            last_info_time: None,
            last_msg_time: None,
            max_member_num: None,
            member_num: None,
            mute_all_member: None,
            next_msg_seq: None,
            self_info: None,
            gross_topic_next_msg_seq: None,
            error_code: None,
            error_info: None,
        }
    }

    pub fn set_group_type(&mut self, group_type: Option<GroupType>) -> &mut Self {
        self.group_type = group_type;
        self
    }

    pub fn set_name<S: AsRef<str>>(&mut self, name: Option<S>) -> &mut Self {
        self.name = name.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_owner_account<S: AsRef<str>>(&mut self, owner_account: Option<S>) -> &mut Self {
        self.owner_account = owner_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_group_id<S: AsRef<str>>(&mut self, group_id: Option<S>) -> &mut Self {
        self.group_id = group_id.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_max_member_count(&mut self, max_member_count: Option<u32>) -> &mut Self {
        self.max_member_count = max_member_count;
        self
    }

    pub fn set_apply_join_option(
        &mut self,
        apply_join_option: Option<ApplyJoinOption>,
    ) -> &mut Self {
        self.apply_join_option = apply_join_option;
        self
    }

    pub fn set_app_defined_data(
        &mut self,
        app_defined_data: Option<Vec<KeyValuePascal>>,
    ) -> &mut Self {
        self.app_defined_data = app_defined_data;
        self
    }

    pub fn set_introduction<S: AsRef<str>>(&mut self, introduction: Option<S>) -> &mut Self {
        self.introduction = introduction.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_notification<S: AsRef<str>>(&mut self, notification: Option<S>) -> &mut Self {
        self.notification = notification.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_face_url<S: AsRef<str>>(&mut self, face_url: Option<S>) -> &mut Self {
        self.face_url = face_url.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_member_list(&mut self, member_list: Option<Vec<Member>>) -> &mut Self {
        self.member_list = member_list;
        self
    }

    pub fn set_app_member_defined_data(
        &mut self,
        app_member_defined_data: Option<Vec<KeyValuePascal>>,
    ) -> &mut Self {
        self.app_member_defined_data = app_member_defined_data;
        self
    }

    pub fn set_support_topic(&mut self, support_topic: Option<u32>) -> &mut Self {
        self.support_topic = support_topic;
        self
    }

    pub fn set_app_id(&mut self, app_id: Option<u32>) -> &mut Self {
        self.app_id = app_id;
        self
    }

    pub fn set_create_time(&mut self, create_time: Option<u64>) -> &mut Self {
        self.create_time = create_time;
        self
    }

    pub fn set_last_info_time(&mut self, last_info_time: Option<u64>) -> &mut Self {
        self.last_info_time = last_info_time;
        self
    }

    pub fn set_last_msg_time(&mut self, last_msg_time: Option<u64>) -> &mut Self {
        self.last_msg_time = last_msg_time;
        self
    }

    pub fn set_max_member_num(&mut self, max_member_num: Option<u32>) -> &mut Self {
        self.max_member_num = max_member_num;
        self
    }

    pub fn set_member_num(&mut self, member_num: Option<u32>) -> &mut Self {
        self.member_num = member_num;
        self
    }

    pub fn set_mute_all_member<S: AsRef<str>>(&mut self, mute_all_member: Option<S>) -> &mut Self {
        self.mute_all_member = mute_all_member.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_next_msg_seq(&mut self, next_msg_seq: Option<u32>) -> &mut Self {
        self.next_msg_seq = next_msg_seq;
        self
    }

    pub fn set_self_info(&mut self, self_info: Option<Member>) -> &mut Self {
        self.self_info = self_info;
        self
    }

    pub fn set_gross_topic_next_msg_seq(
        &mut self,
        gross_topic_next_msg_seq: Option<u32>,
    ) -> &mut Self {
        self.gross_topic_next_msg_seq = gross_topic_next_msg_seq;
        self
    }

    pub fn set_error_code(&mut self, error_code: Option<ErrorCode>) -> &mut Self {
        self.error_code = error_code;
        self
    }

    pub fn set_error_info<S: AsRef<str>>(&mut self, error_info: Option<S>) -> &mut Self {
        self.error_info = error_info.map(|s| s.as_ref().to_string());
        self
    }

    pub fn group_type(&self) -> Option<&GroupType> {
        self.group_type.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    pub fn introduction(&self) -> Option<&str> {
        self.introduction.as_ref().map(|s| s.as_str())
    }

    pub fn notification(&self) -> Option<&str> {
        self.notification.as_ref().map(|s| s.as_str())
    }

    pub fn face_url(&self) -> Option<&str> {
        self.face_url.as_ref().map(|s| s.as_str())
    }

    pub fn owner_account(&self) -> Option<&str> {
        self.owner_account.as_ref().map(|s| s.as_str())
    }

    pub fn group_id(&self) -> Option<&str> {
        self.group_id.as_ref().map(|s| s.as_str())
    }

    pub fn max_member_count(&self) -> Option<u32> {
        self.max_member_count
    }

    pub fn apply_join_option(&self) -> Option<&ApplyJoinOption> {
        self.apply_join_option.as_ref()
    }

    pub fn app_defined_data(&self) -> Option<&Vec<KeyValuePascal>> {
        self.app_defined_data.as_ref()
    }

    pub fn member_list(&self) -> Option<&Vec<Member>> {
        self.member_list.as_ref()
    }

    pub fn app_member_defined_data(&self) -> Option<&Vec<KeyValuePascal>> {
        self.app_member_defined_data.as_ref()
    }

    pub fn support_topic(&self) -> Option<u32> {
        self.support_topic
    }

    pub fn app_id(&self) -> Option<u32> {
        self.app_id
    }

    pub fn create_time(&self) -> Option<u64> {
        self.create_time
    }

    pub fn last_info_time(&self) -> Option<u64> {
        self.last_info_time
    }

    pub fn last_msg_time(&self) -> Option<u64> {
        self.last_msg_time
    }

    pub fn max_member_num(&self) -> Option<u32> {
        self.max_member_num
    }

    pub fn member_num(&self) -> Option<u32> {
        self.member_num
    }

    pub fn mute_all_member(&self) -> Option<&str> {
        self.mute_all_member.as_ref().map(|s| s.as_str())
    }

    pub fn next_msg_seq(&self) -> Option<u32> {
        self.next_msg_seq
    }

    pub fn self_info(&self) -> Option<&Member> {
        self.self_info.as_ref()
    }

    pub fn gross_topic_next_msg_seq(&self) -> Option<u32> {
        self.gross_topic_next_msg_seq
    }

    pub fn error_code(&self) -> Option<&ErrorCode> {
        self.error_code.as_ref()
    }

    pub fn error_info(&self) -> Option<&str> {
        self.error_info.as_ref().map(|s| s.as_str())
    }


}
