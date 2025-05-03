# 🎧 Linkify – The Universal Music Link Discord Bot

> Tired of your friends refusing to check out the song you sent just because they use a different music platform?  
> **Linkify** fixes that.

---

## 💡 What is Linkify?

**Linkify** is a friendly Discord bot that automatically detects music links (like Spotify, Deezer, or YouTube Music) in messages and replies with equivalent links on other major platforms.

No more platform wars — just music.

---

## 🛠 Features

- 🔍 Detects Spotify, Deezer, and YouTube Music track links in Discord messages
- 🔁 Replies with matching links on other platforms
- ⚙️ Works instantly — no need for a command prefix
- 🧠 Smart handling to avoid spamming or duplicating replies

---

## 🚀 Getting Started

1. **Clone this repository**
   ```
   git clone https://github.com/StenguyzCSGO/Linkify.git
   cd Linkify
   ```

2. **Create a virtual environment (recommended)**
   ```
   python -m venv venv
   # On Windows:
   venv\Scripts\activate
   # On macOS/Linux:
   source venv/bin/activate
   ```

3. **Install dependencies**
   ```
   pip install -r requirements.txt
   ```

4. **Configure environment variables**
   - Create a `.env` file (see `.env.example` if available)
   - Add your Discord token and API keys for Spotify, Deezer, and YouTube Music

5. **Run the bot**
   ```
   python main.py
   ```

6. **Send a music link** in any text channel  
   Linkify will reply with universal links that everyone can use

---

## ⚙️ API Keys & Tokens Setup

To run Linkify, you need to create a `.env` file (you can copy `.env.example`, which is versioned in the repository) and fill it with the following credentials.  
**Never commit your personal `.env` file. Only `.env.example` should be versioned to show required variables.**

### 1. Discord Bot Token
- Go to the [Discord Developer Portal](https://discord.com/developers/applications/)
- Create a new application, add a bot, and copy the **Bot Token**
- Add to your `.env`:
  ```
  DISCORD_TOKEN=your_discord_bot_token
  ```

### 2. Spotify API Credentials
- Go to the [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/)
- Create an app to get your **Client ID** and **Client Secret**
- Add to your `.env`:
  ```
  SPOTIFY_CLIENT_ID=your_spotify_client_id
  SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
  ```

### 3. Deezer API
- No API keys are required for Deezer.
- See the [Deezer API documentation](https://developers.deezer.com/api) for more information.

### 4. YouTube Music API (Unofficial)
- This project uses the [ytmusicapi](https://ytmusicapi.readthedocs.io/en/stable/index.html#), an unofficial API for YouTube Music (Google does not provide an official API).
- Follow the [ytmusicapi setup guide](https://ytmusicapi.readthedocs.io/en/stable/setup/index.html) to generate the required authentication headers or credentials.
- Add to your `.env` if needed:
  ```
  YOUTUBE_CLIENT_ID=your_youtube_client_id
  YOUTUBE_CLIENT_SECRET=your_youtube_client_secret
  ```

> **Tip:** Never share your `.env` file. Use `.env.example` to show the required variables without secrets.

---

## 🔐 Permissions Required

To work properly, Linkify needs:
- Read Messages
- Send Messages
- Embed Links (optional but recommended)
- Or administrator to not get mad

You can configure these permissions in the [Discord Developer Portal](https://discord.com/developers/applications/)  
and generate an invite link using OAuth2.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).  
Feel free to use, modify, and share — just give credit!

---

## 🤝 Contributing

Want to improve Linkify or add support for new platforms?  
Pull requests are welcome! Feel free to open an issue first to discuss your ideas.

---

## 🧠 Tech Stack

- Python
- discord.py
- External APIs for link conversion (Spotify, Deezer, YouTube Music)

---

## 📬 Contact

Made with ❤️ by [StenguyzCSGO](https://github.com/StenguyzCSGO)
Have a feature request or spotted a bug? Open an issue!

---

## 🎵 Music is for everyone. Linkify makes sure of that.
