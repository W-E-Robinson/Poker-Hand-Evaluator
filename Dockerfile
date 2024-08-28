# FROM rust:1.80-alpine AS builder
# 
# WORKDIR /usr/src/poker_hand_evaluator
# 
# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src
# COPY src/ src/
# RUN cargo build --release
# 
# COPY src/assets/ src/assets/
# 
# FROM alpine:latest
# 
# WORKDIR /usr/local/bin
# 
# COPY --from=builder /usr/src/poker_hand_evaluator/target/release/poker_hand_evaluator .
# 
# EXPOSE 8080
# 
# CMD ["poker_hand_evaluator"]
FROM rust:1.80-alpine

WORKDIR /usr/src/poker_hand_evaluator

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
COPY src/ src/

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
