# Use a minimal Rust image
FROM rust:1.75-bookworm as builder

# install protobuf compiler
RUN apt-get update \
    && apt-get install -y protobuf-compiler libprotobuf-dev

# Copy the source code
COPY . /usr/src/zkp-auth
WORKDIR /usr/src/zkp-auth

# Build the application
RUN make build

# Copy the binary to a new image
FROM debian:bookworm-slim
COPY --from=builder /usr/src/zkp-auth/target/release/zkp-auth /zkp-auth
