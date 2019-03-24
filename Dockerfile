FROM rust

WORKDIR /root
ADD ./ /root

RUN cargo build
ENTRYPOINT cargo run -- --socket localhost:8000 --base_url localhost:8000