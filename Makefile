$(MAKE)FILES = $(shell find . -maxdepth 3 -type f -name Makefile)
SUBDIRS   = $(filter-out ./,$(dir $($(MAKE)FILES)))
ROOT_DIR = $(shell pwd)

default: build

all: build test

build:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		$(MAKE) build || break; \
		cd $(ROOT_DIR); \
	done

test: build
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		$(MAKE) test || break; \
		cd $(ROOT_DIR); \
	done

watch:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		$(MAKE) watch || break; \
		cd $(ROOT_DIR); \
	done

fmt:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		$(MAKE) fmt || break; \
		cd $(ROOT_DIR); \
	done

clean:
	@for dir in $(SUBDIRS) ; do \
		cd $$dir; \
		$(MAKE) clean || break; \
		cd $(ROOT_DIR); \
	done