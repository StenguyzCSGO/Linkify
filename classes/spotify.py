# https://developer.spotify.com/documentation/web-api

from .abstract_platform import Platform
import re
import os
import requests
import base64
import time
from dotenv import load_dotenv

load_dotenv(os.path.join(os.path.dirname(os.path.dirname(__file__)), ".env"))

class Spotify(Platform):
    client_id = os.environ.get("SPOTIFY_CLIENT_ID")
    client_secret = os.environ.get("SPOTIFY_CLIENT_SECRET")
    access_token = ""
    token_type = ""
    token_expire_in = None
    startTime = None
    headers = {"Authorization": f"Bearer {access_token}"}

    def is_token_valid(self):
        if not self.access_token or not self.startTime or not self.token_expire_in:
            return False
        return (time.time() - self.startTime) < self.token_expire_in        

    def __init__(self, url):
        super().__init__(url)
        self.track_id = None
        if url:
            match = re.search(r"/track/([a-zA-Z0-9]+)", url)
            self.track_id = match.group(1) if match else None
        if not self.is_token_valid():
            self.get_spotify_token()

    def get_spotify_token(self):
        auth_str = f"{self.client_id}:{self.client_secret}"
        b64_auth_str = base64.b64encode(auth_str.encode()).decode()

        headers = {
            "Authorization": f"Basic {b64_auth_str}",
            "Content-Type": "application/x-www-form-urlencoded"
        }
        data = {"grant_type": "client_credentials"}

        response = requests.post("https://accounts.spotify.com/api/token", headers=headers, data=data)
        token_info = response.json()

        self.access_token = token_info.get("access_token")
        self.token_type = token_info.get("token_type")
        self.token_expire_in = token_info.get("expires_in")
        self.headers = {"Authorization": f"Bearer {self.access_token}"}
        self.startTime = time.time()

    def get_title(self):
        response = requests.get(f"https://api.spotify.com/v1/tracks/{self.track_id}", headers=self.headers)
        data = response.json()
        return data["name"]

    def get_artist(self):
        response = requests.get(f"https://api.spotify.com/v1/tracks/{self.track_id}", headers=self.headers)
        data = response.json()
        return data["artists"][0]["name"]

    def get_link(self):
        return self.url
    
    def search_link(self, track_name, track_artist):
        query = f'track:"{track_name}" artist:"{track_artist}"'
        url = "https://api.spotify.com/v1/search"
        params = {
            "q": query,
            "type": "track",
            "limit": 1
        }
        headers = {"Authorization": f"Bearer {self.access_token}"}
        response = requests.get(url, headers=headers, params=params)
        if response.status_code == 200:
            data = response.json()
            items = data.get("tracks", {}).get("items", [])
            if items:
                return items[0]["external_urls"]["spotify"]
        return None