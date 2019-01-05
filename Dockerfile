FROM rust:1.31.1 as builder

RUN mkdir -p /app/
WORKDIR /app

RUN apt-get update && apt-get install -y gcc-6-arm-linux-gnueabihf && \
    rustup override set nightly && \
    rustup target add armv7-unknown-linux-gnueabihf

ADD Cargo.lock ./
ADD Cargo.toml ./
RUN mkdir -p .cargo
ADD docker/cargo.config .cargo/config
ADD src ./src

RUN CC=arm-linux-gnueabihf-gcc-6 cargo build --target=armv7-unknown-linux-gnueabihf --release

FROM arm32v7/debian:stable-slim

COPY --from=builder /app/target/armv7-unknown-linux-gnueabihf/release/rpi-moisture-sensor /bin/

CMD [ "/bin/rpi-moisture-sensor" ]
