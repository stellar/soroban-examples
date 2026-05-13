default: build

CONTRACT_DIRS = contracts/bls12_381_verifier contracts/bn254_verifier

all: test

test: build
	cargo test --workspace

build:
	@for dir in $(CONTRACT_DIRS); do $(MAKE) -C $$dir build; done

fmt:
	cargo fmt --all

clean:
	cargo clean
