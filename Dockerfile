FROM alpine:3.17
COPY ./target/debug/databases /usr/local/bin/server
USER app-runner
CMD ["server"]
