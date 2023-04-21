default: build

all: test

test: build
	cargo test
	cargo test --features testutils

build:
	$(MAKE) -C ../contract || break;
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all --check

clean:
	cargo clean
