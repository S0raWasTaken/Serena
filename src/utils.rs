use std::sync::Arc;

use serenity::{
    framework::standard::CommandResult,
    http::Http,
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

pub async fn handle_result(
    message: &Message,
    http: &Arc<Http>,
    res: CommandResult,
) -> CommandResult {
    match res {
        Ok(_) => Ok(()),
        Err(why) => {
            message
                .reply_ping(http, why.to_string().to_code_block("yml"))
                .await?;
            Err(why)
        }
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
