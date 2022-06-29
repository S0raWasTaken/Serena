use serenity::framework::standard::macros::group;

use crate::commands::{
    general::{ping::PING_COMMAND, prefix::PREFIX_COMMAND},
    moderation::{ban::BAN_COMMAND, kick::KICK_COMMAND, unban::UNBAN_COMMAND},
    util::{clear::CLEAR_COMMAND},
};

#[group]
#[commands(ping, prefix)]
struct General;

#[group]
#[commands(clear)]
struct Util;

#[group]
#[commands(kick, ban, unban)]
struct Moderation;
