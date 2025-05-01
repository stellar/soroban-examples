default: build

all: test

test: build

build:
	deno check main.ts

install:
	deno install \
		--global \
		--name stellar-sign-auth-ed25519-js \
		--config deno.json \
		--allow-read --no-prompt \
		--force \
		main.ts

fmt:
	deno fmt

clean:
	deno clean
