FROM rust:1.72.0-alpine

ENV CARGO_TARGET_DIR=/tmp/target \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apk update && apk add --no-cache \
    bash \
    # TODO: ログを出力できるようにしたい
    # locales \
    make \
    musl-dev \
    # && echo "ja_JP.UTF-8" > /etc/locale.gen \
    # && locale-gen \
    && rustup component add rustfmt \
    && cargo install cargo-watch cargo-make \
    && cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /var/www/api