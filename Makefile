.PHONY: \
	build bump-x check ci clean clippy ensure-clean fmt fmt-check help \
	install-hooks major minor msrv package patch release-commit \
	release-major release-minor release-patch release-push release-stage \
	release-tag-check release-x test update-dev version wasm-check

MSRV ?= 1.91.0
VERSION ?=

CI_TARGETS := fmt-check check clippy test wasm-check package

help:
	@echo "Available commands:"
	@echo ""
	@echo "  fmt / fmt-check     Format Rust or verify formatting"
	@echo "  check / clippy      Compile all targets and lint with warnings denied"
	@echo "  test                Run workspace unit tests"
	@echo "  wasm-check          Compile the library for wasm32-unknown-unknown"
	@echo "  package             Verify the publishable crate package"
	@echo "  msrv                Check with the minimum supported Rust version"
	@echo "  ci                  Run the local CI gate"
	@echo "  update-dev          Install the pinned Rust tools, Wasm target, and hook"
	@echo "  patch|minor|major   Validate and update version files for review"
	@echo "  release-{patch,minor,major}  Commit, tag, and push a SemVer release"
	@echo "  release-x VERSION=x.y.z      Commit, tag, and push an exact release"

version:
	@sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-targets --all-features --locked

clippy:
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

test:
	cargo test --workspace --all-targets --all-features --locked

wasm-check:
	cargo check --workspace --all-features --locked --target wasm32-unknown-unknown

msrv:
	cargo +$(MSRV) check --workspace --all-targets --all-features --locked

package:
	cargo package --locked --offline --allow-dirty -p ic-timers

ci:
	+@set -e; for target in $(CI_TARGETS); do \
		$(MAKE) --no-print-directory "$$target"; \
	done

build:
	cargo build --workspace --all-targets --all-features --locked

clean:
	cargo clean

install-hooks:
	bash scripts/dev/install-git-hooks.sh

update-dev:
	bash scripts/dev/update-dev.sh

ensure-clean:
	@bash scripts/ci/ensure-clean.sh

patch:
	bash scripts/release/bump-version.sh patch

minor:
	bash scripts/release/bump-version.sh minor

major:
	bash scripts/release/bump-version.sh major

bump-x:
	@if [ -z "$(VERSION)" ]; then echo "error: VERSION=x.y.z is required" >&2; exit 2; fi
	bash scripts/release/bump-version.sh "$(VERSION)"

release-patch:
	+$(MAKE) --no-print-directory patch
	+$(MAKE) --no-print-directory release-stage
	+$(MAKE) --no-print-directory release-commit
	+$(MAKE) --no-print-directory release-push

release-minor:
	+$(MAKE) --no-print-directory minor
	+$(MAKE) --no-print-directory release-stage
	+$(MAKE) --no-print-directory release-commit
	+$(MAKE) --no-print-directory release-push

release-major:
	+$(MAKE) --no-print-directory major
	+$(MAKE) --no-print-directory release-stage
	+$(MAKE) --no-print-directory release-commit
	+$(MAKE) --no-print-directory release-push

release-x:
	+$(MAKE) --no-print-directory bump-x VERSION="$(VERSION)"
	+$(MAKE) --no-print-directory release-stage
	+$(MAKE) --no-print-directory release-commit
	+$(MAKE) --no-print-directory release-push

release-stage:
	git add Cargo.toml Cargo.lock CHANGELOG.md README.md crates/ic-timers/Cargo.toml

release-commit:
	@bash scripts/release/commit-release.sh

release-tag-check:
	@bash scripts/release/check-tag-at-head.sh

release-push: ensure-clean release-tag-check
	git push --follow-tags
