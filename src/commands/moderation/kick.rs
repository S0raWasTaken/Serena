use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::utils::parse_mention;

use crate::primitives::{commands, ToClapCommand};
use crate::utils::{get_prefix, handle_result};

#[command]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(
        msg,
        &ctx.http,
        async move {
            let command = commands::kick();
            let guild_id = msg.guild_id.unwrap();
            let prefix = get_prefix(ctx.data.clone(), *guild_id.as_u64()).await;

            let matches =
                command.try_get_matches_from(msg.content.to_clap_command(prefix.clone()))?;

            if let Some(member_mention) = matches.value_of("@Mention") {
                match parse_mention(member_mention) {
                    Some(mention) => {
                        guild_id.kick(&ctx.http, mention).await?;
                        msg.reply(&ctx, "The user has been kicked.").await?;
                    },
                    None => {
                        msg.reply(&ctx, "Something went wrong, try again later.").await?;
                    },
                }
            }

            Ok(())
        }
        .await,
    )
    .await
}
