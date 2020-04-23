FROM rust as builder
WORKDIR /project
COPY ./src .
COPY ./Cargo.toml .

RUN cargo build --release --target x86_64-linux-kernel

FROM alpine
COPY --from=builder /project/target/release/learn-compiler /usr/local/bin

ENTRYPOINT [ "learn-compiler" ]
