use reqwest;
use serde::Deserialize;

use crate::Error;
use crate::types::TrackInfo;

#[derive(Deserialize, Debug)]
struct DeezerSearchResponse {
    data: Vec<DeezerTrack>,
}

#[derive(Deserialize, Debug)]
struct DeezerTrack {
    title: String,
    link: String,
    artist: DeezerArtist,
    album: DeezerAlbum,
}

#[derive(Deserialize, Debug)]
struct DeezerArtist {
    name: String,
}

#[derive(Deserialize, Debug)]
struct DeezerAlbum {
    title: String,
}

fn extract_track_id(url: &str) -> Option<String> {
    if let Some(track_part) = url.split("/track/").nth(1) {
        let track_id = track_part.split('?').next()?.split('/').next()?.to_string();
        if !track_id.is_empty() {
            return Some(track_id);
        }
    }
    None
}

pub async fn get_track_info(url: &str) -> Result<TrackInfo, Error> {
    let track_id = extract_track_id(url).ok_or("Invalid Deezer track URL")?;
    let client = reqwest::Client::new();

    let api_url = format!("https://api.deezer.com/track/{}", track_id);
    let res = client.get(&api_url).send().await?;

    let track: DeezerTrack = res.json().await?;

    Ok(TrackInfo {
        original_platform: "Deezer".to_string(),
        title: track.title,
        artist: track.artist.name,
        album: Some(track.album.title),
    })
}

pub async fn get_track_link(info: &TrackInfo) -> Option<String> {
    let url = format!(
        "https://api.deezer.com/search?q=artist:\"{}\" track:\"{}\"",
        info.artist, info.title
    );

    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.ok()?;

    let search_result: DeezerSearchResponse = res.json().await.ok()?;

    search_result.data.first().map(|track| track.link.clone())
}
