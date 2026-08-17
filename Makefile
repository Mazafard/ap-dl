# APDL - Makefile
VERSION ?= 0.1.0
TAG_NAME = v$(VERSION)

.PHONY: all build release tag check test run clean help icons app bundle dmg notarize

all: check build

help:
	@echo "APDL Automation Commands:"
	@echo "  make check             - Run cargo check"
	@echo "  make test              - Run all unit and API tests"
	@echo "  make run               - Run application in release mode"
	@echo "  make watch             - Run with live file watcher"
	@echo "  make build             - Build optimized local release binary"
	@echo "  make icons             - Generate multi-resolution AppIcon.icns"
	@echo "  make app / bundle      - Create standalone APDL.app bundle"
	@echo "  make dmg               - Create drag-and-drop APDL.dmg installer"
	@echo "  make notarize          - Notarize and staple APDL.dmg with Apple"
	@echo "  make tag VERSION=0.1.0 - Create and push git tag to trigger GitHub Release"
	@echo "  make release VERSION=0.1.0 - Bump version, commit, tag, and publish"
	@echo "  make clean             - Clean build targets and dist artifacts"

check:
	cargo check

test:
	cargo test -- --nocapture

run:
	cargo run --release

watch:
	cargo watch -w src -w ui -c -x run

build:
	cargo build --release

icons:
	./scripts/generate_icons.sh

# Package macOS APDL.app bundle
app: bundle

bundle:
	./scripts/bundle_macos.sh $(VERSION)

# Create compressed drag-and-drop DMG installer
dmg:
	./scripts/create_dmg.sh $(VERSION)

# Notarize DMG with Apple
notarize:
	./scripts/notarize_dmg.sh dist/APDL.dmg

# Tag current commit with version and push to remote
tag:
	@echo "Tagging version $(TAG_NAME)..."
	@git tag -a $(TAG_NAME) -m "Release $(TAG_NAME)"
	@git push origin $(TAG_NAME)
	@echo "Tag $(TAG_NAME) pushed successfully! GitHub Actions will now build and publish the release."

# Bump version in Cargo.toml, commit, and push tag
release:
	@echo "Preparing release $(TAG_NAME)..."
	@sed -i '' 's/^version = .*/version = "$(VERSION)"/' Cargo.toml 2>/dev/null || sed -i 's/^version = .*/version = "$(VERSION)"/' Cargo.toml
	@git add Cargo.toml
	@git commit -m "chore: bump version to $(TAG_NAME)" || true
	@git tag -a $(TAG_NAME) -m "Release $(TAG_NAME)"
	@git push origin HEAD
	@git push origin $(TAG_NAME)
	@echo "Release $(TAG_NAME) triggered on GitHub Actions!"

clean:
	cargo clean
	rm -rf dist/ *.mp4 *.part *.tar.gz *.zip *.dmg
