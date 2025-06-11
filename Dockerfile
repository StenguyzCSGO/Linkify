FROM rust:1.87-slim AS build

WORKDIR /usr/src/linkify

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/linkify/target/release/linkify /usr/local/bin/linkify

CMD ["linkify"]