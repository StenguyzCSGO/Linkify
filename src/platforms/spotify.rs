use base64::{Engine as _, engine::general_purpose};
use once_cell::sync::Lazy;
use reqwest;
use serde::Deserialize;
use std::env;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::Error;
use crate::types::TrackInfo;

static SPOTIFY_TOKEN: Lazy<Mutex<Option<SpotifyToken>>> = Lazy::new(|| Mutex::new(None));

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

#[derive(Deserialize, Debug)]
struct SpotifyTrack {
    name: String,
    artists: Vec<SpotifyArtists>,
    album: SpotifyAlbum,
}

#[derive(Deserialize, Debug)]
struct SpotifyArtists {
    name: String,
}

#[derive(Deserialize, Debug)]
struct SpotifyAlbum {
    name: String,
}

#[derive(Deserialize, Debug)]
struct SpotifySearchTrack {
    external_urls: SpotifyExternalUrls,
}

#[derive(Deserialize, Debug)]
struct SpotifyExternalUrls {
    spotify: String,
}

#[derive(Deserialize, Debug)]
struct SpotifySearchTracks {
    items: Vec<SpotifySearchTrack>,
}

#[derive(Deserialize, Debug)]
struct SpotifySearchResponse {
    tracks: SpotifySearchTracks,
}

fn extract_track_id(url: &str) -> Option<String> {
    let track_part = url.split("/track/").nth(1)?;
    let track_id = track_part.split('?').next()?.split('/').next()?.to_string();

    if !track_id.is_empty() {
        Some(track_id)
    } else {
        None
    }
}

impl SpotifyToken {
    fn is_valid(&self) -> bool {
        self.created_at.elapsed() < Duration::from_secs(self.expires_in)
    }
}

async fn get_access_token() -> Result<SpotifyToken, Error> {
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
        .await
        .map_err(|e| format!("Spotify token request failed: {}", e))?;

    let token: SpotifyTokenResponse = res.json().await?;
    Ok(SpotifyToken {
        access_token: token.access_token,
        expires_in: token.expires_in,
        created_at: Instant::now(),
    })
}

async fn get_access_token_cached() -> Result<String, Error> {
    {
        let token_guard = SPOTIFY_TOKEN.lock().unwrap();
        if let Some(token) = token_guard.as_ref() {
            if token.is_valid() {
                return Ok(token.access_token.clone());
            }
        }
    }
    let new_token = get_access_token().await?;
    let access_token = new_token.access_token.clone();
    let mut token_guard = SPOTIFY_TOKEN.lock().unwrap();
    *token_guard = Some(new_token);
    Ok(access_token)
}

pub async fn get_track_info(_url: &str) -> Result<TrackInfo, Error> {
    let track_id = extract_track_id(_url).ok_or("Invalid Spotify track URL")?;
    let access_token = get_access_token_cached().await?;
    let client = reqwest::Client::new();

    let url = format!("https://api.spotify.com/v1/tracks/{}", track_id);
    let res = client.get(&url).bearer_auth(&access_token).send().await?;

    let track: SpotifyTrack = res.json().await?;
    let artists = track
        .artists
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(TrackInfo {
        original_platform: "Spotify".to_string(),
        title: track.name,
        artist: artists,
        album: Some(track.album.name),
    })
}

pub async fn get_track_link(info: &TrackInfo) -> Option<String> {
    let access_token = get_access_token_cached().await.ok()?;

    let url = format!(
        "https://api.spotify.com/v1/search?q=track:\"{}\" artist:\"{}\"&type=track&limit=1",
        info.title, info.artist
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .bearer_auth(&access_token)
        .send()
        .await
        .ok()?;

    let search_result: SpotifySearchResponse = res.json().await.ok()?;

    search_result
        .tracks
        .items
        .first()
        .map(|track| track.external_urls.spotify.clone())
}
