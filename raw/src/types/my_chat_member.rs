use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MyChatMember {
    chat: Chat,
    from: User,
    date: Integer,
    old_chat_member: ChatMember,
    new_chat_member: ChatMember,
}
