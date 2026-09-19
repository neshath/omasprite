PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share

.PHONY: install uninstall
PROJECT ?= examples/snow-courier
EXPORT_DIR ?= dist/omasprite-game
install:
	mkdir -p "$(BINDIR)" "$(DATADIR)/applications" "$(DATADIR)/icons/hicolor/scalable/apps"
	cargo install --path . --root /tmp/omasprite-install --locked
	cp /tmp/omasprite-install/bin/omarchy-studio "$(BINDIR)/omarchy-studio"
	cp packaging/omarchy-studio.desktop "$(DATADIR)/applications/omarchy-studio.desktop"

uninstall:
	rm -f "$(BINDIR)/omarchy-studio" "$(DATADIR)/applications/omarchy-studio.desktop"

export:
	cargo build --release --locked
	mkdir -p "$(EXPORT_DIR)"
	cp target/release/omarchy-studio "$(EXPORT_DIR)/omasprite-player"
	cp -R "$(PROJECT)" "$(EXPORT_DIR)/game"
	printf '%s\n' '#!/bin/sh' 'exec "$$(dirname "$$0")/omasprite-player" --play "$$(dirname "$$0")/game"' > "$(EXPORT_DIR)/play.sh"
	chmod +x "$(EXPORT_DIR)/play.sh"
