# Use the official Rust image as a base
FROM mcr.microsoft.com/devcontainers/rust:bullseye

ENV DEBIAN_FRONTEND=noninteractive \
    PKG_CONFIG_PATH='/usr/lib/pkgconfig:/usr/local/lib/pkgconfig:/usr/share/pkgconfig' \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_INCREMENTAL=0

# Enable Rust Bindings for Linux D-Bus interprocess communication(Required for Stellar CLI dependencies)
RUN apt-get update && apt-get install -y \
  libdbus-1-dev \
  libudev-dev \
  pkg-config \
  dbus \
  binaryen

RUN mkdir -p /home/linuxbrew/ && \
    chown -R vscode:vscode /home/linuxbrew/

USER vscode

RUN HOMEBREW_BUNDLE_NO_UPGRADE=1 HOMEBREW_BUNDLE_INSTALL_CLEANUP=1 HOMEBREW_NO_INSTALL_UPGRADE=1 && \
    HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_ENV_SYNC_STRICT=1 HOMEBREW_ON_LINUX=1 && \
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" && \
    /home/linuxbrew/.linuxbrew/bin/brew install sccache cargo-watch stellar-cli && \
    /home/linuxbrew/.linuxbrew/bin/brew cleanup --scrub

ENV SCCACHE_DIR=/home/vscode/.cache/sccache \
    SCCACHE_CONF=/home/vscode/.config/sccache/config \
    SCCACHE_DIRECT=true \
    RUSTC_WRAPPER=sccache \
    SCCACHE_CACHE_SIZE=5G

CMD ["/bin/bash", "-c", "make", "all", "&"]




