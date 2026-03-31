FROM rust:1.91.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install cargo-nextest --locked
RUN cargo install bacon --locked
