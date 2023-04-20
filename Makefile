MAKEFILES = $(shell find . -maxdepth 3 -type f -name Makefile)
SUBDIRS   = $(filter-out ./,$(dir $(MAKEFILES)))
ROOT_DIR = $(shell pwd)

default: build

all: build test

build:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		make build || break; \
		cd $(ROOT_DIR); \
	done

test: build
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		make test || break; \
		cd $(ROOT_DIR); \
	done

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean