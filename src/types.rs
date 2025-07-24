#[derive(Debug)]
pub struct TrackInfo {
    pub original_platform: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
}
