default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	mkdir -p opt/
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/soroban_ark_bn254_contract.wasm \
		--wasm-out opt/soroban_ark_bn254_contract.wasm
	@ls -l opt/*.wasm
fmt:
	cargo fmt --all

clean:
	cargo clean
