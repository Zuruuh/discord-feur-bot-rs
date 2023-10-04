set dotenv-load := true

build:
    docker build --tag zuruh/discord-feur-bot-rs .

run:
    docker run --env-file .env zuruh/discord-feur-bot-rs

dev: build run

build-prod:
    docker build --tag zuruh/discord-feur-bot-rs --push --platform linux/amd64 .
