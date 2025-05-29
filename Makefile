dev: setup
	npx tailwindcss -i ./assets/css/base.css -o ./assets/css/base-new.css --watch &
	RUST_LOG=debug cargo watch -x run -i '*sqlite*'

build-css: setup
	npx tailwindcss -i ./assets/css/base.css -o ./assets/css/base-new.css

run: build-css
	RUST_LOG=debug cargo run

release: build-css
	RUST_LOG=debug cargo run --release

setup:
	cargo install cargo-watch
	npm install -D tailwindcss @tailwindcss/cli

clean:
	cargo clean
