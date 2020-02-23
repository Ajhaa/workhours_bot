# faster rust docker build
# https://benjamincongdon.me/blog/2019/12/04/Fast-Rust-Docker-Builds-with-cargo-vendor/

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
RUN cargo install --path . --verbose


FROM debian:stable
RUN apt-get update && apt-get install -y libssl-dev ca-certificates

COPY --from=builder /usr/local/cargo/bin/workhours_bot /bin

CMD ["workhours_bot"]