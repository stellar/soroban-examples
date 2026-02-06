$MAKEFILES = $(shell find . -maxdepth 3 -type f -name Makefile)
SUBDIRS   = $(filter-out ./,$(dir $($MAKEFILES)))

default: build

all: test

build:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir build || exit 1; \
	done

test: build
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir test || exit 1; \
	done

fmt:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir fmt || exit 1; \
	done

clean:
	@for dir in $(SUBDIRS) ; do \
		$(MAKE) -C $$dir clean || exit 1; \
	done