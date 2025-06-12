dev:
	docker-compose -f docker-compose.yml run dev

build:
	docker build -t linkify .

run:
	docker run --rm --env-file .env linkify