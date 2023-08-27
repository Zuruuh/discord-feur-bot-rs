set dotenv-load := true

build-prod:
    docker build --tag zuruh/discord-feur-bot-rs --push .
