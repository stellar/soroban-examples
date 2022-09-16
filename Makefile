default: build-normal

all: build check test

export RUSTFLAGS=-Dwarnings

test: build-normal
	cargo hack --feature-powerset test

build: build-normal build-optimized

build-normal:
	cargo build --target wasm32-unknown-unknown --release -p soroban-token-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-liquidity-pool-contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-single-offer-contract
	cargo hack build --target wasm32-unknown-unknown --release

build-optimized:
	CARGO_TARGET_DIR=target-tiny cargo +nightly hack build --target wasm32-unknown-unknown --release \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done
	cd target-tiny/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

check: build-normal fmt
	cargo hack --feature-powerset check --all-targets
	cargo check --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
	CARGO_TARGET_DIR=target-tiny cargo +nightly clean
