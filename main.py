import discord
from discord.ext import commands
from dotenv import load_dotenv
import os
import re
import requests
from urllib.parse import urlparse, urlunparse

from classes.spotify import Spotify
from classes.deezer import Deezer
from classes.youtube_music import YouTubeMusic

load_dotenv()
DISCORD_TOKEN = os.getenv("DISCORD_TOKEN")

intents = discord.Intents.default()
intents.message_content = True

bot = commands.Bot(command_prefix="!", intents=intents)

MUSIC_DOMAINS = {
    "open.spotify.com": "spotify",
    "www.deezer.com": "deezer_canonical",
    "dzr.page.link": "deezer_dynamical",
    "music.youtube.com": "youtube_music"
}

def resolve_dzr_link(dzr_url: str) -> str:
    response = requests.head(dzr_url, allow_redirects=True)
    return response.url

def get_platform_instance(platform, url):
    if platform == "spotify":
        return Spotify(url)
    elif platform == "deezer_canonical":
        return Deezer(url)
    elif platform == "youtube_music":
        return YouTubeMusic(url)
    else:
        return None

def is_music_url(platform, parsed):
    if platform == "spotify":
        return "/track/" in parsed.path
    if platform == "deezer_canonical":
        return "/track/" in parsed.path
    if platform == "youtube_music":
        return parsed.path == "/watch" and "v=" in parsed.query
    return False

def clean_url(url):
    parsed = urlparse(url)
    path = parsed.path
    query = parsed.query

    # For Spotify, remove /intl-XX/ or /XX/ from the beginning of the path
    if parsed.netloc == "open.spotify.com":
        path = re.sub(r"^/(intl-[a-z]{2}|[a-z]{2})/", "/", path)
        query = ""

    # Clean unnecessary parameters for YouTube Music
    if parsed.netloc == "music.youtube.com":
        match = re.search(r"(?:^|&)v=([^&]+)", query)
        query = f"v={match.group(1)}"

    if parsed.netloc == "www.deezer.com":
        query = ""

    cleaned = urlunparse((parsed.scheme, parsed.netloc, path, parsed.params, query, parsed.fragment))
    return cleaned

def extract_songs_urls(message):
    url_regex = r"https?://[^\s]+"
    urls = re.findall(url_regex, message)
    result = []
    for url in urls:
        parsed = urlparse(url)
        domain = parsed.netloc
        for music_domain, platform in MUSIC_DOMAINS.items():
            if domain in music_domain:
                if platform == "deezer_dynamical":
                    dynamicalToCanonical = resolve_dzr_link(url)
                    url = dynamicalToCanonical
                    parsed = urlparse(url)
                    domain = parsed.netloc
                    platform = "deezer_canonical"
                if is_music_url(platform, parsed):
                    cleaned = clean_url(url)
                    platform_instance = get_platform_instance(platform, cleaned)
                    if platform_instance:
                        result.append({
                            'url': cleaned,
                            'platform': platform,
                            'instance': platform_instance
                        })
                break
    return result

@bot.event
async def on_message(message):
    if message.author == bot.user:
        return
    songs = extract_songs_urls(message.content)
    for song in songs:
        instance = song["instance"]
        platform = song["platform"]
        track_title = instance.get_title()
        track_artist = instance.get_artist()

        for other_domain, other_platform in MUSIC_DOMAINS.items():
            if other_platform != platform:
                platform_instance = get_platform_instance(other_platform, None)
                if platform_instance:
                    link = platform_instance.search_link(track_title, track_artist)
                    if link:
                        await message.channel.send(f"{other_platform.replace('_canonical','').replace('_',' ').title()} : {link}")
    await bot.process_commands(message)

def main():
    while True:
        bot.run(DISCORD_TOKEN)

if __name__ == "__main__":
    main()