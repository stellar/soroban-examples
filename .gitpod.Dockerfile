FROM gitpod/workspace-full:2023-01-16-03-31-28

RUN mkdir -p ~/.local/bin
RUN curl -L -o ~/.local/bin/soroban https://github.com/stellar/soroban-tools/releases/download/v0.4.0/soroban-cli-0.4.0-x86_64-unknown-linux-gnu
RUN chmod +x ~/.local/bin/soroban
RUN curl -L https://github.com/mozilla/sccache/releases/download/v0.3.0/sccache-v0.3.0-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components 1 -C ~/.local/bin sccache-v0.3.0-x86_64-unknown-linux-musl/sccache
RUN chmod +x ~/.local/bin/sccache
RUN curl -L https://github.com/watchexec/cargo-watch/releases/download/v8.1.2/cargo-watch-v8.1.2-x86_64-unknown-linux-gnu.tar.xz | tar xJ --strip-components 1 -C ~/.local/bin cargo-watch-v8.1.2-x86_64-unknown-linux-gnu/cargo-watch

ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=5G
ENV SCCACHE_DIR=/workspace/.sccache

# Remove the existing rustup installation before updating due to:
# https://github.com/gitpod-io/workspace-images/issues/933#issuecomment-1272616892
RUN rustup self uninstall -y
RUN rm -rf .rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN rustup update stable
RUN rustup target add --toolchain stable wasm32-unknown-unknown
RUN rustup component add --toolchain stable rust-src
RUN rustup update nightly
RUN rustup target add --toolchain nightly wasm32-unknown-unknown
RUN rustup component add --toolchain nightly rust-src
RUN rustup default stable

RUN sudo apt-get update && sudo apt-get install -y binaryen
