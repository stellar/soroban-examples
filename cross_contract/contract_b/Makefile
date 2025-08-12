default: build

all: test

test: build
	cargo test

build:
	$(MAKE) -C ../contract_a || break; 
	stellar contract build
	@RUST_VERSION=$$(rustc --version | cut -d' ' -f2); \
	if [ "$$(printf '%s\n%s\n' $$RUST_VERSION 1.81.0 | sort -V | head -n1)" = "1.81.0" ]; then \
		ls -l target/wasm32v1-none/release/*.wasm; \
	else \
		ls -l target/wasm32-unknown-unknown/release/*.wasm; \
	fi

fmt:
	cargo fmt --all

clean:
	cargo clean
