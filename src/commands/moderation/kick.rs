use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    let command = commands::kick();
    let guild_id = msg.guild_id.unwrap();
    let prefix = get_prefix(ctx.data.clone(), *guild_id.as_u64()).await;

    let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix.clone()))?;

    let mention =
        parse_mention(matches.value_of("@mention/ID").unwrap()).ok_or("Invalid mention/ID")?;

    let reason = if let Some(reason) = matches.values_of("reason") {
        format!(
            "Kicked by: {}\nReason: {}",
            msg.author.tag(),
            reason.collect::<Vec<&str>>().join(" ")
        )
    } else {
        format!("Kicked by: {}", msg.author.tag())
    };

    guild_id
        .kick_with_reason(&ctx.http, mention, &reason)
        .await?;

    msg.reply_ping(
        &ctx.http,
        format!(
            "{} was kicked!",
            UserId::from(mention).to_user(&ctx.http).await?.tag()
        ),
    )
    .await?;

    Ok(())
}
