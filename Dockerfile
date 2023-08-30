# First stage: build the application
FROM rust:1.71 as builder

WORKDIR /usr/src

# Install git
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Clone your repository
RUN git clone https://github.com/dkzhang/gen_svg.git # 20230830-1603

WORKDIR /usr/src/gen_svg
# This build step will cache your dependencies
RUN cargo install --path .


# Second stage: the runtime environment
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/gen_svg /usr/local/bin/gen_svg

# Set the startup command to run your binary
CMD ["gen_svg"]