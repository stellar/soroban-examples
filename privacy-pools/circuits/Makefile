CIRCOMLIB=/opt/homebrew/lib/node_modules/circomlib/circuits
CIRCUITS=main.circom commitment.circom merkleProof.circom

# These circuits are not part of the Soroban wasm build. Building/testing them
# requires the circom compiler, snarkjs, and circomlib, which are not part of
# the standard toolchain. The `build`/`test` targets are therefore no-ops so the
# repo-wide `make build`/`make test` sweep does not fail here. Use the
# `test_circuits` target to run the circuit compatibility tests.
default: build

all: test

build:
	@echo "circuits: skipping (requires circom/snarkjs/circomlib; run 'make test_circuits')"

test:
	@echo "circuits: skipping (requires circom/snarkjs/circomlib; run 'make test_circuits')"

clean:
	@rm -rf build

.circuits: $(CIRCUITS)
	@mkdir -p build
	@circom main.circom --r1cs --wasm --sym -o build -l $(CIRCOMLIB) --prime bls12381
	@circom dummy.circom --r1cs --wasm --sym -o build -l $(CIRCOMLIB) --prime bls12381
	@cd test && circom test_merkleProof.circom --wasm -o ../build -l $(CIRCOMLIB) --prime bls12381
	@ls -l build/main.r1cs build/main.sym build/main_js/main.wasm build/test_merkleProof_js/test_merkleProof.wasm

test_circuits: .circuits
	@cd test && \
		cargo run --bin lean-imt-test -- 0 0 0 0 0 && \
		node ../build/test_merkleProof_js/generate_witness.js ../build/test_merkleProof_js/test_merkleProof.wasm circuit_input.json test_merkleProof.wtns && \
		snarkjs wtns export json test_merkleProof.wtns test_merkleProof.wtns.json  && \
		head test_merkleProof.wtns.json && \
		rm circuit_input.json test_merkleProof.wtns test_merkleProof.wtns.json

fmt:
	@cd test && cargo fmt --all