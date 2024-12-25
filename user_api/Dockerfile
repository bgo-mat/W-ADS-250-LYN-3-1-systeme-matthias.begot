FROM rust:1.83.0-bullseye as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo install diesel_cli --version 2.0.0 --no-default-features --features mysql
RUN cargo build --release

COPY . .

FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y \
    libmariadb-dev-compat libmariadb-dev default-mysql-client ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/user_api /usr/local/bin/user_api
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY migrations migrations
COPY entrypoint.sh .

RUN chmod +x entrypoint.sh
EXPOSE 8080
ENTRYPOINT ["./entrypoint.sh"]
