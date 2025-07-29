DEV_COMPOSE_FILE=.tools/docker-compose-dev.yml
PROD_DOCKER_FILE=.tools/Dockerfile.prod

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
	docker compose -f $(DEV_COMPOSE_FILE) exec dev cargo clean

fmt:
	docker compose -f $(DEV_COMPOSE_FILE) exec dev cargo +nightly fmt	

down:
	docker compose -f $(DEV_COMPOSE_FILE) down

prod:
	git switch main
	git pull
	docker build -f $(PROD_DOCKER_FILE) -t linkify-prod .
	-docker stop linkify-prod
	-docker rm linkify-prod
	docker run -d --restart unless-stopped --name linkify-prod -p 8080:8080 linkify-prod