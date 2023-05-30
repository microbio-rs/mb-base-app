FROM rust:1.69 as build

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL sparse

RUN USER=root cargo new --bin mb-base-app
WORKDIR /mb-base-app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./settings.toml ./settings.toml

RUN rm ./target/release/mb-base-app*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /mb-base-app/target/release/mb-base-app .
COPY --from=build /mb-base-app/settings.toml .

CMD ["/mb-base-app"]
