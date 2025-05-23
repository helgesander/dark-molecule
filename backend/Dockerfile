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
LABEL authorts="helgesander"
WORKDIR /app
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
RUN apt-get update && apt-get install -y \
    libpq5 \
    libpq-dev \
    postgresql-client \
    nmap \
    && rm -rf /var/lib/apt/lists/* \

# Install golang
RUN wget https://go.dev/dl/go1.24.2.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.24.2.linux-amd64.tar.gz && \
    export PATH=$PATH:/usr/local/go/bin && \
    go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest \

USER daemon
COPY entrypoint.sh .
COPY src/db src/db
COPY migrations migrations
COPY diesel.toml .
ENTRYPOINT ["./entrypoint.sh"]
