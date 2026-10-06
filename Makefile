.PHONY: \
	actions-check build bump-x check ci clean clippy docs-check ensure-clean fetch fmt fmt-check format-tools-check help \
	install-hooks install-host-tools host-tools-check install-tools tools-check install-ic-tools ic-tools-check major minor msrv package patch pocketic-cohorts pocketic-watchdog publish release-check release-commit \
	pocketic-check provider-check release-major release-minor release-patch release-push release-stage \
	release-impact release-tag-check release-verify release-x repository-check shell-check test testing-check update-dev \
	version wasm-check

MSRV ?= 1.88.0
VERSION ?=
POCKET_IC_VERSION := 16.0.0
POCKET_IC_BIN_ORIGIN := $(origin POCKET_IC_BIN)
POCKET_IC_BIN ?= $(CURDIR)/target/tools/pocket-ic/$(POCKET_IC_VERSION)/pocket-ic
POCKET_IC_AUTO_INSTALL := $(if $(filter undefined,$(POCKET_IC_BIN_ORIGIN)),1,0)
export PATH := $(CURDIR)/.tools/host/bin:$(CURDIR)/.tools/ic/bin:$(PATH)

CI_TARGETS := actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package
RELEASE_TARGETS := fetch pocketic-check ci msrv testing-check pocketic-watchdog pocketic-cohorts
REPOSITORY_TARGETS := actions-check shell-check release-check provider-check fmt-check

help:
	@echo "Available commands:"
	@echo ""
	@echo "  fmt / fmt-check     Sort manifests and format or check Rust in both workspaces"
	@echo "  check / clippy      Compile all targets and lint with warnings denied"
	@echo "  docs-check          Build public API docs with warnings denied"
	@echo "  test                Run workspace unit and API documentation tests"
	@echo "  wasm-check          Compile the library for wasm32-unknown-unknown"
	@echo "  package             Verify the publishable crate package"
	@echo "  fetch               Download locked dependencies for both workspaces"
	@echo "  pocketic-watchdog   Build and run the focused watchdog canister evidence"
	@echo "  pocketic-cohorts    Build and run comparable timer policy cohorts"
	@echo "  pocketic-check      Install or verify the audited PocketIC evidence binary"
	@echo "  provider-check      Enforce the private ic-cdk-timers provider boundary"
	@echo "  testing-check       Check formatting and lint supported nested probes"
	@echo "  release-verify      Run the complete fail-closed release evidence gate"
	@echo "  release-impact      Classify changes since the current version tag"
	@echo "  repository-check    Validate a non-published repository-only update"
	@echo "  publish             Publish the clean, tagged release to crates.io"
	@echo "  actions-check       Check parsed Actions and dependency pin declarations"
	@echo "  install-host-tools  Explicitly install the reviewed jq/yq parser pair"
	@echo "  host-tools-check    Verify the installed parser pair offline"
	@echo "  install-ic-tools    Explicitly install the reviewed six-tool IC bundle"
	@echo "  ic-tools-check      Verify the installed IC bundle offline"
	@echo "  install-tools / tools-check  Prepare or verify both tool bundles"
	@echo "  shell-check         Check repository shell-script syntax"
	@echo "  msrv                Check with the minimum supported Rust version"
	@echo "  ci                  Run the local CI gate"
	@echo "  update-dev          Install pinned Rust/host/IC tools, Wasm target, and hook"
	@echo "  patch|minor|major   Prepare version metadata without deployment tests"
	@echo "  release-{patch,minor,major}  Commit, tag, and push a SemVer release"
	@echo "  release-x VERSION=x.y.z      Commit, tag, and push an exact release"

version:
	@bash scripts/release/workspace-version.sh

# Prepare every target's locked sources before offline metadata validation.
fetch:
	cargo fetch --manifest-path Cargo.toml --locked
	cargo fetch --manifest-path testing/Cargo.toml --locked

format-tools-check:
	@set -e; . ./tool-versions.env; \
		bash .shared-tooling/helpers/scripts/ci/check-format-tools.sh "$$IC_TIMERS_CARGO_SORT_VERSION"

fmt: format-tools-check
	cargo sort --workspace
	cargo sort --workspace testing
	cargo fmt --all
	cargo fmt --manifest-path testing/Cargo.toml --all

fmt-check: format-tools-check
	cargo sort --workspace --check
	cargo sort --workspace --check testing
	cargo fmt --all -- --check
	cargo fmt --manifest-path testing/Cargo.toml --all -- --check

check:
	cargo check --workspace --all-targets --all-features --locked

clippy:
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked

test:
	cargo test --workspace --all-targets --all-features --locked
	cargo test --workspace --doc --all-features --locked

wasm-check:
	cargo check --workspace --all-features --locked --target wasm32-unknown-unknown

msrv:
	cargo +$(MSRV) check --workspace --all-targets --all-features --locked
	cargo +$(MSRV) test --workspace --doc --all-features --locked

testing-check:
	+$(MAKE) --no-print-directory fmt-check
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

install-host-tools:
	bash scripts/dev/install-host-tools.sh

host-tools-check:
	bash scripts/dev/install-host-tools.sh --check

install-ic-tools:
	bash .shared-tooling/helpers/scripts/dev/install-ic-tools.sh --consumer "$(CURDIR)"

ic-tools-check:
	bash .shared-tooling/helpers/scripts/dev/install-ic-tools.sh --consumer "$(CURDIR)" --check

install-tools:
	bash scripts/dev/install-host-tools.sh
	bash .shared-tooling/helpers/scripts/dev/install-ic-tools.sh --consumer "$(CURDIR)"

tools-check:
	bash scripts/dev/install-host-tools.sh --check
	bash .shared-tooling/helpers/scripts/dev/install-ic-tools.sh --consumer "$(CURDIR)" --check

# Keep the established gate entry point; the shared owner also checks Cargo
# declarations and the tracked lockfile of each independent workspace.
actions-check:
	bash scripts/dev/install-host-tools.sh --check
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/check-dependency-pins.sh \
		--consumer "$(CURDIR)" --cargo-inheritance

shell-check:
	@set -e; for script in .githooks/pre-commit scripts/ci/*.sh scripts/dev/*.sh scripts/release/*.sh \
		.shared-tooling/helpers/scripts/ci/*.sh .shared-tooling/helpers/scripts/dev/*.sh; do \
		bash -n "$$script"; \
	done

release-check:
	bash scripts/ci/verify-shared-tooling-snapshot.sh
	bash scripts/ci/verify-shared-tooling-snapshot.sh --manifest .shared-tooling-audits.snapshot
	bash .shared-tooling/helpers/scripts/ci/verify-shared-tooling-snapshot.sh \
		--consumer "$(CURDIR)/.shared-tooling/helpers"
	bash scripts/ci/test-shared-snapshots.sh
	bash scripts/ci/test-host-tools.sh
	bash .shared-tooling/helpers/scripts/ci/test-format-tools.sh
	bash .shared-tooling/helpers/scripts/ci/test-ic-tools.sh
	bash .shared-tooling/helpers/scripts/ci/test-evidence-checksums.sh
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/test-dependency-pins.sh
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/test-cargo-metadata.sh
	perl .shared-tooling/helpers/scripts/ci/test-local-lock-versions.pl
	bash scripts/ci/test-release-runner.sh
	bash .shared-tooling/helpers/scripts/ci/check-release-commands.sh "$(CURDIR)" tool-versions.env
	bash scripts/release/test-committed-release.sh
	bash scripts/release/test-release-index.sh
	bash scripts/release/test-finalize-changelog.sh
	bash scripts/release/test-readme-version.sh
	bash scripts/release/test-release-impact.sh
	bash scripts/release/test-lockfiles.sh
	bash scripts/release/test-release-prose-warning.sh
	bash scripts/release/test-release-gate.sh
	bash scripts/release/test-version-preparation.sh
	bash scripts/release/test-tag-at-head.sh
	bash scripts/release/test-commit-release.sh
	bash scripts/ci/test-pocketic-verification.sh
	bash scripts/ci/test-git-hook.sh
	bash scripts/ci/test-repository-checks.sh
	bash scripts/release/readme-version.sh --check

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

release-x:
	@if [ -z "$(VERSION)" ]; then echo "error: VERSION=x.y.z is required" >&2; exit 2; fi
	bash scripts/release/bump-version.sh --check "$(VERSION)"
	bash scripts/release/commit-release.sh --check-before-bump
	+$(MAKE) --no-print-directory release-verify
	+$(MAKE) --no-print-directory bump-x VERSION="$(VERSION)"
	+$(MAKE) --no-print-directory release-stage
	+$(MAKE) --no-print-directory release-commit
	+$(MAKE) --no-print-directory release-push

release-stage:
	@set -e; bash scripts/release/workspace-version.sh >/dev/null; \
		git add Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md README.md

release-commit:
	@bash scripts/release/commit-release.sh

release-tag-check:
	@bash scripts/release/check-tag-at-head.sh

release-push: ensure-clean release-tag-check
	@set -e; version="$$(bash scripts/release/workspace-version.sh)"; \
		git push --no-follow-tags --atomic "$(RELEASE_REMOTE)" \
		"HEAD:refs/heads/$(RELEASE_BRANCH)" "refs/tags/v$$version:refs/tags/v$$version"

publish: ensure-clean release-tag-check package
	cargo publish --locked --registry crates-io -p ic-timers

# Shared Tooling owns the standard release order and Git effects.
RELEASE_REMOTE ?= origin
RELEASE_BRANCH ?= main
ifneq ($(word 2,$(filter release-patch release-minor release-major release-resume,$(MAKECMDGOALS))),)
$(error Select exactly one release target)
endif
.PHONY: release-resume release-version release-preflight release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check

release-patch release-minor release-major:
	+@bash scripts/ci/run-release.sh "$(@:release-%=%)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

release-resume:
	+@bash scripts/ci/run-release.sh resume "$(VERSION)" "$(RELEASE_REMOTE)" "$(RELEASE_BRANCH)"

release-version:
	@bash scripts/release/workspace-version.sh
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
	@printf '%s\0' Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md README.md
release-tagged-check release-push-check:
	@bash scripts/release/adapter.sh check-committed
	@bash scripts/release/check-tag-at-head.sh "$(RELEASE_COMMIT)" "$(RELEASE_VERSION)"
