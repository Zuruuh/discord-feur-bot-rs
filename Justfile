set dotenv-load := true

build:
    cargo build --release
    docker build --tag zuruh/discord-feur-bot-rs .

run:
    docker run --env-file .env zuruh/discord-feur-bot-rs

build-prod:
    docker build --tag zuruh/discord-feur-bot-rs --push --platform linux/amd64 .
