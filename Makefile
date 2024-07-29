
setup:
	pnpm install

setup-dev: setup
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall cargo-watch
	cargo build

build:
	NODE_ENV=production npx tailwindcss -c ./tailwind.config.js -o ./src/assets/tailwind.css --minify
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D clippy::all

dev-nightly:
	RUSTFLAGS="-Z threads=8" cargo watch -x run

dev:
	cargo watch -x run

prod:
	ENV="PROD" cargo run --release
