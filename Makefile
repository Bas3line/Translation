build:
	cargo build --release

run:
	cargo run

dev:
	RUST_LOG=debug cargo run

docker-build:
	docker-compose build

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f bot

db-migrate:
	sqlx migrate run

clean:
	cargo clean
	docker-compose down -v

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

.PHONY: build run dev docker-build docker-up docker-down docker-logs db-migrate clean test fmt clippy
