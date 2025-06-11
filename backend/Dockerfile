FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/backend /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/
COPY migrations ./migrations
COPY diesel.toml .env ./

# Устанавливаем только runtime-зависимости
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

# Entrypoint
COPY entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["entrypoint.sh"]
