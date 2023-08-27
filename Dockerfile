FROM rust:1.72
ENV BOT_TOKEN=$BOT_TOKEN

WORKDIR /srv

COPY . .

RUN cargo build --release

CMD [ "/srv/target/release/discord-feur-bot-rust" ]
