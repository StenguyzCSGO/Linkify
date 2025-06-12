use serenity::prelude::*;
use serenity::utils::token::validate;
use serenity::async_trait;
use serenity::model::channel::Message;
use std::env;
use std::process;
use serenity::gateway::ShardManager;
use std::sync::Arc;
use serenity::prelude::TypeMapKey;

struct Handler;
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let data_read = ctx.data.read().await;
            let manager = data_read
                .get::<ShardManagerContainer>()
                .expect("ShardManager not found");

            let runners = manager.runners.lock().await;
            let runner = runners
                .get(&ctx.shard_id)
                .expect("No shard runner for current shard");

            let reply = match runner.latency {
                Some(duration) => format!("{} ms", duration.as_millis()),
                None => "Not yet known".to_string(),
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, reply).await {
                println!("Error sending message: {why:?}");
             }
        }
    }
}

#[tokio::main]
async fn main() {
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    if validate(&discord_token).is_err() {
        eprintln!("Invalid Discord token");
        process::exit(1);
    }

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT; 

    let mut client = Client::builder(&discord_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error when creating client");

    let shard_manager = client.shard_manager.clone();
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(shard_manager);
    }
        

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }    
}