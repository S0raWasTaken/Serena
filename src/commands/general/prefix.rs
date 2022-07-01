use super::*;

#[command]
#[only_in(guilds)]
#[required_permissions("MANAGE_GUILD")]
async fn prefix(ctx: &Context, msg: &Message) -> CommandResult {
    handle_result(msg, &ctx.http, async move {
        let command = commands::prefix();
        let guild_id = *msg.guild_id.unwrap().as_u64();
        let prefix = get_prefix(ctx.data.clone(), guild_id).await;

        let matches = command.try_get_matches_from(msg.content.to_clap_command(prefix.clone()))?;

        if matches.is_present("show") {
            msg.reply_ping(
                &ctx.http,
                format!(
                    "Current prefix: {}",
                    ctx.data
                        .read()
                        .await
                        .get::<Prefixes>()
                        .ok_or("error: Failed to get prefixes database")?
                        .get_data(true)?
                        .get(&guild_id)
                        .map_or(DEFAULT_PREFIX, String::as_str)
                )
                .to_code_block("yml"),
            )
            .await?;
            return Ok(());
        }

        let new_prefix = matches.value_of("prefix").unwrap();
        if prefix == new_prefix {
            Err("error: Same prefix supplied. Database was not modified".into())
        } else {
            let mut database = ctx.data.write().await;
            let data = database
                .get_mut::<Prefixes>()
                .ok_or("error: Failed to get prefixes database")?;

            let mut borrowed_data = data.get_data(true)?;
            borrowed_data.insert(guild_id, new_prefix.to_string());
            data.put_data(borrowed_data, true)?;

            msg.reply_ping(
                &ctx.http,
                format!("success: Guild prefix was changed to '{}'", new_prefix)
                    .to_code_block("yml"),
            )
            .await?;
            Ok(())
        }
    })
    .await
}
