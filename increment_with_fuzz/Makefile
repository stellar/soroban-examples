default: build

all: test

test: build
	cargo test
	cargo check --manifest-path=fuzz/Cargo.toml

build:
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
