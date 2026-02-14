FROM rust:1.75-slim as builder
WORKDIR /usr/src/novacore
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/novacore/target/release/novacore-gateway .
COPY .env .
EXPOSE 3000
CMD ["./novacore-gateway"]