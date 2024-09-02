setup:
	cp -n .env.example .env
	cd build_utils && pnpm install
	cargo install sqlx-cli

clean:
	rm -rf dist/assets dist/Logs dist/app build_utils/node_modules Logs # Keeps .env
	cargo clean

build:
	rm -rf dist/assets dist/Logs dist/app # Keeps .env
	mkdir -p dist
	cargo build -p app --release
	cp target/release/app dist/
	cd build_utils && zsh build-assets.zsh
	cd build_utils && pnpm make-tailwind
	cp -n .env dist/.env

test:
	cargo test

lint:
	cargo clippy --fix

force-lint:
	cargo clippy --fix --allow-dirty --allow-staged

dev:
	RUST_SPANTRACE=1 RUST_BACKTRACE=full RUST_LIB_BACKTRACE=1 cargo run -p app

prod: build
	cd dist && ./app

.PHONY: test lint force-lint dev prod