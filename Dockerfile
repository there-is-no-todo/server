FROM rust:1.67 as builder
WORKDIR /usr/src/databases
COPY Cargo.toml .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libsqlite3-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/databases /srv/databases
COPY --from=builder /usr/src/databases/migrations /srv/migrations
COPY --from=builder /usr/src/databases/db /srv/db
ENTRYPOINT ["/srv/databases"]
