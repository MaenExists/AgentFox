.PHONY: all build install clean

PREFIX ?= $(HOME)/.local
BINDIR = $(PREFIX)/bin

all: build

build:
	cargo build --release

install: build
	mkdir -p $(BINDIR)
	rm -f $(BINDIR)/afox $(BINDIR)/afoxd
	cp target/release/afox $(BINDIR)/afox
	cp target/release/afoxd $(BINDIR)/afoxd
	@echo "AgentFox installed to $(BINDIR)"
	@echo "Ensure $(BINDIR) is in your PATH."

clean:
	cargo clean
