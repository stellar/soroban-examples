default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../new_contract || break;
	cargo build --target wasm32-unknown-unknown --release 
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
