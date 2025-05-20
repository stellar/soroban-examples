default: build

all: test

test: build
	cargo test

build:
	cargo rustc --target wasm32-unknown-unknown --release --crate-type=cdylib
	mkdir -p opt/
	for i in target/wasm32-unknown-unknown/release/*.wasm ; do \
		wasm-opt -Oz "$$i" -o "opt/$$(basename $$i)"; \
		ls -l "opt/$$(basename $$i)"; \
	done
fmt:
	cargo fmt --all

clean:
	cargo clean
