# APDL - Makefile
VERSION ?= 0.0.1
TAG_NAME = v$(VERSION)

.PHONY: all build release tag check test run clean help

all: check build

help:
	@echo "APDL Automation Commands:"
	@echo "  make check             - Run cargo check"
	@echo "  make test              - Run all unit and API tests"
	@echo "  make run               - Run application in release mode"
	@echo "  make watch             - Run with live file watcher"
	@echo "  make build             - Build optimized local release binary"
	@echo "  make tag VERSION=0.0.1 - Create and push git tag to trigger GitHub Release"
	@echo "  make release VERSION=0.0.1 - Update version, commit, tag, and trigger CI/CD"
	@echo "  make clean             - Clean build targets and temporary parts"

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

# Package macOS APDL.app bundle with native icon
app: build
	@echo "Creating APDL.app bundle..."
	@mkdir -p dist/APDL.app/Contents/MacOS
	@mkdir -p dist/APDL.app/Contents/Resources
	@cp target/release/ap-dl dist/APDL.app/Contents/MacOS/APDL
	@cp assets/icon.icns dist/APDL.app/Contents/Resources/AppIcon.icns
	@cp assets/icon.png dist/APDL.app/Contents/Resources/icon.png
	@echo '<?xml version="1.0" encoding="UTF-8"?>' > dist/APDL.app/Contents/Info.plist
	@echo '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' >> dist/APDL.app/Contents/Info.plist
	@echo '<plist version="1.0"><dict>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundleExecutable</key><string>APDL</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundleIconFile</key><string>AppIcon</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundleIdentifier</key><string>com.apdl.app</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundleName</key><string>APDL</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundlePackageType</key><string>APPL</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>CFBundleShortVersionString</key><string>$(VERSION)</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>LSMinimumSystemVersion</key><string>10.13</string>' >> dist/APDL.app/Contents/Info.plist
	@echo '  <key>NSHighResolutionCapable</key><true/>' >> dist/APDL.app/Contents/Info.plist
	@echo '</dict></plist>' >> dist/APDL.app/Contents/Info.plist
	@echo "APDL.app created at dist/APDL.app! You can launch it with: open dist/APDL.app"

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
	rm -f *.mp4 *.part *.tar.gz *.zip
