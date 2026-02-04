@just:
    just --list

install:
    npm install

tailwind: install
    npx tailwindcss -i ./input.css -o ./public/styles.css --minify

tailwind-watch: install
    npx tailwindcss -i ./input.css -o ./public/styles.css --watch

serve: tailwind
    trunk serve --port 3000 --open

build: tailwind
    -cargo clean
    rm -rf dist .trunk
    trunk build --release

check:
    cargo check --all --tests
    cargo fmt --all -- --check

format:
    cargo fmt

lint:
    cargo clippy --all --tests -- -D warnings
