use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum EventType {
    OnEventGroupJoin,
    OnEventGroupAdmin,
    OnEventGroupExit,
    OnEventGroupExitSuccess,
    OnEventGroupAdminSystemNotify,
    OnEventGroupRevoke,
    OnEventGroupShut,
    OnEventGroupSystemNotify,
    UnknownEventType(String),
}

impl From<String> for EventType {
    fn from(str: String) -> Self {
        match str {
            str if str == "ON_EVENT_GROUP_JOIN" => EventType::OnEventGroupJoin,
            str if str == "ON_EVENT_GROUP_ADMIN" => EventType::OnEventGroupAdmin,
            str if str == "ON_EVENT_GROUP_EXIT" => EventType::OnEventGroupExit,
            str if str == "ON_EVENT_GROUP_EXIT_SUCC" => EventType::OnEventGroupExitSuccess,
            str if str == "ON_EVENT_GROUP_ADMINSYSNOTIFY" => EventType::OnEventGroupAdminSystemNotify,
            str if str == "ON_EVENT_GROUP_REVOKE" => EventType::OnEventGroupRevoke,
            str if str == "ON_EVENT_GROUP_SHUT" => EventType::OnEventGroupShut,
            str if str == "ON_EVENT_GROUP_SYSTEMNOTIFY" => EventType::OnEventGroupSystemNotify,
            _ => EventType::UnknownEventType(str),
        }
    }
}
