
setup:
	npm install

setup-dev: setup
	curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
	cargo binstall cargo-watch
	npx tailwindcss -o ./assets/tailwind.css
	cargo build

build:
	NODE_ENV=production npx tailwindcss -c ./tailwind.config.js -o ./assets/tailwind.css --minify
	cargo build --release

lint:
	cargo clippy

run-dev:
	cargo watch -x run

run-prod:
	cargo run --release