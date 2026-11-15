#!/bin/sh
# Check entry point of spec 0002, "The check entry point".
# Usage: scripts/check.sh <name> [<name>...]
# To reproduce a failed CI job named X, check out the same commit and run scripts/check.sh X.
set -eu

# Pinned tool versions. They are written here and nowhere else.
CARGO_DENY_VERSION=0.20.2
CARGO_FUZZ_VERSION=0.13.2
CARGO_CYCLONEDX_VERSION=0.5.10
FUZZ_TOOLCHAIN=nightly-2026-10-17
# The PostgreSQL image of the integration check, pinned by its index digest (spec 0004).
POSTGRES_IMAGE=postgres:18.7-bookworm@sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f

usage() {
  cat >&2 <<'USAGE'
usage: scripts/check.sh <name> [<name>...]

required checks: fmt clippy test deny linux-build guards docs
all              fmt clippy test deny guards docs, in that order, without stopping at a failure
tools            make cargo-deny available under target/tools
fuzz-tools       make cargo-fuzz and the fuzz toolchain available
fuzz             run every fuzz target of both fuzz packages (not a required check)
integration      run apps/pool/tests/integration.rs against PostgreSQL 18 in Docker (not a required check)
sbom             write target/sbom/pool.cdx.json (not a required check)

exit status: 0 every named check passed, 1 a check failed, 2 usage error, 3 a check could not run
USAGE
}

CDPATH= cd -- "$(dirname -- "$0")/.."
ROOT=$(pwd)

: "${CARGO_INCREMENTAL=0}"
: "${CARGO_PROFILE_DEV_DEBUG=0}"
export CARGO_INCREMENTAL CARGO_PROFILE_DEV_DEBUG
PATH="$ROOT/target/tools/bin:$PATH"
export PATH

# The status of the check that is running: 0 passed, 1 failed, 3 could not run.
status=0

# Runs one command of a check. Any status other than 0 fails the check.
run() {
  rc=0
  "$@" || rc=$?
  if [ "$rc" -ne 0 ]; then
    status=1
    return 1
  fi
}

# Runs one command whose exit 3 means the check could not run.
run3() {
  rc=0
  "$@" || rc=$?
  case "$rc" in
    0) return 0 ;;
    3) status=3 ;;
    *) status=1 ;;
  esac
  return 1
}

# Reports that a check could not run, with one line that says what is missing.
missing() {
  echo "$1" >&2
  status=3
  return 1
}

need_command() {
  command -v "$1" >/dev/null 2>&1 || missing "$2: $1 is required and was not found on PATH"
}

# Succeeds when "cargo <subcommand> --version" prints exactly "<tool> <version>".
tool_is() {
  out=$(cargo "$1" --version 2>/dev/null) || out=
  [ "$out" = "$2 $3" ]
}

repo_check() {
  if [ ! -f tools/repo-check/Cargo.toml ]; then
    missing "pool-repo-check is required and tools/repo-check/Cargo.toml does not exist"
    return 1
  fi
  run3 cargo run -q --locked --manifest-path tools/repo-check/Cargo.toml --target-dir target/repo-check -- "$@"
}

check_fmt() {
  need_command cargo fmt || return 1
  run cargo fmt --all --check || return 1
  run cargo fmt --check --manifest-path tools/repo-check/Cargo.toml || return 1
  run cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml || return 1
  run cargo fmt --check --manifest-path crates/pool-protocol/fuzz/Cargo.toml
}

check_clippy() {
  need_command cargo clippy || return 1
  run cargo clippy --workspace --all-targets --locked -- -D warnings || return 1
  run cargo clippy --manifest-path tools/repo-check/Cargo.toml --target-dir target/repo-check --all-targets --locked -- -D warnings
}

check_test() {
  need_command cargo test || return 1
  run cargo test --workspace --locked || return 1
  run cargo test --manifest-path tools/repo-check/Cargo.toml --target-dir target/repo-check --locked
}

check_deny() {
  need_command cargo deny || return 1
  if ! tool_is deny cargo-deny "$CARGO_DENY_VERSION"; then
    missing "cargo-deny $CARGO_DENY_VERSION is required; run scripts/check.sh tools" || return 1
  fi
  run cargo deny --locked --config deny.toml check || return 1
  # The list of allowed licenses is the list of the product, and the command above keeps it from growing unseen.
  # A fuzz lockfile holds a part of the product and the fuzz runtime, so for it an allowed license that no package
  # uses is not an error (spec 0004, "Dependency policy"). Every other rule of deny.toml applies unchanged.
  run cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check -A license-not-encountered || return 1
  run cargo deny --locked --manifest-path crates/pool-protocol/fuzz/Cargo.toml --config deny.toml check -A license-not-encountered
}

check_linux_build() {
  need_command docker linux-build || return 1
  need_command git linux-build || return 1
  run3 scripts/linux-build.sh
}

check_guards() {
  need_command cargo guards || return 1
  repo_check workspace || return 1
  repo_check ci
}

check_docs() {
  need_command cargo docs || return 1
  need_command git docs || return 1
  base=
  if [ -n "${POOL_CHECK_BASE:-}" ] && [ "$POOL_CHECK_BASE" != "0000000000000000000000000000000000000000" ]; then
    base=$POOL_CHECK_BASE
  elif git rev-parse --verify -q origin/main >/dev/null; then
    base=origin/main
  fi
  if [ -n "$base" ]; then
    repo_check docs --base "$base"
  else
    repo_check docs
  fi
}

check_tools() {
  need_command cargo tools || return 1
  if tool_is deny cargo-deny "$CARGO_DENY_VERSION"; then
    echo "cargo-deny $CARGO_DENY_VERSION is already available"
    return 0
  fi
  if ! cargo install --locked --root target/tools "cargo-deny@$CARGO_DENY_VERSION"; then
    missing "tools: cargo install of cargo-deny $CARGO_DENY_VERSION failed" || return 1
  fi
}

check_integration() {
  need_command docker integration || return 1
  need_command cargo integration || return 1
  run3 env POOL_IT_IMAGE="$POSTGRES_IMAGE" scripts/integration/run.sh
}

check_fuzz_tools() {
  need_command cargo fuzz-tools || return 1
  need_command rustup fuzz-tools || return 1
  if tool_is fuzz cargo-fuzz "$CARGO_FUZZ_VERSION"; then
    echo "cargo-fuzz $CARGO_FUZZ_VERSION is already available"
  elif ! cargo install --locked --root target/tools "cargo-fuzz@$CARGO_FUZZ_VERSION"; then
    missing "fuzz-tools: cargo install of cargo-fuzz $CARGO_FUZZ_VERSION failed" || return 1
  fi
  if rustup toolchain list | grep -q "^$FUZZ_TOOLCHAIN"; then
    echo "the toolchain $FUZZ_TOOLCHAIN is already installed"
  elif ! rustup toolchain install --profile minimal "$FUZZ_TOOLCHAIN"; then
    missing "fuzz-tools: rustup toolchain install $FUZZ_TOOLCHAIN failed" || return 1
  fi
}

# Runs one fuzz target of one package on its corpus, its seed directory (when it has one) and its committed
# regression inputs. Usage: fuzz_target <crate dir> <target> <max_len> [<seed dir>]
fuzz_target() {
  crate=$1
  target=$2
  max_len=$3
  seed=${4:-}
  mkdir -p "$crate/fuzz/corpus/$target"
  run env CARGO_TARGET_DIR="$ROOT/target/fuzz" sh -c 'CDPATH= cd -- "$1" && shift && exec cargo "+$1" fuzz run "$2" "fuzz/corpus/$2" "fuzz/regressions/$2" $5 -- "-max_total_time=$3" "-max_len=$4"' \
    fuzz "$crate" "$FUZZ_TOOLCHAIN" "$target" "$seconds" "$max_len" "$seed" || return 1
  # cargo-fuzz creates the directory even when nothing crashed, so only a file in it is a finding.
  if [ -d "$crate/fuzz/artifacts/$target" ] && [ -n "$(find "$crate/fuzz/artifacts/$target" -type f | head -n 1)" ]; then
    echo "fuzz: a crash file exists under $crate/fuzz/artifacts/$target/; report it with its bytes" >&2
    status=1
    return 1
  fi
}

check_fuzz() {
  need_command cargo fuzz || return 1
  need_command rustup fuzz || return 1
  if ! tool_is fuzz cargo-fuzz "$CARGO_FUZZ_VERSION"; then
    missing "cargo-fuzz $CARGO_FUZZ_VERSION is required; run scripts/check.sh fuzz-tools" || return 1
  fi
  if ! rustup toolchain list | grep -q "^$FUZZ_TOOLCHAIN"; then
    missing "the toolchain $FUZZ_TOOLCHAIN is required; run scripts/check.sh fuzz-tools" || return 1
  fi
  seconds=${POOL_FUZZ_SECONDS:-30}
  fuzz_target crates/pool-config validate 4096 tests/fixtures || return 1
  fuzz_target crates/pool-protocol startup 16384 || return 1
  fuzz_target crates/pool-protocol frontend_message 16384 || return 1
  fuzz_target crates/pool-protocol scram 4096
}

check_sbom() {
  need_command cargo sbom || return 1
  # cargo-cyclonedx 0.5.10 prints its name twice: `cargo-cyclonedx-cyclonedx 0.5.10`.
  if ! tool_is cyclonedx cargo-cyclonedx-cyclonedx "$CARGO_CYCLONEDX_VERSION"; then
    missing "cargo-cyclonedx $CARGO_CYCLONEDX_VERSION is required; run cargo install --locked --root target/tools cargo-cyclonedx@$CARGO_CYCLONEDX_VERSION" || return 1
  fi
  # The tool can write pool.cdx.json next to a manifest before it fails, so the cleanup runs
  # whatever the tool returned, and a failed cleanup fails the check.
  sbom_ok=1
  run cargo cyclonedx --manifest-path apps/pool/Cargo.toml --format json --spec-version 1.5 --all --override-filename pool.cdx || sbom_ok=0
  if [ "$sbom_ok" -eq 1 ]; then
    run mkdir -p target/sbom || sbom_ok=0
  fi
  if [ "$sbom_ok" -eq 1 ]; then
    run mv apps/pool/pool.cdx.json target/sbom/pool.cdx.json || sbom_ok=0
  fi
  run find apps crates -name 'pool.cdx.json' -not -path '*/target/*' -exec rm -f {} + || return 1
  [ "$sbom_ok" -eq 1 ]
}

# Validate every name before running anything.
if [ "$#" -eq 0 ]; then
  usage
  exit 2
fi
names=
for arg in "$@"; do
  case "$arg" in
    all) names="$names fmt clippy test deny guards docs" ;;
    fmt | clippy | test | deny | linux-build | guards | docs | tools | fuzz-tools | fuzz | sbom | integration) names="$names $arg" ;;
    *)
      echo "scripts/check.sh: unknown check: $arg" >&2
      usage
      exit 2
      ;;
  esac
done

set -- $names
summary=
any_failed=0
any_not_run=0
for name in "$@"; do
  echo "== check: $name"
  status=0
  case "$name" in
    linux-build) check_linux_build || : ;;
    fuzz-tools) check_fuzz_tools || : ;;
    *) "check_$name" || : ;;
  esac
  case "$status" in
    0) line="== ok: $name" ;;
    3) line="== FAILED: $name"; any_not_run=1 ;;
    *) line="== FAILED: $name"; any_failed=1 ;;
  esac
  echo "$line"
  summary="$summary$line
"
done

if [ "$#" -gt 1 ]; then
  echo "== summary"
  printf '%s' "$summary"
fi

if [ "$any_failed" -ne 0 ]; then
  exit 1
fi
if [ "$any_not_run" -ne 0 ]; then
  exit 3
fi
exit 0
