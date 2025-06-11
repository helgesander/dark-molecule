FROM rust:latest

WORKDIR /app
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

COPY . .

RUN trunk build --release

ENTRYPOINT [ "trunk", "serve", "--release" ]