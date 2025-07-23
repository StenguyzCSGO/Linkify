mod commands;

use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let guild_id = env::var("GUILD_ID")
        .ok()
        .and_then(|id| id.parse::<u64>().ok())
        .map(serenity::GuildId::new);
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping()
            ],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                if let Some(guild_id) = guild_id {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id).await?;
                    println!("Development mode");
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    println!("Production mode");
                }
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    match client {
        Ok(mut client) => match client.start().await {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to start Serenity Client: {}", e),
        },
        Err(e) => eprintln!("Failed to create Serenity Client: {}", e),
    }
}