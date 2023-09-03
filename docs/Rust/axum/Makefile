build:
	docker-compose build

dev:
	docker compose exec app bash
	sqlx db create
	sqlx migrate run
	RUST_LOG=debug cargo watch -x run

test:
	cargo test

# standalone test
test-s:
	cargo test --no-default-features
