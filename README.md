# Linkify

## Usage

Use the following `make` commands to simplify development and production release:

- `make up` : Start the development Docker container and build the project in release mode.
- `make sh` : Open a shell inside the development Docker container.
- `make check` : Run `cargo check` inside the development Docker container to verify the code.
- `make run` : Run the project in release mode inside the development Docker container.
- `make clean`: Delete the `target` folder (which contains binaries and artefacts from dependencies) to rebuild from scratch.
- `make down` : Stop and remove the development Docker container.
- `make prod` : Easily deploy Linkify in production.

These commands help you manage the Docker-based Rust development environment