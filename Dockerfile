FROM rust:1.67 as builder
WORKDIR /usr/src/databases
COPY . .
RUN cargo install --path .

FROM alpine:3.17
COPY --from=builder /usr/local/cargo/bin/databases /usr/local/bin/databases
RUN adduser -u 10001 -D app-runner
USER app-runner
CMD ["databases"]
