DEV_COMPOSE_FILE=.tools/docker-compose-dev.yml

up:
	docker compose -f $(DEV_COMPOSE_FILE) up -d --remove-orphans
	docker compose -f $(DEV_COMPOSE_FILE) exec dev cargo build --release

sh:
	docker compose -f $(DEV_COMPOSE_FILE) exec -it dev bash

check:
	docker compose -f $(DEV_COMPOSE_FILE) exec dev cargo check

run:
	docker compose -f $(DEV_COMPOSE_FILE) exec dev cargo run --release

clean:
	cargo clean

down:
	docker compose -f $(DEV_COMPOSE_FILE) down