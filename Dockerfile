# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:latest

# Copy local code to the container image.
WORKDIR /usr/src/pubg_rust_server
COPY . .

RUN cargo install --path .
# Install production dependencies and build a release artifact.
# RUN cargo build --release

# Add metadata to the image to describe which port the container is listening on at runtime.
EXPOSE 8080

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
# CMD ["pubg_rust_server", "-a", "0.0.0.0", "-p", "8080"]
CMD pubg_rust_server


# FROM rust:latest as builder
# WORKDIR /usr/src/pubg_rust_server
# COPY . .
# RUN cargo install --path .
# # Add metadata to the image to describe which port the container is listening on at runtime.
# EXPOSE 8080

# # Service must listen to $PORT environment variable.
# # This default value facilitates local development.
# ENV PORT 8080

# FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/local/cargo/bin/pubg_rust_server /usr/local/bin/pubg_rust_server
# CMD pubg_rust_server