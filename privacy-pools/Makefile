default: build

all: test

test: build
	cd contract && cargo test

build:
	$(MAKE) -C ../groth16_verifier || break; 
	cd contract && stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
