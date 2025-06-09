# Dev Mode Jobs

dev-ui:
	RUST_LOG=debug cargo watch -x "run --bin engelsystem-rs-frontend" -i '*sqlite*'

dev-api:
	RUST_LOG=debug cargo watch -x "run --bin engelsystem-rs-api" -i '*sqlite*'

dev-css:
	npx tailwindcss -i ./engelsystem-rs-frontend/assets/css/base.css -o ./engelsystem-rs-frontend/assets/css/base-gen.css --watch

dev:
	npm install
	npm run dev

# Release Mode Jobs

build-css:
	npx tailwindcss -i ./engelsystem-rs-frontend/assets/css/base.css -o ./engelsystem-rs-frontend/assets/css/base-gen.css

ui: build-css
	cargo run --release --bin engelsystem-rs-frontend

api: build-css
	cargo run --release --bin engelsystem-rs-api

# Util

clean:
	cargo clean

PHONY: dev-ui dev-api dev-css dev build-css ui api clean
