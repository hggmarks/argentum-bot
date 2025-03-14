FROM rust:1.81-slim

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN cargo build --release

CMD ["./target/release/argentum-bot"]
