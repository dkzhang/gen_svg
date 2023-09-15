# First stage: build the application
FROM rust:1.71 as builder

WORKDIR /usr/src

# Install git
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Clone your repository
RUN git clone https://github.com/dkzhang/gen_svg.git # 20230915-1700

WORKDIR /usr/src/gen_svg
# This build step will cache your dependencies
RUN cargo install --path .


# Second stage: the runtime environment
FROM debian:bullseye-slim

RUN #apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/gen_svg
# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/gen_svg /usr/local/bin/gen_svg
COPY --from=builder /usr/src/gen_svg/config /usr/gen_svg/config

# Set the startup command to run your binary
CMD ["gen_svg"]