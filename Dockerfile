FROM rust:1-bullseye as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/cablemodem_stats /usr/local/bin/cablemodem_stats
COPY docker/start.sh /start.sh
CMD ["/start.sh"]