PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share

.PHONY: install uninstall
install:
	mkdir -p "$(BINDIR)" "$(DATADIR)/applications" "$(DATADIR)/icons/hicolor/scalable/apps"
	cargo install --path . --root /tmp/omasprite-install --locked
	cp /tmp/omasprite-install/bin/omarchy-studio "$(BINDIR)/omarchy-studio"
	cp packaging/omarchy-studio.desktop "$(DATADIR)/applications/omarchy-studio.desktop"

uninstall:
	rm -f "$(BINDIR)/omarchy-studio" "$(DATADIR)/applications/omarchy-studio.desktop"
