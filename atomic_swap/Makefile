default: build

all: test

test: build
	cargo test
	cargo test --features testutils

build:
	cargo build --target wasm32-unknown-unknown --release -p soroban-atomic-swap-contract
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all --check

clean:
	cargo clean
