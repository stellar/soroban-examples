default: build

all: build test

test: build
	cargo test
	cargo test --features testutils
	cargo test --features token-wasm

build:
	cargo build --target wasm32-unknown-unknown --release -p soroban-token-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-liquidity-pool-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-single-offer-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-cross-contract-a-contract
	cargo build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

test-optimized: build-optimized
	cargo test
	cargo test --features testutils
	cargo test --features token-wasm

build-optimized:
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort -p soroban-token-contract
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort -p soroban-liquidity-pool-contract
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort -p soroban-single-offer-contract
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort -p soroban-cross-contract-a-contract
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
