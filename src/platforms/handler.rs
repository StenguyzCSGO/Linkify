use crate::Error;
use crate::types::TrackInfo;

use super::{deezer, spotify};

#[derive(PartialEq)]
pub enum PlatformHandler {
    Spotify,
    Deezer,
    YouTubeMusic,
}

impl PlatformHandler {
    /// Retourne toutes les plateformes dans un ordre alphabétique fixe
    pub fn all_platforms() -> Vec<Self> {
        vec![Self::Deezer, Self::Spotify, Self::YouTubeMusic]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Spotify => "Spotify",
            Self::Deezer => "Deezer",
            Self::YouTubeMusic => "YouTube Music",
        }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains("spotify.com") && url.contains("/track/") {
            return Some(Self::Spotify);
        }
        if url.contains("deezer.com/track/") {
            return Some(Self::Deezer);
        }
        None
    }

    pub fn is_coming_soon(&self) -> bool {
        matches!(self, Self::YouTubeMusic)
    }

    pub async fn get_track_info(&self, url: &str) -> Result<TrackInfo, Error> {
        match self {
            Self::Spotify => spotify::get_track_info(url).await,
            Self::Deezer => deezer::get_track_info(url).await,
            Self::YouTubeMusic => Err("YouTube Music is coming soon".into()),
        }
    }

    pub async fn get_track_link(&self, info: &TrackInfo) -> Option<String> {
        match self {
            Self::Spotify => spotify::get_track_link(info).await,
            Self::Deezer => deezer::get_track_link(info).await,
            Self::YouTubeMusic => None,
        }
    }
}