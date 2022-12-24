FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
# Calculate cargo dependencies.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build test & release dependencies - this is the caching layer!
RUN cargo chef cook --recipe-path recipe.json | tee /tmp/cook.txt
RUN cargo chef cook --release --recipe-path recipe.json | tee /tmp/cook.txt

# Build & test application
COPY . .
RUN cargo test
RUN cargo build --release --bin monolith

FROM gcr.io/distroless/cc-debian11:debug
WORKDIR app
COPY --from=builder /app/target/release/monolith .

ENTRYPOINT ["/app/monolith"]
