use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::ban();
        let guild_id = msg.guild_id.unwrap();
        let prefix = get_prefix(ctx.data.clone(), *guild_id.as_u64()).await;

        let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix))?;

        let mention =
            parse_mention(matches.value_of("@mention/ID").unwrap()).ok_or("Invalid mention/ID")?;

        let reason = if let Some(reason) = matches.values_of("reason") {
            format!(
                "Banned by: {}\nReason: {}",
                msg.author.tag(),
                reason.collect::<Vec<_>>().join(" ")
            )
        } else {
            format!("Banned by: {}", msg.author.tag())
        };

        let dmd = matches
            .value_of("delete-messages")
            .unwrap_or("0")
            .parse()
            .unwrap(); // Unwrapping is fine, since the value is already validated in commands::ban.

        guild_id
            .ban_with_reason(&ctx.http, mention, dmd, reason)
            .await?;

        msg.reply_ping(
            &ctx.http,
            format!(
                "{} was banned!",
                UserId::from(mention).to_user(&ctx.http).await?.tag()
            ),
        )
        .await?;
        Ok(())
    })
    .await
}
