use crate::Error;
use crate::types::TrackInfo;

use super::{deezer, spotify};

#[derive(PartialEq)]
pub enum PlatformHandler {
    Spotify,
    Deezer,
}

impl PlatformHandler {
    pub fn all_platforms() -> Vec<Self> {
        vec![Self::Spotify, Self::Deezer]
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Spotify" => Some(Self::Spotify),
            "Deezer" => Some(Self::Deezer),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Spotify => "Spotify",
            Self::Deezer => "Deezer",
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

    pub async fn get_track_info(&self, url: &str) -> Result<TrackInfo, Error> {
        match self {
            Self::Spotify => spotify::get_track_info(url).await,
            Self::Deezer => deezer::get_track_info(url).await,
        }
    }

    pub async fn get_track_link(&self, info: &TrackInfo) -> Option<String> {
        match self {
            Self::Spotify => spotify::get_track_link(info).await,
            Self::Deezer => deezer::get_track_link(info).await,
        }
    }
}
