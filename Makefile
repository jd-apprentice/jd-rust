all: test lint format docker compose

dev:
	cargo watch -x run

build: src/main.rs
	cargo build --release --bin jd-rust

docker: 
	docker build -f docker/Dockerfile -t rust_template .

compose:
	docker compose -f docker/compose.yml up

test:
	cargo test

lint:
	cargo clippy -- -D warnings
 
format:
	cargo fmt --all --check

.PHONY: build docker test lint format compose