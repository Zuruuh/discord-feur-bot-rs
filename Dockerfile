FROM rust:1.72
ENV BOT_TOKEN=$BOT_TOKEN

WORKDIR /srv

COPY . .

RUN cargo build --release

CMD [ "./target/release/discord-feur-bot-rs" ]
