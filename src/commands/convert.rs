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
                let mut final_links = Vec::new();
                for platform in PlatformHandler::all_platforms() {
                    if platform.name() != track_info.original_platform {
                        if let Some(link) = platform.get_track_link(&track_info).await {
                            final_links.push(link);
                        }
                    }
                }
                let links_str = if final_links.is_empty() {
                    "No links found on other platforms for this music.".to_string()
                } else {
                    PlatformHandler::all_platforms()
                        .into_iter()
                        .filter(|p| p.name() != track_info.original_platform)
                        .zip(final_links.iter())
                        .map(|(platform, link)| format!("{}: <{}>", platform.name(), link))
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                ctx.say(format!(
                    "🎵 **{}** by **{}**{}\nOriginal link:\n<{}>\n\nConverted links:\n{}",
                    track_info.title,
                    track_info.artist,
                    track_info
                        .album
                        .map(|a| format!(" ({})", a))
                        .unwrap_or_default(),
                    url,
                    links_str
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
