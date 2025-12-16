FROM rust:1.92-slim AS builder

RUN apt-get update && apt-get install -y \
  pkg-config \
  libssl-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl3 \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 harmony

WORKDIR /app

COPY --from=builder /app/target/release/harmony-backend ./
COPY --from=builder /app/migrations /app/migrations

RUN chown -R harmony:harmony /app

USER harmony

EXPOSE 8000

ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=8000

CMD ["./harmony-backend"]