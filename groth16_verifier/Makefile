default: build

all: test

test: build
	cargo test --workspace

build:
	stellar contract build --package soroban-groth16-verifier-bls12-381
	stellar contract build --package soroban-groth16-verifier-bn254
	mkdir -p opt/
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/soroban_groth16_verifier_bls12_381.wasm \
		--wasm-out opt/soroban_groth16_verifier_bls12_381.wasm
	stellar contract optimize \
		--wasm target/wasm32v1-none/release/soroban_groth16_verifier_bn254.wasm \
		--wasm-out opt/soroban_groth16_verifier_bn254.wasm
	@ls -l opt/*.wasm
fmt:
	cargo fmt --all

clean:
	cargo clean
