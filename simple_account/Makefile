default: build

all: build test

test: build
	cargo test
	cargo test --features testutils

build:
	cargo build --target wasm32-unknown-unknown --release -p soroban-simple-account-contract
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all --check

clean:
	cargo clean
