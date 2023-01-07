build-db:
	cd db && docker-compose build

run-db:
	cd db && docker-compose up

dev:
	cargo watch -x run

test:
	cargo test

test-s:
	cargo test --no-default-features

migrate-db:
	cd db && sqlx database create
	cd db && sqlx migrate run

fuga: migrate-db
	pwd