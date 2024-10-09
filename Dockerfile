FROM rust:1.81 as builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/buy-bucks .

CMD ["./buy-bucks"]
