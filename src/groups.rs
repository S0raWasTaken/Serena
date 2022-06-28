use serenity::framework::standard::macros::group;

use crate::commands::general::ping::PING_COMMAND;
use crate::commands::general::prefix::PREFIX_COMMAND;
use crate::commands::moderation::kick::KICK_COMMAND;
use crate::commands::util::clear::CLEAR_COMMAND;

#[group]
#[commands(ping, prefix, kick)]
struct General;

#[group]
#[commands(clear)]
struct Util;
