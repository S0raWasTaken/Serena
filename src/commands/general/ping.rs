use std::time::Instant;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::primitives::{commands, ToClapCommand};
use crate::utils::{get_prefix, handle_result};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(
        msg,
        &ctx.http,
        async move {
            let command = commands::ping();
            let prefix = get_prefix(ctx.data.clone(), *msg.guild_id.unwrap().as_u64()).await;
            command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

            let time = Instant::now();

            let mut new_message = msg.reply(&ctx.http, "Ping?").await?;

            new_message
                .edit(&ctx.http, |m| {
                    m.content(format!("Pong! Latency is {:.2?}", time.elapsed()))
                })
                .await?;
            Ok(())
        }
        .await,
    )
    .await
}
