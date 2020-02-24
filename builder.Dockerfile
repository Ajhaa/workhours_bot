FROM rust:1.41.0-stretch AS builder

RUN USER=root cargo new --bin workhours_bot
WORKDIR /workhours_bot

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/workhours_bot*
RUN cargo build --release
RUN cargo install --path . --verbose

CMD ["workhours_bot"]