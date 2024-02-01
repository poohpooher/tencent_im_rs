use crate::api::v4::common::{KeyValuePascal, MemberRole, MsgFlag};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "Member_Account")]
    /// 멤버 아이디
    member_account: Option<String>,

    #[serde(rename = "Role")]
    /// 멤버 역할
    role: Option<MemberRole>,

    #[serde(rename = "AppMemberDefinedData")]
    /// 멤버 커스텀 필드
    app_member_defined_data: Option<Vec<KeyValuePascal>>,

    #[serde(rename = "JoinTime")]
    join_time: Option<u64>,

    #[serde(rename = "LastSendMsgTime")]
    last_send_msg_time: Option<u64>,

    #[serde(rename = "MsgFlag")]
    msg_flag: Option<MsgFlag>,

    #[serde(rename = "MsgSeq")]
    msg_seq: Option<u32>,

    #[serde(rename = "MuteUntil")]
    mute_until: Option<u64>,

    #[serde(rename = "NextMsgSeq")]
    next_msg_seq: Option<u32>,

    #[serde(rename = "NameCard")]
    name_card: Option<String>,

    #[serde(rename = "GrossTopicReadSeq")]
    gross_topic_read_seq: Option<u32>,

    #[serde(rename = "Result")]
    /// 결과용 필드, 0: 실패, 1: 성공, 2: 이미 그룹에 속해있음, 3: 승인 대기 중
    result: Option<u32>,
}

impl Default for Member {
    fn default() -> Self {
        Self::new()
    }
}

impl Member {
    pub fn new() -> Self {
        Self {
            member_account: None,
            role: None,
            app_member_defined_data: None,
            join_time: None,
            last_send_msg_time: None,
            msg_flag: None,
            msg_seq: None,
            mute_until: None,
            next_msg_seq: None,
            name_card: None,
            gross_topic_read_seq: None,
            result: None,
        }
    }

    pub fn set_member_account<S: AsRef<str>>(&mut self, member_account: Option<S>) -> &mut Self {
        self.member_account = member_account.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_role(&mut self, role: Option<MemberRole>) -> &mut Self {
        self.role = role;
        self
    }

    pub fn set_app_member_defined_data(
        &mut self,
        app_member_defined_data: Option<Vec<KeyValuePascal>>,
    ) -> &mut Self {
        self.app_member_defined_data = app_member_defined_data;
        self
    }

    pub fn set_join_time(&mut self, join_time: Option<u64>) -> &mut Self {
        self.join_time = join_time;
        self
    }

    pub fn set_last_send_msg_time(&mut self, last_send_msg_time: Option<u64>) -> &mut Self {
        self.last_send_msg_time = last_send_msg_time;
        self
    }

    pub fn set_msg_flag(&mut self, msg_flag: Option<MsgFlag>) -> &mut Self {
        self.msg_flag = msg_flag;
        self
    }

    pub fn set_msg_seq(&mut self, msg_seq: Option<u32>) -> &mut Self {
        self.msg_seq = msg_seq;
        self
    }

    pub fn set_mute_until(&mut self, mute_until: Option<u64>) -> &mut Self {
        self.mute_until = mute_until;
        self
    }

    pub fn set_next_msg_seq(&mut self, next_msg_seq: Option<u32>) -> &mut Self {
        self.next_msg_seq = next_msg_seq;
        self
    }

    pub fn set_name_card<S: AsRef<str>>(&mut self, name_card: Option<S>) -> &mut Self {
        self.name_card = name_card.map(|s| s.as_ref().to_string());
        self
    }

    pub fn set_gross_topic_read_seq(&mut self, gross_topic_read_seq: Option<u32>) -> &mut Self {
        self.gross_topic_read_seq = gross_topic_read_seq;
        self
    }

    pub fn set_result(&mut self, result: Option<u32>) -> &mut Self {
        self.result = result;
        self
    }
}
