FROM rust:1.86-slim-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev libssl3
COPY Cargo.toml Cargo.lock ./

RUN cargo fetch --locked
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libssl3 ca-certificates

ENV PORT="8080"
ENV IP="0.0.0.0"


COPY --from=builder /app/target/release/search_project /usr/local/bin/

CMD ["/usr/local/bin/search_project"]
EXPOSE 8080
