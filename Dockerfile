FROM rust:1.81 as builder

WORKDIR /opt/app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /opt/app

COPY --from=builder /opt/app/target/release/buy-bucks /opt/app/buy-bucks

CMD ["/opt/app/buy-bucks"]
