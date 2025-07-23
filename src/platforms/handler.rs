use crate::types::TrackInfo;

use super::spotify;

pub enum PlatformHandler {
    Spotify,
}

impl PlatformHandler {
    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains("spotify.com") && url.contains("/track/") {
            Some(Self::Spotify)
        } else {
            None
        }
    }

    pub async fn get_track_info(
        &self,
        url: &str,
    ) -> Result<TrackInfo, Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Self::Spotify => spotify::get_track_info(url).await,
        }
    }

    pub async fn get_track_link(&self, info: &TrackInfo) -> Option<String> {
        match self {
            Self::Spotify => spotify::get_track_link(info).await,
        }
    }
}
