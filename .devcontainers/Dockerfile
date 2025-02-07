FROM mcr.microsoft.com/devcontainers/rust:latest

RUN rustup target add --toolchain 1.81 wasm32-unknown-unknown
RUN rustup component add --toolchain 1.81 rust-src
RUN rustup default 1.81

RUN sudo apt-get update && sudo apt-get install -y binaryen \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y \

# Enable sparse registry support, which will cause cargo to download only what
# it needs from crates.io, rather than the entire registry.
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install tools
RUN cargo install --locked stellar-cli --features opt && \
    cargo install sccache --locked

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache