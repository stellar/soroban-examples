default: build

all: build test

test: build
	cargo test
	cargo test --features testutils

build:
	cd ../contract
	cargo build --target wasm32-unknown-unknown --release -p soroban-deployer-test-contract

	cd ../deployer
	cargo build --target wasm32-unknown-unknown --release -p soroban-deployer-contract
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
