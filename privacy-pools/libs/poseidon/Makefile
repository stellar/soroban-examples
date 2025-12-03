default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	mkdir -p opt/
	stellar contract optimize \
		--wasm ../target/wasm32v1-none/release/poseidon.wasm \
		--wasm-out opt/poseidon.wasm
	@ls -l opt/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

.PHONY: default all test build build-demo install fmt clean
