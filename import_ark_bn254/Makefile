default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	mkdir -p opt/
	for i in target/wasm32v1-none/release/*.wasm ; do \
		stellar contract optimize --wasm "$$i" --wasm-out "opt/$$(basename $$i)"; \
		ls -l "opt/$$(basename $$i)"; \
	done
fmt:
	cargo fmt --all

clean:
	cargo clean
