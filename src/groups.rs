use crate::commands::general::{ping::PING_COMMAND, prefix::PREFIX_COMMAND};
use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, prefix)]
struct General;
