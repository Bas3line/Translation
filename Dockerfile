FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY target/release/mega-chinese /app/mega-chinese
COPY migrations /app/migrations

ENV RUST_LOG=info

CMD ["/app/mega-chinese"]
