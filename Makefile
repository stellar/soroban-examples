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
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		make watch || break; \
		cd $(ROOT_DIR); \
	done

fmt:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		make fmt || break; \
		cd $(ROOT_DIR); \
	done

clean:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		make clean || break; \
		cd $(ROOT_DIR); \
	done