use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::unban();
    let guild_id = msg.guild_id.unwrap();
    let prefix = get_prefix(ctx.data.clone(), *guild_id.as_u64()).await;

    let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

    let mention = parse_mention(matches.value_of("ID").unwrap()).ok_or("Invalid mention/ID")?;

    guild_id.unban(&ctx.http, mention).await?;

    msg.reply_ping(
        &ctx,
        format!(
            "{} was unbanned.",
            if let Ok(user) = UserId::from(mention).to_user(&ctx.http).await {
                user.tag()
            } else {
                mention.to_string()
            }
        ),
    )
    .await?;

    Ok(())
}
