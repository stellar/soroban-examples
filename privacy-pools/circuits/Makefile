CIRCOMLIB=/opt/homebrew/lib/node_modules/circomlib/circuits
CIRCUITS=main.circom commitment.circom merkleProof.circom

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