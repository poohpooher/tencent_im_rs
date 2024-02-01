use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OperateField {
    AppName,
    AppId,
    Company,
    ActiveUserNum,
    RegistUserNumOneDay,
    RegistUserNumTotal,
    LoginTimes,
    LoginUserNum,
    UpMsgNum,
    SendMsgUserNum,
    APNSMsgNum,
    C2CUpMsgNum,
    C2CDownMsgNum,
    C2CSendMsgUserNum,
    C2CAPNSMsgNum,
    MaxOnlineNum,
    DownMsgNum,
    ChainIncrease,
    ChainDecrease,
    GroupUpMsgNum,
    GroupDownMsgNum,
    GroupSendMsgUserNum,
    GroupAPNSMsgNum,
    GroupSendMsgGroupNum,
    GroupJoinGroupTimes,
    GroupQuitGroupTimes,
    GroupNewGroupNum,
    GroupAllGroupNum,
    GroupDestroyGroupNum,
    CallBackReq,
    CallBackRsp,
    Date,
}
