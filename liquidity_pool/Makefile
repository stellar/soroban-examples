default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../token || break;
	cargo rustc --crate-type cdylib --target wasm32-unknown-unknown --release
	@ls -l target/wasm32-unknown-unknown/release/*.wasm
fmt:
	cargo fmt --all

clean:
	cargo clean
