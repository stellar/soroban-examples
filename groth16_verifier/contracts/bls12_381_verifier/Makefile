.PHONY: default all test build fmt clean

default: build

WASM = ../../target/wasm32v1-none/release/bls12_381_verifier.wasm

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l $(WASM)

fmt:
	cargo fmt --all

clean:
	cargo clean
