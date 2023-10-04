FROM rust:1.72

WORKDIR /srv

RUN mkdir src && touch src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

RUN rm -rf src
COPY . .

RUN cargo build --release

CMD [ "/srv/target/release/discord-feur-bot-rust" ]
