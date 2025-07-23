use base64::{Engine as _, engine::general_purpose};
use reqwest;
use serde::Deserialize;
use std::env;
use std::time::{Duration, Instant};

use crate::types::TrackInfo;

fn extract_track_id(url: &str) -> Option<String> {
    let track_part = url.split("/track/").nth(1)?;
    let track_id = track_part.split('?').next()?.split('/').next()?.to_string();

    if !track_id.is_empty() {
        Some(track_id)
    } else {
        None
    }
}

#[derive(Deserialize, Debug)]
struct SpotifyTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug)]
struct SpotifyToken {
    access_token: String,
    expires_in: u64,
    created_at: Instant,
}

impl SpotifyToken {
    fn is_valid(&self) -> bool {
        self.created_at.elapsed() < Duration::from_secs(self.expires_in)
    }
}

async fn get_access_token() -> Result<SpotifyToken, Box<dyn std::error::Error + Send + Sync>> {
    let client_id = env::var("SPOTIFY_CLIENT_ID")?;
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET")?;
    let auth = format!("{}:{}", client_id, client_secret);
    let b64_auth = general_purpose::STANDARD.encode(auth);

    let client = reqwest::Client::new();
    let res = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", format!("Basic {}", b64_auth))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("grant_type=client_credentials")
        .send()
        .await?;

    let token: SpotifyTokenResponse = res.json().await?;
    Ok(SpotifyToken {
        access_token: token.access_token,
        expires_in: token.expires_in,
        created_at: Instant::now(),
    })
}

pub async fn get_track_info(
    _url: &str,
) -> Result<TrackInfo, Box<dyn std::error::Error + Send + Sync>> {
    if !token.is_valid() {
        get_access_token();
    }
    let track_id = extract_track_id(_url).ok_or("Invalid Spotify track URL")?;
    let access_token = get_access_token().await?;
    let client = reqwest::Client::new();

    let url = format!("https://api.spotify.com/v1/tracks/{}", track_id);
    let response = client.get(&url).bearer_auth(access_token).send().await?;

    // TODO: Parse response into TrackInfo
    Ok(TrackInfo {
        title: "Imagine".to_string(),
        artist: "John Lennon".to_string(),
        album: Some("Imagine".to_string()),
    })
}

pub async fn get_track_link(_info: &TrackInfo) -> Option<String> {
    // You can generate a Spotify track link from the TrackInfo if you store the track ID.
    // For now, return None or a placeholder.
    None
}
