# Linkify

## Usage

Use the `make` commands below to simplify development and production release.


- `make dev` : Build the dependencies once, then get a Docker shell to rebuild on every change.
- `make build` : Build the project in release mode using the customized Docker image.
- `make run` : Run the binary previously built with `make build` in an optimized Docker image.