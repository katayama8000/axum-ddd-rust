FROM rust:1.88.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-nextest --locked
RUN cargo install bacon --locked

# Install mysql-client
RUN apt-get update && apt-get install -y default-mysql-client && rm -rf /var/lib/apt/lists/*
