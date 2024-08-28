FROM rust:1.80-alpine

WORKDIR /usr/src/poker_hand_evaluator

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src/ src/

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
