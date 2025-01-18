FROM rust:1.70 as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y redis-server && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/solana-wallet-analyzer /usr/local/bin/
COPY --from=builder /usr/src/app/.env.example /.env

EXPOSE 6379

COPY docker-entrypoint.sh /
RUN chmod +x /docker-entrypoint.sh

ENTRYPOINT ["/docker-entrypoint.sh"]