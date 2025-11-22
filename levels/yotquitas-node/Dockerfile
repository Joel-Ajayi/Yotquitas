# Multi-stage build for Yotquitas Node
# Build context should be workspace root (../..)
FROM rust:1.70 as builder

WORKDIR /app

# Copy both packages (needed for path dependencies)
COPY levels/yotquitas-core ./levels/yotquitas-core
COPY levels/yotquitas-node ./levels/yotquitas-node

# Build the node
WORKDIR /app/levels/yotquitas-node
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/yotquitas-node /app/yotquitas-node

# Copy config files
COPY --from=builder /app/levels/yotquitas-node/config /app/config

# Create data directory
RUN mkdir -p /app/data

# Expose RPC port
EXPOSE 8545

# Run the node with dev config by default
CMD ["./yotquitas-node", "--config-type", "dev"]



