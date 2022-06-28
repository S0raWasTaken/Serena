use std::time::Duration;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::MessageId},
    utils::parse_mention,
};
use tokio::time::sleep;

use crate::{
    ensure,
    primitives::{commands, ErrorBox, ToClapCommand},
    utils::{get_prefix, handle_result},
};

#[command]
#[only_in(guilds)]
#[aliases("cls", "rm")]
#[required_permissions("MANAGE_MESSAGES")]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(
        msg,
        &ctx.http,
        async move {
            let command = commands::clear();
            let guild_id = *msg.guild_id.unwrap().as_u64();
            let prefix = get_prefix(ctx.data.clone(), guild_id).await;

            let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

            let amount = matches.value_of("amount").unwrap().parse::<u64>()?;

            let mut messages = msg
                .channel_id
                .messages(&ctx.http, |m| {
                    if let Some(message) = matches.value_of("from_message") {
                        if let Ok(message_id) = get_message_id(message) {
                            if matches.is_present("after") {
                                m.after(message_id).limit(amount)
                            } else {
                                m.before(message_id).limit(amount)
                            }
                        } else {
                            m.limit(0)
                        }
                    } else if matches.is_present("after") {
                        m.after(msg.id).limit(amount)
                    } else {
                        m.before(msg.id).limit(amount)
                    }
                })
                .await?;

            if let Some(user) = matches.value_of("@mention/ID") {
                let mention = parse_mention(user).ok_or("Invalid mention or user ID")?;
                messages = messages
                    .iter()
                    .filter(|m| m.author.id == mention)
                    .cloned()
                    .collect();
            }

            ensure!(!messages.is_empty(), "error: No messages found to delete.");

            let messages_len = messages.len();

            while !messages.is_empty() {
                let taken = if messages.len() <= 100 {
                    messages.splice(.., []).collect::<Vec<_>>()
                } else {
                    messages.splice(..100, []).collect::<Vec<_>>()
                };

                msg.channel_id.delete_messages(&ctx.http, &taken).await?;
            }

            let temporary_message = msg
                .reply(&ctx.http, format!("Cleared {} messages.", messages_len))
                .await?;

            sleep(Duration::from_secs(3)).await;
            temporary_message.delete(&ctx.http).await?;

            Ok(())
        }
        .await,
    )
    .await
}

fn get_message_id(message: &str) -> ErrorBox<MessageId> {
    Ok(MessageId::try_from(message.parse::<u64>()?)?)
}
