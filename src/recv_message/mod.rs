#![allow(dead_code)]
#![allow(unused_variables)]

mod event;
mod event_type;
mod message_type;

pub use event_type::EventType;

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ReceivedData {
    PrivateMsg {
        #[serde(rename = "FromUin")]
        from_user_id: i64,

        #[serde(rename = "ToUin")]
        to_user_id: i64,

        #[serde(rename = "Content")]
        content: String,

        #[serde(rename = "MsgType")]
        msg_type: message_type::MsgType,

        #[serde(rename = "MsgSeq")]
        msg_seq: i64,

        #[serde(rename = "RedBaginfo")]
        red_bag_info: Option<String>,
    },
    GroupMsg {
        #[serde(rename = "FromGroupId")]
        from_group_id: i64,

        #[serde(rename = "FromGroupName")]
        from_group_name: String,

        #[serde(rename = "FromUserId")]
        from_user_id: i64,

        #[serde(rename = "FromNickName")]
        from_nick_name: String,

        #[serde(rename = "MsgTime")]
        msg_time: i64,

        #[serde(rename = "MsgRandom")]
        msg_random: i64,

        #[serde(rename = "RedBaginfo")]
        red_bag_info: Option<String>,

        #[serde(rename = "Content")]
        content: String,

        #[serde(rename = "MsgType")]
        msg_type: message_type::MsgType,

        #[serde(rename = "MsgSeq")]
        msg_seq: i64,
    },
	Event{
		event_msg: event::EventMsg,
	},
    Unknown {
        #[serde(flatten)]
        data: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize)]
struct ConnData {
    #[serde(rename = "WebConnId")]
    web_conn_id: String,

    #[serde(rename = "Data")]
    data: ReceivedData,
}

#[derive(Debug, Deserialize)]
pub struct ReceivedMessage {
    #[serde(rename = "CurrentQQ")]
    current_qq: i64,

    #[serde(rename = "CurrentPacket")]
    current_packet: ConnData,
}
