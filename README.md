# Linkify 🎵

**Linkify** is a Discord bot written in **Rust** that allows converting music links from one platform (e.g., Spotify, Deezer) to equivalent links on other platforms.

---

## ⚡ Usage

You can use the following `make` commands to simplify development and production:

* `make up` : Start the development Docker container and build the project in release mode.
* `make sh` : Open a shell inside the development Docker container.
* `make check` : Run `cargo check` inside the development container to verify the code.
* `make run` : Run the project in release mode inside the development container.
* `make clean` : Delete the `target` folder (removes compiled binaries and dependency artifacts) to rebuild from scratch.
* `make down` : Stop and remove the development Docker container.
* `make prod` : Build and run Linkify in production.

These commands help you manage the Docker-based Rust development environment quickly and efficiently.

---

## 🌐 Features

* `/ping` command to check that the bot is running.
* `/convert <url>` command:

  * Detects the platform of the provided music link.
  * Retrieves track information (title, artist, album).
  * Generates equivalent links on other supported platforms.
* Currently supported platforms:

  * ✅ Spotify
  * ✅ Deezer
  * 🔜 Support for additional platforms coming soon (like YT Music)

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
    │   ├── handler.rs
    │   └── mod.rs
    ├── types.rs               # Shared data structures
    └── main.rs                # Bot entry point
```

---

## 📜 License

This project is licensed under the [MIT License](./LICENSE).
