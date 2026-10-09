.PHONY: \
	actions-check build bump-x check ci clean clippy docs-check ensure-clean fetch help \
	install-hooks install-testkit-server major minor msrv package patch pocketic-cohorts pocketic-watchdog publish release-check release-commit \
	pocketic-check provider-check release-push release-stage \
	release-impact release-tag-check release-verify release-x repository-check shell-check test testing-check update-dev \
	version wasm-check

MSRV ?= 1.88.0
VERSION ?=
include make/tools.mk
include make/rust-format.mk

CI_TARGETS := actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package
RELEASE_TARGETS := fetch install-testkit-server pocketic-check ci msrv testing-check pocketic-watchdog pocketic-cohorts
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
	@echo "  provider-check      Enforce the private ic-cdk-timers provider boundary"
	@echo "  testing-check       Check formatting and lint supported unpublished probes"
	@echo "  release-verify      Run the complete fail-closed release evidence gate"
	@echo "  release-impact      Classify changes since the current version tag"
	@echo "  repository-check    Validate a non-published repository-only update"
	@echo "  publish             Publish the clean, tagged release to crates.io"
	@echo "  actions-check       Check parsed Actions and dependency pin declarations"
	@echo "  install-host-tools  Install pinned jq/yq/ripgrep/cloc host tools"
	@echo "  host-tools-check    Verify the complete host bundle offline"
	@echo "  install-ic-tools    Explicitly install the reviewed five-tool IC bundle"
	@echo "  install-testkit-server  Prepare the locked Testkit CLI and its admitted server"
	@echo "  ic-tools-check      Verify the installed IC bundle offline"
	@echo "  install-tools / tools-check  Prepare or verify both tool bundles"
	@echo "  cloc                Report root-workspace Rust LOC and test counts"
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

check:
	cargo check -p ic-timers --all-targets --all-features --locked

clippy:
	cargo clippy -p ic-timers --all-targets --all-features --locked -- -D warnings

docs-check:
	RUSTDOCFLAGS="-D warnings" cargo doc -p ic-timers --all-features --no-deps --locked

test:
	cargo test -p ic-timers --all-targets --all-features --locked
	cargo test -p ic-timers --doc --all-features --locked

wasm-check:
	cargo check -p ic-timers --all-features --locked --target wasm32-unknown-unknown

msrv:
	cargo +$(MSRV) check -p ic-timers --all-targets --all-features --locked
	cargo +$(MSRV) test -p ic-timers --doc --all-features --locked

testing-check:
	+$(MAKE) --no-print-directory fmt-check
	cargo +$(MSRV) clippy \
		-p ic-timers-runtime-probe -p ic-timers-pocketic --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features baseline --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features once --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features after-completion --all-targets --locked -- -D warnings
	cargo +$(MSRV) clippy \
		-p ic-timers-size-probe --no-default-features --features watchdog --all-targets --locked -- -D warnings

package:
	cargo package --locked --offline --allow-dirty -p ic-timers

install-testkit-server:
	bash scripts/dev/testkit-server.sh setup

pocketic-check:
	@bash scripts/dev/testkit-server.sh check

pocketic-watchdog: pocketic-check
	CARGO_TARGET_DIR="$(CURDIR)/testing/target" \
		cargo +$(MSRV) build -p ic-timers-runtime-probe \
		--profile timer-probe --target wasm32-unknown-unknown --locked
	@set -e; server="$$(bash scripts/dev/testkit-server.sh check)"; \
		POCKET_IC_BIN="$$server" \
		IC_TIMERS_PROBE_WASM="$(CURDIR)/testing/target/wasm32-unknown-unknown/timer-probe/ic_timers_runtime_probe.wasm" \
		cargo +$(MSRV) test -p ic-timers-pocketic --locked tests::

pocketic-cohorts: pocketic-check
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-baseline" \
		cargo +$(MSRV) build -p ic-timers-size-probe \
		--profile timer-probe --target wasm32-unknown-unknown --locked
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-once" \
		cargo +$(MSRV) build -p ic-timers-size-probe \
		--profile timer-probe --target wasm32-unknown-unknown --locked --no-default-features --features once
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-after-completion" \
		cargo +$(MSRV) build -p ic-timers-size-probe \
		--profile timer-probe --target wasm32-unknown-unknown --locked --no-default-features --features after-completion
	CARGO_TARGET_DIR="$(CURDIR)/testing/target/cohort-watchdog" \
		cargo +$(MSRV) build -p ic-timers-size-probe \
		--profile timer-probe --target wasm32-unknown-unknown --locked --no-default-features --features watchdog
	@set -e; server="$$(bash scripts/dev/testkit-server.sh check)"; \
		POCKET_IC_BIN="$$server" \
		IC_TIMERS_COHORT_ROOT="$(CURDIR)/testing/target" \
		cargo +$(MSRV) test -p ic-timers-pocketic --locked \
			comparable_policy_cohorts_report_size_and_instruction_subjects -- --nocapture

# Keep the established gate entry point; the shared owner also checks Cargo
# declarations and the tracked root lockfile for all maintained members.
actions-check: host-tools-check
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/check-dependency-pins.sh \
		--consumer "$(CURDIR)" --cargo-inheritance

shell-check:
	@set -e; for script in .githooks/pre-commit scripts/ci/*.sh scripts/dev/*.sh scripts/release/*.sh \
		.shared-tooling/helpers/scripts/ci/*.sh; do \
		bash -n "$$script"; \
	done

release-check:
	bash scripts/ci/verify-shared-tooling-snapshot.sh
	bash scripts/ci/verify-shared-tooling-snapshot.sh --manifest .shared-tooling-audits.snapshot
	bash .shared-tooling/helpers/scripts/ci/verify-shared-tooling-snapshot.sh \
		--consumer "$(CURDIR)/.shared-tooling/helpers"
	bash scripts/ci/test-shared-snapshots.sh
	bash scripts/ci/test-host-tools.sh
	bash scripts/ci/test-tool-commands.sh
	bash scripts/ci/test-rust-tools.sh
	bash scripts/ci/test-cloc.sh
	bash scripts/ci/test-failure-evidence.sh
	bash scripts/ci/test-format-tools.sh
	bash scripts/ci/test-ic-tools.sh
	bash scripts/ci/test-evidence-checksums.sh
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/test-dependency-pins.sh
	YQ="$(CURDIR)/.tools/host/bin/yq" bash .shared-tooling/helpers/scripts/ci/test-cargo-metadata.sh
	perl .shared-tooling/helpers/scripts/ci/test-local-lock-versions.pl
	bash scripts/ci/test-release-runner.sh
	bash .shared-tooling/helpers/scripts/ci/check-release-commands.sh "$(CURDIR)" \
		ci/tool-versions.env make/tools.mk make/rust-format.mk make/release.mk
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
	bash scripts/ci/test-testkit-server.sh
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
	cargo build -p ic-timers --all-targets --all-features --locked

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
	+@set -e; \
		bash scripts/ci/check-make-execution.sh; \
		if [ -z "$(VERSION)" ]; then echo "error: VERSION=x.y.z is required" >&2; exit 2; fi; \
		bash scripts/release/bump-version.sh --check "$(VERSION)"; \
		bash scripts/release/commit-release.sh --check-before-bump; \
		$(MAKE) --no-print-directory release-verify; \
		$(MAKE) --no-print-directory bump-x VERSION="$(VERSION)"; \
		$(MAKE) --no-print-directory release-stage; \
		$(MAKE) --no-print-directory release-commit; \
		$(MAKE) --no-print-directory release-push

release-stage:
	@set -e; bash scripts/release/workspace-version.sh >/dev/null; \
		git add Cargo.toml Cargo.lock CHANGELOG.md README.md

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

# These consumer entrypoints use the reviewed root snapshot. Preserve their
# former repository-local routing even if a caller exports another snapshot root.
format-tools-check fmt fmt-check release-patch release-minor release-major release-resume: override SHARED_TOOLING_ROOT = $(CURDIR)
format-tools-check fmt fmt-check: override HOST_TOOL_VERSIONS = $(CURDIR)/ci/tool-versions.env
# Keep direct delivery authoritative for all standard entrypoints, including
# when an ambient environment or Make command-line variable selects PR delivery.
release-patch release-minor release-major release-resume: override export RELEASE_DELIVERY := direct
include make/release.mk
.PHONY: release-version release-preflight release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check

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
	@printf '%s\0' Cargo.toml Cargo.lock CHANGELOG.md README.md
release-tagged-check release-push-check:
	@bash scripts/release/adapter.sh check-committed
	@bash scripts/release/check-tag-at-head.sh "$(RELEASE_COMMIT)" "$(RELEASE_VERSION)"
