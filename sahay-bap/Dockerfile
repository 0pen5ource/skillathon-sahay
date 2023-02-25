FROM rust:1.67
RUN USER=root cargo new --bin sahay-bap
WORKDIR /sahay-bap
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN cargo build --release
CMD ["./target/release/sahay-bap"]
