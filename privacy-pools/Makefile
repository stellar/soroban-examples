CIRCOMLIB=/opt/homebrew/lib/node_modules/circomlib/circuits
CIRCUITS=circuits/main.circom circuits/commitment.circom circuits/merkleProof.circom

.circuits: $(CIRCUITS)
	@mkdir -p circuits/build
	@cd circuits && circom main.circom --r1cs --wasm --sym -o build -l $(CIRCOMLIB) --prime bls12381
	@cd circuits && circom dummy.circom --r1cs --wasm --sym -o build -l $(CIRCOMLIB) --prime bls12381
	@cd circuits/test && circom test_merkleProof.circom --wasm -o ../build -l $(CIRCOMLIB) --prime bls12381
	@ls -l circuits/build/main.r1cs circuits/build/main.sym circuits/build/main_js/main.wasm circuits/build/test_merkleProof_js/test_merkleProof.wasm

test_circuits: .circuits
	@cd circuits/test && \
		cargo run --bin lean-imt-test -- 0 0 0 0 0 && \
		node ../build/test_merkleProof_js/generate_witness.js ../build/test_merkleProof_js/test_merkleProof.wasm circuit_input.json test_merkleProof.wtns && \
		snarkjs wtns export json test_merkleProof.wtns test_merkleProof.wtns.json  && \
		head test_merkleProof.wtns.json && \
		rm circuit_input.json test_merkleProof.wtns test_merkleProof.wtns.json