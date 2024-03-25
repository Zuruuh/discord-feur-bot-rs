set dotenv-load := true

build:
    cargo build --release

run:
    docker run --env-file .env zuruh/discord-feur-bot-rs

publish:
    docker build . --tag registry.zuruh.dev/discord-feur-bot --push --platform linux/amd64
