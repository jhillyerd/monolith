FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR monolith

FROM chef AS planner
COPY . .
# Calculate cargo dependencies.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /monolith/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin monolith

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR monolith
COPY --from=builder /monolith/target/release/monolith /usr/local/bin
ENTRYPOINT ["/usr/local/bin/monolith"]
