include!("blawgd.rs");

pub const MSG_TYPE_CREATE_POST: &str = "/blawgd.MsgCreatePost";
pub const MSG_TYPE_FOLLOW: &str = "/blawgd.MsgFollow";
pub const MSG_TYPE_STOP_FOLLOW: &str = "/blawgd.MsgStopFollow";
pub const MSG_TYPE_LIKE: &str = "/blawgd.MsgLikePost";
pub const MSG_TYPE_REPOST: &str = "/blawgd.MsgRepost";
pub const MSG_TYPE_UPDATE_ACCOUNT_INFO: &str = "/blawgd.MsgUpdateAccountInfo";
pub const MSG_BANK_SEND: &str = "/cosmos.bank.v1beta1.MsgSend";
