FROM rust:1.41.0-stretch AS builder

RUN USER=root cargo new --bin workhours_bot
WORKDIR /workhours_bot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/workhours_bot*
RUN cargo build --release

CMD ["./target/release/workhours_bot"]