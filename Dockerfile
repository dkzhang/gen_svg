# First stage: build the application
FROM rust:1.71 as builder

WORKDIR /usr/src/gen_svg

# Install git
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Clone your repository
RUN git clone https://github.com/dkzhang/gen_svg.git # 20230830-1530

# This build step will cache your dependencies
RUN cargo build --release


# Second stage: the runtime environment
FROM debian:buster-slim

RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/gen_svg/target/release/gen_svg /usr/local/bin/gen_svg

# Set the startup command to run your binary
CMD ["gen_svg"]