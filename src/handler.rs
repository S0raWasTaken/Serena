use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::prelude::Ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready to fight!", ready.user.name);
    }
}
