dev:
	npx tailwindcss -i ./assets/css/base.css -o ./assets/css/base-new.css --watch &
	RUST_LOG=debug cargo watch -x run -i '*sqlite*'

build-css:
	npx tailwindcss -i ./assets/css/base.css -o ./assets/css/base-new.css

run:
	RUST_LOG=debug cargo run

release:
	RUST_LOG=debug cargo run --release

setup:
	cargo install cargo-watch
	npm install -D tailwindcss @tailwindcss/cli

clean:
	cargo clean
