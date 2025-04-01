# Use the official Rust image as a base
FROM mcr.microsoft.com/devcontainers/rust:bullseye

ENV CARGO_INCREMENTAL=0 \
    RUSTFLAGS='-C target-cpu=native' \
    DEBIAN_FRONTEND=noninteractive \
    PKG_CONFIG_PATH='/usr/lib/pkgconfig:/usr/local/lib/pkgconfig:/usr/share/pkgconfig'

WORKDIR "/workspace/soroban-examples"

# Enable Rust Bindings for Linux D-Bus interprocess communication(Required for Stellar CLI dependencies)
RUN apt-get update && apt-get install -y \
  libdbus-1-dev \
  libudev-dev \
  pkg-config \
  dbus





