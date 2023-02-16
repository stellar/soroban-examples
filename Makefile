default: build

all: build test

test: build
	cargo test
	cargo test --features testutils

build:
	cargo build --target wasm32-unknown-unknown --release -p soroban-token-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-cross-contract-a-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-atomic-swap-contract
	cargo build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
