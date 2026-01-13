restart: down up

up:
	docker compose up -d

down:
	docker compose down --remove-orphans

build:
	cargo build --release

exec:
	docker compose exec -it tornet /bin/bash
