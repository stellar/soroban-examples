default: build

all: test

test: build
	cargo test
	cargo check --manifest-path=fuzz/Cargo.toml

build:
	soroban contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
