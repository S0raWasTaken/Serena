use crate::commands::{
    general::{ping::PING_COMMAND, prefix::PREFIX_COMMAND},
    util::clear::CLEAR_COMMAND,
    moderation::kick::KICK_COMMAND
};

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, prefix, kick)]
struct General;

#[group]
#[commands(clear)]
struct Util;
