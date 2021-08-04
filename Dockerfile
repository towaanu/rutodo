FROM rust:alpine

# cargo fmt
RUN rustup component add rustfmt

RUN apk update && apk add \
    build-base \
    openssl-dev

RUN cargo install movine

CMD ["cargo", "run"]
