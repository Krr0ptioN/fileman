PREFIX ?= $(HOME)/.local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share
APPDIR = $(DATADIR)/applications
ICONDIR = $(DATADIR)/icons/hicolor/scalable/apps

.PHONY: build install uninstall

build:
	cargo build --release

install: build
	install -Dm755 target/release/stiff $(BINDIR)/stiff
	install -Dm644 etc/stiff.svg $(ICONDIR)/stiff.svg
	sed 's|Exec=stiff|Exec=$(BINDIR)/stiff|' etc/stiff.desktop \
		| install -Dm644 /dev/stdin $(APPDIR)/stiff.desktop
	@echo "Installed to $(PREFIX). Make sure $(BINDIR) is in your PATH."

uninstall:
	rm -f $(BINDIR)/stiff
	rm -f $(APPDIR)/stiff.desktop
	rm -f $(ICONDIR)/stiff.svg
