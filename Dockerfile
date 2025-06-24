FROM messense/rust-musl-cross:aarch64-musl AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static-debian12
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/test-app /usr/local/bin/test-app
ENTRYPOINT ["/usr/local/bin/test-app"]