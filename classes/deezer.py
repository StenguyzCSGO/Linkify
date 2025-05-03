# https://developers.deezer.com/api/search

from .abstract_platform import Platform
import requests
import re

class Deezer(Platform):
    def __init__(self, url):
        super().__init__(url)
        self.track_id = None
        if url:
            match = re.search(r"/track/(\d+)", url)
            self.track_id = match.group(1) if match else None

    def get_title(self):
        if not self.track_id:
            return "ID Deezer introuvable"
        response = requests.get(f"https://api.deezer.com/track/{self.track_id}")
        data = response.json()
        return data.get("title", "Titre inconnu")

    def get_artist(self):
        if not self.track_id:
            return "ID Deezer introuvable"
        response = requests.get(f"https://api.deezer.com/track/{self.track_id}")
        data = response.json()
        return data.get("artist", {}).get("name", "Artiste inconnu")

    def get_link(self):
        return self.url
    
    def search_link(self, track_name, track_artist):
        response = requests.get(f"https://api.deezer.com/search?q=artist:\"{track_artist}\" track:\"{track_name}\"")
        data = response.json()
        if data.get("data") and len(data["data"]) > 0:
            return data["data"][0]["link"]
        return None