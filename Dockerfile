FROM rust:1.85-slim-bookworm AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY cli/ cli/
WORKDIR /build/cli
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/cli/target/release/internode /usr/local/bin/internode
RUN chmod +x /usr/local/bin/internode
WORKDIR /root
CMD ["/bin/bash"]
