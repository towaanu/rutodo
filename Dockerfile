FROM rust:alpine

# cargo fmt
RUN rustup component add rustfmt

RUN apk update && apk add \
    build-base

CMD ["cargo", "run"]
