#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports, clippy::unreadable_literal)]
use std::{collections::HashSet, env::var};

use dotenv::dotenv;
use groups::{GENERAL_GROUP, MODERATION_GROUP, UTIL_GROUP};
use handler::Handler;
use primitives::{ErrorBox, Prefixes, DEFAULT_PREFIX};
use rustbreak::FileDatabase;
use serenity::{framework::StandardFramework, http::Http, Client};
use utils::handle_result;

mod commands;
mod groups;
mod handler;
mod primitives;
mod utils;

#[tokio::main]
async fn main() -> ErrorBox<()> {
    dotenv()?;

    let token = var("DISCORD_TOKEN")?;
    let application_id = var("APPLICATION_ID")?.parse::<u64>()?;
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => exit!(1, "Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => exit!(2, "No app info:\n{:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .on_mention(Some(bot_id))
                .ignore_webhooks(true)
                .delimiters([", ", ",", " "])
                .allow_dm(false)
                .with_whitespace(true)
                .ignore_bots(true)
                .dynamic_prefix(|ctx, msg| {
                    Box::pin(async move {
                        Some(
                            ctx.data
                                .clone()
                                .read()
                                .await
                                .get::<Prefixes>()?
                                .borrow_data()
                                .ok()?
                                .get(msg.guild_id?.as_u64())
                                .map_or(DEFAULT_PREFIX, String::as_str)
                                .to_string(),
                        )
                    })
                })
        })
        .after(handle_result)
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP)
        .group(&UTIL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .application_id(application_id)
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<Prefixes>(FileDatabase::load_from_path_or_default(
            "./guild_prefixes.yml",
        )?);
    }

    client.start().await?;

    Ok(())
}
