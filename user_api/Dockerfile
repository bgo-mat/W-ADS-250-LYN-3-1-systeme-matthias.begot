# Étape 1 : Build
FROM rust:1.83.0-bullseye as builder

WORKDIR /app

# Cache des dépendances
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copie du code source réel
COPY . .
# Installation de diesel_cli pour exécuter les migrations
RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo build --release

# Étape 2 : Runtime
FROM debian:bullseye-slim AS runtime

WORKDIR /app

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
