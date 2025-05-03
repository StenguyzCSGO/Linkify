# https://ytmusicapi.readthedocs.io/en/stable/

from .abstract_platform import Platform
import os
from dotenv import load_dotenv
from ytmusicapi import YTMusic, OAuthCredentials
import re

load_dotenv(os.path.join(os.path.dirname(os.path.dirname(__file__)), ".env"))

client_id = os.environ.get("YOUTUBE_CLIENT_ID")
client_secret = os.environ.get("YOUTUBE_CLIENT_SECRET")
oauth_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), "oauth.json")

ytmusic = YTMusic(
    oauth_path,
    oauth_credentials=OAuthCredentials(client_id=client_id, client_secret=client_secret)
)

class YouTubeMusic(Platform):
    def __init__(self, url):
        super().__init__(url)
        self.video_id = None
        if url:
            match = re.search(r"v=([a-zA-Z0-9_-]+)", url)
            self.video_id = match.group(1) if match else None

    def get_title(self):
        if not self.video_id:
            return "ID YouTube introuvable"
        response = ytmusic.get_song(videoId=self.video_id)
        return response["videoDetails"]["title"]

    def get_artist(self):
        if not self.video_id:
            return "ID YouTube introuvable"
        response = ytmusic.get_song(videoId=self.video_id)
        return response["videoDetails"]["author"]

    def get_link(self):
        return self.url
    
    def search_link(self, track_name, track_artist):
        query = f"{track_name} {track_artist}"
        results = ytmusic.search(query=query, filter="songs", limit=1)
        if results:
            return f"https://music.youtube.com/watch?v={results[0]['videoId']}"
        return None