FROM rust

WORKDIR /root
ADD ./ /root

RUN cargo build
ENTRYPOINT cargo run
