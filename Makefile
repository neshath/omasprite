APP := omarchy-studio
PREFIX ?= $(HOME)/.local

.PHONY: build release run check test fmt clippy install uninstall
build:
	cargo build --locked
release:
	cargo build --release --locked
run:
	cargo run --locked
check:
	cargo check --locked
test:
	cargo test --locked
fmt:
	cargo fmt --all -- --check
clippy:
	cargo clippy --locked --all-targets --all-features -- -D warnings
install: release
	install -Dm755 target/release/$(APP) $(PREFIX)/bin/$(APP)
	install -Dm644 packaging/omarchy-studio.desktop $(PREFIX)/share/applications/omarchy-studio.desktop
	install -Dm644 assets/omarchy-studio.svg $(PREFIX)/share/icons/hicolor/scalable/apps/omarchy-studio.svg
uninstall:
	rm -f $(PREFIX)/bin/$(APP) $(PREFIX)/share/applications/omarchy-studio.desktop $(PREFIX)/share/icons/hicolor/scalable/apps/omarchy-studio.svg
