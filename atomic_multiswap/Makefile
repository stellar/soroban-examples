CUR_DIR = $(shell pwd)
default: build

all: test

test: build
	cargo test
	cargo test --features testutils

build:
	@cd ../atomic_swap && make build || break; 

	@cd $(CUR_DIR);
	cargo build --target wasm32-unknown-unknown --release -p soroban-atomic-multiswap-contract
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all --check

clean:
	cargo clean
