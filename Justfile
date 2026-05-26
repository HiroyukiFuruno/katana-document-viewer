set shell := ["bash", "-uc"]

REPO_ROOT := justfile_directory()
RTK := env_var_or_default("RTK", `command -v rtk 2> /dev/null || true`)
RTK_CMD := if RTK == "" { "" } else { RTK + " " }
JOBS := env_var_or_default("JOBS", "2")
CARGO := env_var_or_default("CARGO", "cargo")
VERSION := env_var_or_default("VERSION", `awk -F '"' '/^version = / { print $2; exit }' Cargo.toml`)
VERSION_BARE := replace(VERSION, "v", "")
TAG := "v" + VERSION_BARE
COVERAGE_MIN_LINES := "100"
RELEASE_REPO := env_var_or_default("RELEASE_REPO", "HiroyukiFuruno/katana-document-viewer")
KAL_VERSION := env_var_or_default("KAL_VERSION", "0.5.1")
KAL_ROOT := env_var_or_default("KAL_ROOT", REPO_ROOT + "/target/kal")
KAL := env_var_or_default("KAL", KAL_ROOT + "/bin/kal")

export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings")

default: help

# Show available tasks
help:
    @just --list --unsorted

# Apply Rust formatting
fmt:
    {{CARGO}} fmt --all

# Check Rust formatting
fmt-check:
    {{CARGO}} fmt --all -- --check

# Run strict Clippy checks
lint:
    {{CARGO}} clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::todo -D clippy::unimplemented -D clippy::dbg_macro -D clippy::panic -D clippy::wildcard_imports

# Run Rust syntax based structural checks
ast-lint: ensure-kal
    "{{KAL}}" check

# Run workspace tests
test:
    {{CARGO}} test --workspace --all-targets --all-features --locked

# Backward-compatible test entrypoint
unit-test: test

# Run coverage as a required full-check gate
coverage:
    {{CARGO}} llvm-cov --workspace --all-targets --all-features --locked --summary-only --fail-under-lines {{COVERAGE_MIN_LINES}}

# Show missing coverage lines without relaxing the coverage gate
coverage-missing:
    {{CARGO}} llvm-cov --workspace --all-targets --all-features --locked --show-missing-lines --fail-under-lines {{COVERAGE_MIN_LINES}}

# Run the local quality gate
check: fmt-check lint ast-lint test
    @echo "checks passed"

# Verify VERSION follows the published release line
release-target-check:
    bash scripts/release/verify-version.sh "{{VERSION}}"
    python3 scripts/release/verify-release-target.py --target-version "{{TAG}}" --repo "{{RELEASE_REPO}}"

# Verify package metadata. crates.io publish is disabled until crate naming is fixed.
release-verify: check coverage
    bash scripts/release/verify-version.sh "{{VERSION}}"
    bash scripts/release/assert-no-crates-publish-target.sh

# Verify release branch readiness before merging
release-check: release-target-check release-verify
    bash scripts/release/assert-crates-not-published.sh "{{VERSION}}"

# Sweep old build artifacts locally
sweep:
    @{{RTK_CMD}}cargo sweep --time 7 || true

# Remove build artifacts
clean: sweep
    {{CARGO}} clean

# Update dependency crates safely
update-safe:
    {{RTK_CMD}}cargo update

# Upgrade all dependency crates and refresh the lockfile
update:
    {{RTK_CMD}}cargo upgrade -i
    {{RTK_CMD}}cargo update

# List outdated dependency crates
outdated:
    @cp Cargo.toml Cargo.toml.bak
    @sed -e '/^\[patch\.crates-io\]/,$d' Cargo.toml.bak > Cargo.toml
    @{{RTK_CMD}}cargo outdated --workspace || (mv Cargo.toml.bak Cargo.toml && exit 1)
    @mv Cargo.toml.bak Cargo.toml

[private]
ensure-kal:
    @if [[ "{{KAL}}" == */* ]]; then \
        test -x "{{KAL}}" && "{{KAL}}" version | grep -q "kal {{KAL_VERSION}}" && exit 0; \
      elif command -v "{{KAL}}" >/dev/null 2>&1 && "{{KAL}}" version | grep -q "kal {{KAL_VERSION}}"; then \
        exit 0; \
      fi; \
      {{CARGO}} install katana-ast-lint --version "{{KAL_VERSION}}" --locked --force --root "{{KAL_ROOT}}"
