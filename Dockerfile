FROM rust

WORKDIR /root
ADD ./ /root

RUN cargo build

ENV base_url https://phonead.ventures
ENV socket 0.0.0.0:8000
ENTRYPOINT cargo run -- --socket $socket --base_url $base_url
