FROM rust:1.72.0-alpine

ENV CARGO_TARGET_DIR=/tmp/target \
    LANG=ja_JP.utf8

RUN apk update && apk add --no-cache bash musl-dev \
    && rustup component add rustfmt \
    && cargo install cargo-watch cargo-make \
    && cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /var/www/api