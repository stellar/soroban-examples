default: build

all: test

test: build
	cargo test
	for target in $$(cargo fuzz list); do \
		cargo +nightly fuzz run --sanitizer=thread $$target -- -runs=0; \
	done

build:
	stellar contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
