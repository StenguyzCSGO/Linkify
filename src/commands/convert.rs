use crate::platforms::handler::PlatformHandler;
use crate::utils::DevResponse;
use crate::{Context, Error};
use std::time::Instant;

#[poise::command(slash_command, prefix_command)]
pub async fn convert(
    ctx: Context<'_>,
    #[description = "Music URL to convert"] url: String,
) -> Result<(), Error> {
    if let Some(handler) = PlatformHandler::from_url(&url) {
        let track_info_start = Instant::now();
        match handler.get_track_info(&url).await {
            Ok(track_info) => {
                let track_info_duration = track_info_start.elapsed();
                
                let all_platforms = PlatformHandler::all_platforms();
                let search_start = Instant::now();
                
                let mut platform_links = Vec::new();
                for platform in &all_platforms {
                    if platform.is_coming_soon() {
                        platform_links.push((platform.name(), "Coming soon...".to_string()));
                    } else if platform.name() == track_info.original_platform {
                        platform_links.push((platform.name(), url.clone()));
                    } else {
                        if let Some(link) = platform.get_track_link(&track_info).await {
                            platform_links.push((platform.name(), link));
                        }
                    }
                }
                let search_duration = search_start.elapsed();
                
                let links_str = if platform_links.is_empty() {
                    "No links found for this music.".to_string()
                } else {
                    platform_links
                        .iter()
                        .map(|(name, link)| {
                            if link == "Coming soon..." {
                                format!("{}: {}", name, link)
                            } else {
                                format!("{}: <{}>", name, link)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                let base_message = format!(
                    "🎵 **{}** by **{}**{}\n\n{}",
                    track_info.title,
                    track_info.artist,
                    track_info
                        .album
                        .map(|a| format!(" ({})", a))
                        .unwrap_or_default(),
                    links_str
                );

                let response = DevResponse::new(base_message)
                    .add_timing("Fetch title", track_info_duration)
                    .add_timing("Fetch links", search_duration)
                    .build();

                ctx.say(response).await?;
            }
            Err(e) => {
                ctx.say(format!("❌ Error: {:?}", e)).await?;
            }
        }
    } else {
        ctx.say("❌ Unsupported platform or invalid URL").await?;
    }
    Ok(())
}