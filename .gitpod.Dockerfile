FROM gitpod/workspace-full:2022-10-17-21-33-26

RUN mkdir -p ~/.local/bin
RUN curl -L https://github.com/stellar/soroban-cli/releases/download/v0.1.2/soroban-cli-0.1.2-x86_64-unknown-linux-gnu.tar.gz | tar xz -C ~/.local/bin soroban
RUN curl -L https://github.com/mozilla/sccache/releases/download/v0.3.0/sccache-v0.3.0-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components 1 -C ~/.local/bin sccache-v0.3.0-x86_64-unknown-linux-musl/sccache
RUN chmod +x ~/.local/bin/sccache
RUN curl -L https://github.com/taiki-e/cargo-hack/releases/download/v0.5.21/cargo-hack-x86_64-unknown-linux-gnu.tar.gz | tar xz -C ~/.local/bin cargo-hack
RUN curl -L https://github.com/watchexec/cargo-watch/releases/download/v8.1.2/cargo-watch-v8.1.2-x86_64-unknown-linux-gnu.tar.xz | tar xJ --strip-components 1 -C ~/.local/bin cargo-watch-v8.1.2-x86_64-unknown-linux-gnu/cargo-watch

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache

RUN rustup install stable
RUN rustup target add --toolchain stable wasm32-unknown-unknown
RUN rustup component add --toolchain stable rust-src
RUN rustup install nightly
RUN rustup target add --toolchain nightly wasm32-unknown-unknown
RUN rustup component add --toolchain nightly rust-src
RUN rustup default stable

RUN sudo apt-get update && sudo apt-get install -y binaryen

# RUN docker pull stellar/quickstart:soroban-dev
