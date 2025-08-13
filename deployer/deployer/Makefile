default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../contract || break;
	stellar contract build
	@ls -l target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean
