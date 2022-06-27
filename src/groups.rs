use crate::commands::general::{ping::PING_COMMAND, prefix::PREFIX_COMMAND};
use crate::commands::moderation::{kick::KICK_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, prefix, kick)]
struct General;
