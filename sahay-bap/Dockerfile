FROM rust:1.67 as build
RUN USER=root cargo new --bin sahay-bap
WORKDIR /sahay-bap
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/sahay-bap*
RUN cargo build --release
FROM rust:1.67-slim-buster
COPY --from=build /holodeck/target/release/sahay-bap .
CMD ["./sahay-bap"]
