use serde::Deserialize;

use super::EventType;

pub struct EventData {}

#[derive(Debug, Deserialize)]
pub struct EventMsg {
    from_user_id: i64,

    to_user_id: i64,

    msg_type: EventType,
}
