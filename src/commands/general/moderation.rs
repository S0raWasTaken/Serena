use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use serenity::utils::MessageBuilder;

#[command]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() > 0 {
        let kicked_user = &msg.mentions[0];

        if msg.author.id == kicked_user.id {
            msg.reply(&ctx, "You can not ban yourself.").await?;

        } else {
            let response = MessageBuilder::new()
            .push("The user ")
            .push(&kicked_user.name)
            .push(" has been kicked.")
            .build();

            msg.guild_id.unwrap().kick(&ctx.http, kicked_user.id).await?;
            msg.reply(ctx, response).await?;
            }
        } else {
            msg.reply(&ctx, "You should mention someone who you want to kick.").await?;
        }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.len() > 0 {
        let banned_user = &msg.mentions[0];

        if msg.author.id == banned_user.id {
            msg.reply(&ctx, "You can not ban yourself.").await?;

        } else {
            let response = MessageBuilder::new()
            .push("The user ")
            .push(&banned_user.name)
            .push(" has been banned.")
            .build();

            msg.guild_id.unwrap().ban(&ctx.http, banned_user.id, 0).await?;
            msg.reply(ctx, response).await?;
            }
        } else {
            msg.reply(&ctx, "You should mention someone who you want to kick.").await?;
        }
    Ok(())
}
