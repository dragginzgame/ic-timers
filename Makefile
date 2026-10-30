.PHONY: \
	actions-check build bump-x check ci clean clippy docs-check ensure-clean fetch help \
	install-hooks install-testkit-server major minor msrv package patch pocketic-cohorts pocketic-watchdog publish release-check \
	pocketic-check provider-check \
	release-impact release-tag-check release-verify repository-check shell-check test testing-check update-dev \
	version wasm-check

# Deferred until a compiler-selecting target needs it. Read-only checks require
# prepared host parsers; explicit MSRV command/environment overrides still win.
MSRV ?= $(or $(shell PATH="$(CURDIR)/.tools/host/bin:$$PATH" bash "$(CURDIR)/scripts/ci/read-cargo-workspace-rust-version.sh" "$(CURDIR)/Cargo.toml"),$(error Cannot read workspace rust-version; run make install-host-tools))
VERSION ?=
LOCAL_TOOL_INSTALL_TARGETS := install-release-editor install-testkit-server
LOCAL_TOOL_CHECK_TARGETS := release-editor-check pocketic-check
override SHARED_TOOLING_ROOT := $(CURDIR)
# Keep one consumer-owned declaration for Make and standalone adapters.
override RELEASE_DELIVERY_POLICY := $(CURDIR)/ci/release-delivery.txt
RELEASE_DELIVERY ?= direct
export RELEASE_DELIVERY
include make/consumer.mk

CI_TARGETS := actions-check shell-check provider-check fmt-check clippy release-check docs-check test wasm-check package
RELEASE_TARGETS := fetch install-release-editor install-testkit-server pocketic-check ci msrv testing-check pocketic-watchdog pocketic-cohorts
REPOSITORY_TARGETS := actions-check shell-check release-check provider-check fmt-check

help:
	@echo "Available commands:"
	@echo ""
	@echo "  fmt / fmt-check     Sort manifests and format or check Rust for all workspace members"
	@echo "  check / clippy      Compile library targets and lint with warnings denied"
	@echo "  docs-check          Build public API docs with warnings denied"
	@echo "  test                Run library unit and API documentation tests"
	@echo "  wasm-check          Compile the library for wasm32-unknown-unknown"
	@echo "  package             Verify the publishable crate package"
	@echo "  fetch               Download locked dependencies for the root workspace"
	@echo "  pocketic-watchdog   Build and run the focused watchdog canister evidence"
	@echo "  pocketic-cohorts    Build and run comparable timer policy cohorts"
	@echo "  pocketic-check      Check the Testkit-selected PocketIC server offline"
	@echo "  provider-check      Enforce private IC dispatch and executor ownership"
	@echo "  testing-check       Check formatting and lint supported unpublished probes"
	@echo "  release-verify      Run the complete fail-closed release evidence gate"
	@echo "  release-impact      Classify changes since the current version tag"
	@echo "  repository-check    Validate a non-published repository-only update"
	@echo "  publish             Publish the clean, tagged release to crates.io"
	@echo "  actions-check       Check parsed Actions and dependency pin declarations"
	@echo "  install-host-tools  Install pinned jq/yq/ripgrep/cloc host tools"
	@echo "  host-tools-check    Verify the complete host bundle offline"
	@echo "  install-testkit-server  Prepare the locked Testkit CLI and its admitted server"
	@echo "  install-tools / tools-check  Prepare or verify host/Cargo tools, editor and Testkit"
	@echo "  shared-tooling-check  Verify the reviewed root snapshot offline"
	@echo "  install-hooks       Activate the reviewed repository-local formatting hook"
	@echo "  cloc                Report root-workspace Rust LOC and test counts"
	@echo "  shell-check         Check repository shell-script syntax"
	@echo "  msrv                Check with the minimum supported Rust version"
	@echo "  ci                  Run the local CI gate"
	@echo "  update-dev          Install selected Rust, host/Cargo tools, Testkit, and hook"
	@echo "  patch|minor|major   Prepare version metadata without deployment tests"
	@echo "  release-{patch,minor,major}  Commit, tag, and push a SemVer release"
	@echo "  release-resume VERSION=x.y.z Resume exact saved shared release intent"

version:
	@bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml

# Prepare every target's locked sources before offline metadata validation.
fetch:
	+cargo fetch --manifest-path Cargo.toml --locked

check:
	+cargo check -p ic-timers --all-targets --all-features --locked

clippy:
	+cargo clippy -p ic-timers --all-targets --all-features --locked -- -D warnings

docs-check:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p ic-timers --all-features --no-deps --locked

test:
	+cargo test -p ic-timers --all-targets --all-features --locked
	+cargo test -p ic-timers --doc --all-features --locked
	+cargo test -p ic-timers-test-protocol --lib --locked
	+cargo test -p ic-timers-runtime-probe --lib --locked

wasm-check:
	+cargo check -p ic-timers --all-features --locked --target wasm32-unknown-unknown

msrv:
	+cargo +$(MSRV) check -p ic-timers --all-targets --all-features --locked
	+cargo +$(MSRV) check -p ic-timers --all-features --locked --target wasm32-unknown-unknown
	+cargo +$(MSRV) test -p ic-timers --doc --all-features --locked

testing-check:
	+$(MAKE) --no-print-directory fmt-check
	+cargo +$(MSRV) clippy \
		-p ic-timers-runtime-probe -p ic-timers-pocketic -p ic-timers-test-protocol --all-targets --locked -- -D warnings
	+cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features baseline --all-targets --locked -- -D warnings
	+cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features once --all-targets --locked -- -D warnings
	+cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features after-completion --all-targets --locked -- -D warnings
	+cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features watchdog --all-targets --locked -- -D warnings

package:
	+cargo package --locked --offline --allow-dirty -p ic-timers
	@version="$$(bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)"; \
		perl scripts/ci/check-license.pl --kind mit --file crates/ic-timers/LICENSE \
		--archive "target/package/ic-timers-$$version.crate" --member "ic-timers-$$version/LICENSE"

install-testkit-server:
	+bash scripts/dev/testkit-server.sh setup

.PHONY: install-release-editor release-editor-check
install-release-editor:
	+bash scripts/dev/release-editor.sh setup

release-editor-check:
	+bash scripts/dev/release-editor.sh check

pocketic-check:
	+@bash scripts/dev/testkit-server.sh check

pocketic-watchdog: pocketic-check
	+@set -e; server="$$(bash scripts/dev/testkit-server.sh check)"; \
		POCKET_IC_BIN="$$server" \
		RUSTUP_TOOLCHAIN="$(MSRV)" cargo run -p ic-timers-pocketic --locked -- watchdog

pocketic-cohorts: pocketic-check
	+@set -e; server="$$(bash scripts/dev/testkit-server.sh check)"; \
		POCKET_IC_BIN="$$server" \
		RUSTUP_TOOLCHAIN="$(MSRV)" cargo run -p ic-timers-pocketic --locked -- cohorts

# Keep the established gate entry point; the shared owner also checks Cargo
# declarations and the tracked root lockfile for all maintained members.
actions-check: host-tools-check
	perl scripts/ci/check-no-symlinks.pl --consumer "$(CURDIR)"
	YQ="$(CURDIR)/.tools/host/bin/yq" bash scripts/ci/check-dependency-pins.sh \
		--consumer "$(CURDIR)" --cargo-inheritance
	bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$(CURDIR)" --external-ic
	bash scripts/ci/check-library-dependency-boundary.sh

shell-check:
	@set -e; for script in .githooks/pre-commit scripts/ci/*.sh scripts/dev/*.sh scripts/release/*.sh; do \
		bash -n "$$script"; \
	done

# Shared owns generic mechanics and their regression matrices.
# Retain Timers' actual wiring, release/recovery and host/product evidence.
release-check:
	bash scripts/ci/test-fixture-completion.sh
	+$(MAKE) --no-print-directory shared-tooling-check
	bash scripts/ci/test-rust-preparation.sh
	bash scripts/ci/test-failure-evidence.sh
	bash scripts/ci/check-release-commands.sh "$(CURDIR)" \
		ci/tool-versions.env ci/release-delivery.txt make/consumer.mk make/tools.mk make/rust-format.mk make/release.mk \
		scripts/ci/check-release-delivery.sh \
		make/execution.mk scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh
	bash scripts/release/test-committed-release.sh
	bash scripts/release/test-release-index.sh
	bash scripts/release/test-release-impact.sh
	RELEASE_COMMIT='' bash scripts/ci/cargo-release-metadata.sh check-graph ic-timers
	bash scripts/release/test-release-gate.sh
	bash scripts/release/test-version-preparation.sh
	bash scripts/release/test-tag-at-head.sh
	bash scripts/ci/test-testkit-server.sh
	bash scripts/ci/test-git-hook.sh
	bash scripts/ci/test-repository-checks.sh
	bash scripts/ci/test-library-dependency-boundary.sh

provider-check:
	bash scripts/ci/check-provider-boundary.sh

ci:
	+@set -e; for target in $(CI_TARGETS); do \
		$(MAKE) --no-print-directory "$$target"; \
	done

release-verify:
	+@set -e; state_root="$$(git rev-parse --git-path release-state)"; \
		VALIDATION_REPOSITORY_ROOT="$(CURDIR)" \
		VALIDATION_FAILURE_LOG_DIR="$$state_root/validation-failures" \
		bash scripts/ci/run-validation-targets.sh --fail-fast $(RELEASE_TARGETS)

release-impact:
	@set -e; impact="$$(bash scripts/release/classify-release-impact.sh)"; \
		echo "Release impact: $$impact"

repository-check:
	+@set -e; impact="$$(bash scripts/release/classify-release-impact.sh)"; \
		if [ "$$impact" = "crate" ]; then \
			echo "error: crate-impacting changes require the release validation path" >&2; \
			exit 1; \
		fi; \
		echo "Repository impact: $$impact"; \
		for target in $(REPOSITORY_TARGETS); do \
			$(MAKE) --no-print-directory "$$target"; \
		done

build:
	+cargo build -p ic-timers --all-targets --all-features --locked

clean:
	+cargo clean

update-dev:
	@bash scripts/dev/update-dev.sh $(if $(filter file,$(origin MSRV)),,"$(MSRV)")

ensure-clean:
	@if ! git rev-parse --verify HEAD >/dev/null 2>&1; then \
		echo "error: repository has no commit yet" >&2; exit 1; \
	fi
	@bash scripts/ci/check-release-source.sh

patch:
	bash scripts/release/bump-version.sh patch

minor:
	bash scripts/release/bump-version.sh minor

major:
	bash scripts/release/bump-version.sh major

bump-x:
	@if [ -z "$(VERSION)" ]; then echo "error: VERSION=x.y.z is required" >&2; exit 2; fi
	bash scripts/release/bump-version.sh "$(VERSION)"

release-tag-check:
	@set -e; version="$$(bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)"; \
		commit="$$(git rev-parse HEAD)"; \
		bash scripts/ci/check-release-tag.sh "$$commit" "$$version"

# Sequential recursive calls retain admission and lint ordering under -j.
publish:
	+$(MAKE) --no-print-directory ensure-clean
	+$(MAKE) --no-print-directory release-tag-check
	+$(MAKE) --no-print-directory clippy
	+$(MAKE) --no-print-directory package
	+cargo publish --locked --registry crates-io -p ic-timers

# These consumer entrypoints use the reviewed root snapshot. Preserve their
# former repository-local routing even if a caller exports another snapshot root.
format-tools-check fmt fmt-check release-patch release-minor release-major release-resume: override SHARED_TOOLING_ROOT = $(CURDIR)
format-tools-check fmt fmt-check: override HOST_TOOL_VERSIONS = $(CURDIR)/ci/tool-versions.env
format-tools-check fmt fmt-check: override RUST_TOOL_VERSIONS = $(CURDIR)/ci/tool-versions.env
.PHONY: release-version release-preflight release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check

release-version:
	@bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml
release-preflight:
	@bash scripts/release/adapter.sh preflight
release-prepare-version:
	@IC_TIMERS_RELEASE_DATE="$(RELEASE_DATE)" bash scripts/release/bump-version.sh "$(RELEASE_VERSION)"
release-prepared-check:
	@bash scripts/release/adapter.sh check
release-commit-check:
	@bash scripts/release/adapter.sh commit-check
release-committed-check:
	@bash scripts/release/adapter.sh check-committed
release-files:
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md
release-tagged-check release-push-check:
	@bash scripts/release/adapter.sh check-committed
	@bash scripts/ci/check-release-tag.sh "$(RELEASE_COMMIT)" "$(RELEASE_VERSION)"
