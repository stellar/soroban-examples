FROM gitpod/workspace-full:latest

# Remove the existing rustup installation before updating due to:
# https://github.com/gitpod-io/workspace-images/issues/933#issuecomment-1272616892
RUN rustup self uninstall -y
RUN rm -rf .rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y

RUN rustup install 1.81
RUN rustup target add --toolchain 1.81 wasm32-unknown-unknown
RUN rustup component add --toolchain 1.81 rust-src
RUN rustup default 1.81

RUN sudo apt-get update && sudo apt-get install -y binaryen

# Enable sparse registry support, which will cause cargo to download only what
# it needs from crates.io, rather than the entire registry.
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install cargo binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y stellar-cli cargo-watch sccache

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache