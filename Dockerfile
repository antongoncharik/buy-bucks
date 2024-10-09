FROM rust:1.81 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/buy-bucks .

CMD ["buy-bucks"]
