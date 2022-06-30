use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::unban();
        let guild_id = msg.guild_id.unwrap();
        let prefix = get_prefix(ctx.data.clone(), *guild_id.as_u64()).await;

        let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

        let id = parse_mention(matches.value_of("ID").unwrap()).ok_or("Invalid mention/ID")?;

        guild_id.unban(&ctx.http, id).await?;

        msg.reply_ping(
            &ctx,
            format!(
                "{} was unbaned.",
                UserId::from(id).to_user(&ctx.http).await?.tag()
            ),
        )
        .await?;

        Ok(())
    })
    .await
}
