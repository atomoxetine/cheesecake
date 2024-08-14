setup:
	pnpm install

install: setup
	cargo install cargo-watch
	cargo install sqlx-cli

build:
	pnpm make-tailwind
	cargo build --release

test:
	cargo test

lint:
	cargo clippy

## Create a .cargo/config.toml instead and configure your IDE's rust-analyzer to use it, or else caching
## won't work and you'll be setting yourself up for a bad time, trust me.
# dev-nightly:
# 	RUSTFLAGS="-Z threads=8" cargo watch -x run

dev:
	cargo watch -x run

prod:
	ENV="PROD" cargo run --release