use crate::platforms::handler::PlatformHandler;
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn convert(
    ctx: Context<'_>,
    #[description = "Music URL to convert"] url: String,
) -> Result<(), Error> {
    if let Some(handler) = PlatformHandler::from_url(&url) {
        match handler.get_track_info(&url).await {
            Ok(track_info) => {
                ctx.say(format!(
                    "🎵 **{}** by **{}**{}",
                    track_info.title,
                    track_info.artist,
                    track_info
                        .album
                        .map(|a| format!(" ({})", a))
                        .unwrap_or_default()
                ))
                .await?;
            }
            Err(e) => {
                ctx.say(format!("❌ Error: {}", e)).await?;
            }
        }
    } else {
        ctx.say("❌ Unsupported platform or invalid URL").await?;
    }
    Ok(())
}
