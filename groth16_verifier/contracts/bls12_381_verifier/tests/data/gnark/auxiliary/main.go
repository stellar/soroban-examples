package main

import (
	"os"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
	gnarktosnarkjs "github.com/mysteryon88/gnark-to-snarkjs"
)

const (
	ProofPath = "../proof.json"
	VKeyPath  = "../verification_key.json"
)

func main() {

	scalarField := ecc.BLS12_381.ScalarField()

	circuit := Circuit{}

	// Compile the circuit into an R1CS
	r1cs, err := frontend.Compile(scalarField, r1cs.NewBuilder, &circuit)
	if err != nil {
		panic(err)
	}

	// Setup the proving and verifying keys
	pk, vk, err := groth16.Setup(r1cs)
	if err != nil {
		panic(err)
	}

	// Set the witness values
	circuit.X = 3
	circuit.Y = 35

	// Create a witness for the circuit
	witnessFull, err := frontend.NewWitness(&circuit, scalarField)
	if err != nil {
		panic(err)
	}

	// Create a witness for the public inputs
	witnessPublic, err := frontend.NewWitness(&circuit, scalarField, frontend.PublicOnly())
	if err != nil {
		panic(err)
	}

	// Prove the circuit
	proof, err := groth16.Prove(r1cs, pk, witnessFull)
	if err != nil {
		panic(err)
	}

	// Verify the proof
	err = groth16.Verify(proof, vk, witnessPublic)
	if err != nil {
		panic(err)
	}

	// Export the proof
	{

		proof_out, err := os.Create(ProofPath)
		if err != nil {
			panic(err)
		}

		defer proof_out.Close()

		err = gnarktosnarkjs.ExportProof(proof, []string{"35"}, proof_out)
		if err != nil {
			panic(err)
		}
	}

	// Export the verification key
	{
		out, err := os.Create(VKeyPath)
		if err != nil {
			panic(err)
		}
		defer out.Close()
		err = gnarktosnarkjs.ExportVerifyingKey(vk, out)
		if err != nil {
			panic(err)
		}
	}

}
