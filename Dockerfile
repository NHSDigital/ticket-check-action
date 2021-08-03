FROM rust:1-buster as builder
COPY . /
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Create the execution container by copying the compiled hello world to it and running it
FROM scratch
COPY --from=builder /usr/local/cargo/bin/ticket-check-action /ticket-check-action
CMD ["/ticket-check-action"]
