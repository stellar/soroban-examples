default: build

all: test

test: build

build:
	cargo check

install:
	cargo install --locked --path .

fmt:
	cargo fmt --all

clean:
	cargo clean
