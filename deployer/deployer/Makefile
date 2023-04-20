CUR_DIR = $(shell pwd)

default: build

all: build test

test: build
	cargo test
	cargo test --features testutils

build:
	@cd ../contract && make build || break; 

	@cd $(CUR_DIR);
	cargo build --target wasm32-unknown-unknown --release -p soroban-deployer-contract
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
