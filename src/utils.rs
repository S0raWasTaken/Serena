use std::sync::Arc;

use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandError},
    model::channel::Message,
    prelude::{RwLock, TypeMap},
};

use crate::primitives::{Prefixes, ToCodeBlock, DEFAULT_PREFIX};

pub async fn get_prefix(data: Arc<RwLock<TypeMap>>, guild_id: u64) -> String {
    data.read()
        .await
        .get::<Prefixes>()
        .unwrap()
        .get_data(true)
        .unwrap()
        .get(&guild_id)
        .unwrap_or(&DEFAULT_PREFIX.into())
        .clone()
}

#[hook]
pub async fn handle_result(
    ctx: &Context,
    message: &Message,
    _: &str,
    result: Result<(), CommandError>,
) {
    if let Err(why) = result {
        message
            .reply_ping(&ctx.http, why.to_string().to_code_block("yml"))
            .await
            .ok();
    }
}

#[macro_export]
macro_rules! exit {
    ($exit_code:expr, $($args:tt)*) => {{
        ::std::eprintln!($($args)*);
        ::std::process::exit($exit_code);
    }};
}

#[macro_export]
macro_rules! ensure {
    ($($x:expr, $y:expr),* $(,)?) => {
        $(
            if !$x {
                $crate::error!($y)
            }
        )*
    }
}

#[macro_export]
macro_rules! error {
    ($x:expr) => {{
        return Err($x.into());
    }};
}
