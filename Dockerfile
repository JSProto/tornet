# Build Stage
FROM rust:latest as builder

# Set Docker environment variable
ENV DOCKER_ENV=true

WORKDIR /usr/src/tornet
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime Stage
FROM ubuntu:24.04

# Install Tor and necessary dependencies
# we need ca-certificates for https requests
RUN apt-get update && apt-get install -y \
    tor \
    ca-certificates \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/tornet/target/release/tornet /usr/local/bin/tornet


# Set the working directory
WORKDIR /root

# Configure Tor to listen on 0.0.0.0:9050 if we want external containers to use it
# But for the app itself, it connects to localhost:9050.

# Fix permissions for Tor directories
RUN chown -R debian-tor:debian-tor /var/lib/tor && \
    mkdir -p /var/run/tor && \
    chown -R debian-tor:debian-tor /var/run/tor && \
    chmod 700 /var/lib/tor /var/run/tor

# Create a configuration file for Tor that sets the SOCKS port to 9050
RUN echo "SocksPort 0.0.0.0:9050" > /etc/tor/torrc \
    && echo "SocksPolicy accept *" >> /etc/tor/torrc \
    && echo "Log notice stdout" >> /etc/tor/torrc \
    && echo "DataDirectory /var/lib/tor" >> /etc/tor/torrc \
    && echo "User debian-tor" >> /etc/tor/torrc

# Expose Tor SOCKS port
EXPOSE 9050

# Default command
CMD ["tornet", "--count", "0"]
