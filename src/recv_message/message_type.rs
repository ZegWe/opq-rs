use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum MsgType {
    TextMsg,
    XmlMsg,
    PicMsg,
    AtMsg,
    BigFaceMsg,
    Unknown(String),
}

impl From<String> for MsgType {
    fn from(msg: String) -> Self {
        match msg {
            msg if msg == "TextMsg" => MsgType::TextMsg,
            msg if msg == "XmlMsg" => MsgType::XmlMsg,
            msg if msg == "PicMsg" => MsgType::PicMsg,
            msg if msg == "AtMsg" => MsgType::AtMsg,
            msg if msg == "BigFaceMsg" => MsgType::BigFaceMsg,
            _ => MsgType::Unknown(msg),
        }
    }
}

impl From<MsgType> for String {
    fn from(msg: MsgType) -> Self {
        match msg {
            MsgType::TextMsg => "TextMsg".to_string(),
            MsgType::XmlMsg => "XmlMsg".to_string(),
            MsgType::PicMsg => "PicMsg".to_string(),
            MsgType::AtMsg => "AtMsg".to_string(),
            MsgType::BigFaceMsg => "BigFaceMsg".to_string(),
            MsgType::Unknown(msg) => format!("UnknownMsgType({})", msg),
        }
    }
}
