FROM rust:1.67 as builder
WORKDIR /usr/src/databases
COPY Cargo.toml .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
COPY . .
RUN cargo install --debug --path .

FROM debian:bullseye-slim
WORKDIR /srv
RUN apt-get update && apt-get install -y libsqlite3-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/databases ./databases
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./Rocket.toml ./Rocket.toml
COPY ./diesel.toml ./diesel.toml
CMD ["./databases"]
