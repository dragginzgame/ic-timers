.PHONY: \
	actions-check build bump-x check ci clean clippy docs-check ensure-clean fmt fmt-check help \
	install-hooks major minor msrv package patch pocketic-cohorts pocketic-watchdog publish release-check release-commit \
	pocketic-check provider-check release-major release-minor release-patch release-push release-stage \
	release-tag-check release-verify release-x shell-check test testing-check update-dev version wasm-check

MSRV ?= 1.88.0
VERSION ?=
POCKET_IC_VERSION := 15.0.0
POCKET_IC_BIN_ORIGIN := $(origin POCKET_IC_BIN)
POCKET_IC_BIN ?= $(CURDIR)/target/tools/pocket-ic/$(POCKET_IC_VERSION)/pocket-ic
POCKET_IC_AUTO_INSTALL := $(if $(filter undefined,$(POCKET_IC_BIN_ORIGIN)),1,0)

CI_TARGETS := actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package
RELEASE_TARGETS := pocketic-check ci msrv testing-check pocketic-watchdog pocketic-cohorts

help:
	@echo "Available commands:"
	@echo ""
	@echo "  fmt / fmt-check     Format Rust or verify formatting"
	@echo "  check / clippy      Compile all targets and lint with warnings denied"
	@echo "  docs-check          Build public API docs with warnings denied"
	@echo "  test                Run workspace unit tests"
	@echo "  wasm-check          Compile the library for wasm32-unknown-unknown"
	@echo "  package             Verify the publishable crate package"
	@echo "  pocketic-watchdog   Build and run the focused watchdog canister evidence"
	@echo "  pocketic-cohorts    Build and run comparable timer policy cohorts"
	@echo "  pocketic-check      Install or verify the audited PocketIC evidence binary"
	@echo "  provider-check      Enforce the private ic-cdk-timers provider boundary"
	@echo "  testing-check       Lint every supported nested probe configuration"
	@echo "  release-verify      Run the complete fail-closed release evidence gate"
	@echo "  publish             Publish the clean, tagged release to crates.io"
	@echo "  actions-check       Verify external Actions use full commit SHAs"
	@echo "  shell-check         Check repository shell-script syntax"
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

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked

test:
	cargo test --workspace --all-targets --all-features --locked

wasm-check:
	cargo check --workspace --all-features --locked --target wasm32-unknown-unknown

msrv:
	cargo +$(MSRV) check --workspace --all-targets --all-features --locked

testing-check:
	cargo fmt --manifest-path testing/Cargo.toml --all -- --check
	cargo +$(MSRV) clippy --manifest-path testing/Cargo.toml \
		-p ic-timers-runtime-probe -p ic-timers-pocketic --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy --manifest-path testing/Cargo.toml \
		-p ic-timers-size-probe --no-default-features --features baseline --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy --manifest-path testing/Cargo.toml \
		-p ic-timers-size-probe --no-default-features --features once --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy --manifest-path testing/Cargo.toml \
		-p ic-timers-size-probe --no-default-features --features after-completion --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy --manifest-path testing/Cargo.toml \
		-p ic-timers-size-probe --no-default-features --features watchdog --all-targets --locked -- -D warnings

package:
	cargo package --locked --offline --allow-dirty -p ic-timers

pocketic-check:
	POCKET_IC_BIN="$(POCKET_IC_BIN)" \
		POCKET_IC_AUTO_INSTALL="$(POCKET_IC_AUTO_INSTALL)" \
		bash scripts/ci/check-pocketic.sh

pocketic-watchdog: pocketic-check
	cargo +$(MSRV) build --manifest-path testing/Cargo.toml -p ic-timers-runtime-probe \
		--release --target wasm32-unknown-unknown --locked
	POCKET_IC_BIN="$(POCKET_IC_BIN)" \
		IC_TIMERS_PROBE_WASM="$(CURDIR)/testing/target/wasm32-unknown-unknown/release/ic_timers_runtime_probe.wasm" \
		cargo +$(MSRV) test --manifest-path testing/Cargo.toml -p ic-timers-pocketic --locked tests::

pocketic-cohorts: pocketic-check
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-baseline" \
		cargo +$(MSRV) build --manifest-path testing/Cargo.toml -p ic-timers-size-probe \
		--release --target wasm32-unknown-unknown --locked
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-once" \
		cargo +$(MSRV) build --manifest-path testing/Cargo.toml -p ic-timers-size-probe \
		--release --target wasm32-unknown-unknown --locked --no-default-features --features once
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-after-completion" \
		cargo +$(MSRV) build --manifest-path testing/Cargo.toml -p ic-timers-size-probe \
		--release --target wasm32-unknown-unknown --locked --no-default-features --features after-completion
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-watchdog" \
		cargo +$(MSRV) build --manifest-path testing/Cargo.toml -p ic-timers-size-probe \
		--release --target wasm32-unknown-unknown --locked --no-default-features --features watchdog
	POCKET_IC_BIN="$(POCKET_IC_BIN)" \
		IC_TIMERS_COHORT_ROOT="$(CURDIR)/testing/target" \
		cargo +$(MSRV) test --manifest-path testing/Cargo.toml -p ic-timers-pocketic --locked \
			comparable_policy_cohorts_report_size_and_instruction_subjects -- --nocapture

actions-check:
	bash scripts/ci/check-github-actions-pinned.sh

shell-check:
	bash -n .githooks/pre-commit scripts/ci/*.sh scripts/dev/*.sh scripts/release/*.sh

release-check:
	bash scripts/release/test-finalize-changelog.sh
	bash scripts/release/test-finalize-release-truth.sh
	bash scripts/release/test-release-prose-warning.sh
	bash scripts/release/test-release-gate.sh
	bash scripts/release/check-release-truth.sh

provider-check:
	bash scripts/ci/check-provider-boundary.sh

ci:
	+@set -e; for target in $(CI_TARGETS); do \
		$(MAKE) --no-print-directory "$$target"; \
	done

release-verify:
	+@set -e; for target in $(RELEASE_TARGETS); do \
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
	@version="$$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"; \
		git add Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md README.md \
			crates/ic-timers/Cargo.toml docs/status/current.md docs/adoption/canic.md \
			"docs/changelog/$${version}.md"

release-commit:
	@bash scripts/release/commit-release.sh

release-tag-check:
	@bash scripts/release/check-tag-at-head.sh

release-push: ensure-clean release-tag-check
	git push --follow-tags

publish: ensure-clean release-tag-check package
	cargo publish --locked --registry crates-io -p ic-timers
