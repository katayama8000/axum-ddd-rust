FROM rust:1.82.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-watch
RUN cargo install cargo-nextest --locked