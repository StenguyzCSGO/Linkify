# Linkify 🎵 – Universal Music Link Discord Bot (Rust)

Tired of friends ignoring your music links because they use a different platform? Linkify converts music links between platforms so everyone can listen.

Built in Rust for performance and safety (after a quick Python PoC that worked but wasn’t production-grade).

---

## 💡 What is Linkify?

Linkify is a friendly Discord bot written in Rust. Give it a track URL (Spotify, Deezer, etc.) and it replies with equivalent links on other platforms. No more platform wars — just music.

---

## 🛠 Features

- Slash commands designed for speed and clarity
  - `/convert <url>`: Detects the platform, fetches track metadata (title, artist, album), and returns matching links on other platforms
  - `/ping`: Quick health check
- Platforms support
  - ✅ Spotify — fully supported
  - 🚧 Deezer — nearing full support
  - 🔜 YouTube Music — coming soon
  - 🔜 SoundCloud — coming soon
  - ⛔ Apple Music — not planned (requires a $99/year developer license)

Note: This version uses slash commands for conversions.

---

## 🚀 Getting Started

Development and production are containerized. The Makefile provides a smooth workflow.

1) Clone the repository and go to the bot folder
   - Top-level repo contains both the bot and the web app. From the root, `cd Linkify-Bot`.

2) Configure environment variables
   - Copy `.env.example` to `.env`
   - Fill in your credentials (see “API Keys & Tokens Setup” below)
   - Optional: set `GUILD_ID` for faster slash-command registration on a test server during development

3) Start the dev container and build
   - `make up` builds the image and compiles the project inside Docker

4) Run the bot
   - `make run` to launch in release mode inside the dev container

5) Iterate quickly
   - `make check` to run `cargo check`
   - `make sh` to open a shell inside the container
   - `make down` to stop and remove the dev container
   - `make clean` to remove the `target/` folder

Production
- `make prod` builds and runs Linkify with the production image

---

## ⚙️ API Keys & Tokens Setup

Create a `.env` file (based on `.env.example`, which is versioned) and fill the following variables:

1) Discord Bot Token
- Create a bot in the Discord Developer Portal and copy the token
- `.env`:
  - `DISCORD_TOKEN=your_discord_bot_token`

2) Spotify API Credentials
- Create an app in the Spotify Developer Dashboard
- `.env`:
  - `SPOTIFY_CLIENT_ID=your_spotify_client_id`
  - `SPOTIFY_CLIENT_SECRET=your_spotify_client_secret`

3) Deezer API
- No API key required for basic usage
- Docs: https://developers.deezer.com/api

4) Faster command registration in development (optional)
- Set a test server ID so slash commands register instantly while developing
- `.env` (optional):
  - `GUILD_ID=your_discord_guild_id`

Tip: Never commit your personal `.env`. Only `.env.example` should be versioned to document required variables.

---

## 🔐 Discord Permissions

For best results, make sure the bot has:
- Read Messages / View Channels
- Send Messages
- Use Slash Commands (applications.commands)
- Embed Links (recommended)

Generate an invite link in the Discord Developer Portal with scopes `bot` and `applications.commands` and the minimal permissions above.

---

## 📂 Project Structure

```
├── .env.example               # Example configuration
├── .tools/                    # Docker setup (dev/prod)
│   ├── Dockerfile.dev
│   ├── Dockerfile.prod
│   └── docker-compose-dev.yml
├── Cargo.toml                 # Rust dependencies
├── LICENSE                    # MIT license
├── Makefile                   # Dev & prod helper commands
├── README.md                  # Documentation
└── src/
    ├── commands/              # Bot commands
    │   ├── convert.rs
    │   ├── ping.rs
    │   └── mod.rs
    ├── platforms/             # Platform connectors
    │   ├── deezer.rs
    │   ├── spotify.rs
    │   ├── youtube_music.rs
    │   ├── handler.rs
    │   └── mod.rs
    ├── types.rs               # Shared data structures
    └── main.rs                # Bot entry point
```

---

## 🧠 Tech Stack

- Rust
- serenity + poise (Discord + slash commands)
- tokio (async runtime)
- reqwest, serde, dotenv
- Docker for dev/prod workflows

---

## 🤝 Contributing

Want to improve Linkify or add support for new platforms? PRs are welcome! Feel free to open an issue first to discuss ideas.

---

## 📜 License

This project is licensed under the [MIT License](./LICENSE).

---
🎵 Music is for everyone. Linkify makes sure of that.
