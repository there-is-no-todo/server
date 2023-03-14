FROM rust:1.67 as builder
WORKDIR /usr/src/databases
COPY Cargo.toml .
RUN mkdir src \
    && echo "// dummy file" > src/lib.rs \
    && cargo build
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/databases /usr/local/bin/databases
CMD ["databases"]
