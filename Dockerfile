FROM rust:1.86-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev libssl3
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY src ./src
COPY static ./static
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libssl3

ENV PORT="3000"
ENV IP="127.0.0.1"

COPY --from=builder /app/target/release/search_project /usr/local/bin/
COPY --from=builder /app/static /usr/local/bin/static
CMD ["/usr/local/bin/search_project"]
EXPOSE 3000
