#!/bin/sh
# Proof that every check rejects a deliberately wrong change, spec 0002, "Proving the checks" (0002 AC-11).
# Usage: scripts/prove-checks.sh [--with-linux-build]
# It works on a private clone of the committed HEAD and never changes the working tree it was started from.
set -eu

# An exported GIT_DIR, GIT_WORK_TREE or GIT_INDEX_FILE (Git sets them for hooks and rebase --exec) would win over
# git -C and send the commits of the proof to the repository this script was started from.
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE

usage() {
  echo "usage: scripts/prove-checks.sh [--with-linux-build]" >&2
}

with_linux_build=0
for arg in "$@"; do
  case "$arg" in
    --with-linux-build) with_linux_build=1 ;;
    *)
      echo "scripts/prove-checks.sh: unknown argument: $arg" >&2
      usage
      exit 2
      ;;
  esac
done

CDPATH= cd -- "$(dirname -- "$0")/.."
# The checks run in the copy change directory too, so an exported CDPATH must not reach them either.
unset CDPATH
ROOT=$(pwd)

for tool in git cargo awk; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "prove-checks: $tool is required and was not found on PATH" >&2
    exit 3
  fi
done
if [ -n "$(git status --porcelain)" ]; then
  echo "prove-checks: commit or remove local changes first; the proof uses only the committed tree" >&2
  exit 3
fi

COMMIT=$(git rev-parse HEAD)
TMP=$(mktemp -d "${TMPDIR:-/tmp}/pool-prove.XXXXXX")
trap 'rm -rf "$TMP"' EXIT INT TERM
COPY=$TMP/tree
OUT=$TMP/out.txt

git clone -q "$ROOT" "$COPY"
git -C "$COPY" checkout -q --detach "$COMMIT"

: "${CARGO_INCREMENTAL=0}"
: "${CARGO_PROFILE_DEV_DEBUG=0}"
POOL_CHECK_BASE=$COMMIT
CARGO_TARGET_DIR=$ROOT/target/prove
PATH="$ROOT/target/tools/bin:$PATH"
export CARGO_INCREMENTAL CARGO_PROFILE_DEV_DEBUG POOL_CHECK_BASE CARGO_TARGET_DIR PATH

# --- edits, all on files inside the copy, with plain text matching ---

# Rewrites a file in place from the output of a command, keeping its mode.
rewrite() {
  file=$1
  shift
  "$@" "$file" >"$TMP/edit"
  cat "$TMP/edit" >"$file"
}

append() {
  printf '%s\n' "$2" >>"$1"
}

# Inserts a line after the first line that equals the header.
insert_after() {
  rewrite "$1" awk -v h="$2" -v l="$3" '{ print } !done && $0 == h { print l; done = 1 }'
}

# Replaces the first occurrence of a literal text.
replace_first() {
  rewrite "$1" awk -v a="$2" -v b="$3" '
    !done && (i = index($0, a)) > 0 {
      $0 = substr($0, 1, i - 1) b substr($0, i + length(a))
      done = 1
    }
    { print }'
}

# Deletes the lines that start with a literal text.
delete_lines_starting() {
  rewrite "$1" awk -v p="$2" 'index($0, p) != 1 { print }'
}

# Deletes a table header and its lines up to the next table.
delete_table() {
  rewrite "$1" awk -v h="$2" '
    $0 == h { skip = 1; next }
    skip && /^\[/ { skip = 0 }
    !skip { print }'
}

# Replaces the first line that equals the given text.
replace_line() {
  rewrite "$1" awk -v a="$2" -v b="$3" '!done && $0 == a { $0 = b; done = 1 } { print }'
}

# Replaces the first line that starts with the given text.
replace_line_starting() {
  rewrite "$1" awk -v a="$2" -v b="$3" '!done && index($0, a) == 1 { $0 = b; done = 1 } { print }'
}

# Replaces the first commit hash after actions/checkout@ with v7.
unpin_checkout() {
  rewrite "$1" awk '
    !done && match($0, /actions\/checkout@[0-9a-f]+/) {
      $0 = substr($0, 1, RSTART - 1) "actions/checkout@v7" substr($0, RSTART + RLENGTH)
      done = 1
    }
    { print }'
}

# --- the cases ---

restore() {
  git -C "$COPY" reset -q --hard "$COMMIT"
  git -C "$COPY" clean -fdq
}

# Runs one check inside the copy. Its output goes to $OUT; its status is returned.
check() {
  rc=0
  (cd "$COPY" && scripts/check.sh "$1") >"$OUT" 2>&1 || rc=$?
  return "$rc"
}

failed=
note() {
  failed="$failed $1"
  echo "FAIL $1 $2 $3"
}

# Runs the check for a case that is already applied and compares exit status and output.
expect() {
  case_id=$1
  name=$2
  shift 2
  rc=0
  check "$name" || rc=$?
  if [ "$rc" -eq 0 ]; then
    note "$case_id" "$name" "accepted the change (exit 0)"
    return 0
  fi
  for text in "$@"; do
    if ! grep -qF -- "$text" "$OUT"; then
      note "$case_id" "$name" "exit $rc, but the output does not contain: $text"
      return 0
    fi
  done
  echo "PASS $case_id $name rejected the change"
  matched "$@"
}

# Prints the first output line that contains every required text, else the first that contains the first one, as evidence.
matched() {
  cp "$OUT" "$TMP/match"
  for text in "$@"; do
    grep -F -- "$text" "$TMP/match" >"$TMP/match.next" || :
    cat "$TMP/match.next" >"$TMP/match"
  done
  line=$(head -n 1 "$TMP/match")
  if [ -z "$line" ]; then
    line=$(grep -F -m 1 -- "$1" "$OUT" || :)
  fi
  echo "  exit $rc, matched: $(printf '%s' "$line" | cut -c 1-200)"
}

C=$COPY

apply_case_4() {
  printf '%s\n' 'config_version = 2' >"$C/crates/pool-config/tests/fixtures/valid-minimal.toml"
}

echo "prove-checks: commit $COMMIT, copy in $COPY"
echo "prove-checks: scripts/check.sh all on the unchanged copy"
rc=0
check all || rc=$?
if [ "$rc" -ne 0 ]; then
  cat "$OUT"
  echo "prove-checks: scripts/check.sh all failed on the unchanged copy (exit $rc); a case could not show its own failure" >&2
  exit 1
fi

append "$C/apps/pool/src/main.rs" 'fn  prove_fmt( ){}'
expect 1 fmt 'Diff in'
restore

append "$C/crates/pool-config/src/lib.rs" 'pub fn prove_clippy(v: &Vec<u8>) -> usize { v.len() }'
expect 2 clippy 'ptr_arg'
restore

append "$C/crates/pool-config/src/lib.rs" 'pub fn prove_print() { println!("x"); }'
expect 3 clippy 'print_stdout'
restore

apply_case_4
expect 4 test 'test result: FAILED'
restore

replace_first "$C/Cargo.toml" 'license = "Apache-2.0"' 'license = "GPL-3.0-only"'
expect 5 deny 'license is not explicitly allowed'
restore

# Case 6: a made up advisory in a private copy of the advisory database.
insert_after "$C/deny.toml" '[advisories]' "db-path = \"$TMP/advisory-db\""
rc=0
check deny || rc=$?
if [ "$rc" -ne 0 ]; then
  note 6 deny "the first run with a private advisory database failed (exit $rc)"
else
  set -- "$TMP"/advisory-db/advisory-db-*
  if [ "$#" -ne 1 ] || [ ! -d "$1" ]; then
    note 6 deny "expected one advisory-db-* directory under $TMP/advisory-db"
  else
    mkdir -p "$1/crates/itoa"
    cat >"$1/crates/itoa/RUSTSEC-0000-0001.md" <<'ADVISORY'
```toml
[advisory]
id = "RUSTSEC-0000-0001"
package = "itoa"
date = "2026-11-14"
url = "https://example.invalid/pool-fixture"
categories = ["memory-corruption"]

[versions]
patched = []
```

# Fixture advisory used to prove the advisory check

This advisory is not real. It exists only so the check can be shown to fail.
ADVISORY
    expect 6 deny 'RUSTSEC-0000-0001'
  fi
fi
restore

insert_after "$C/apps/pool/Cargo.toml" '[dependencies]' 'itoa.workspace = true'
insert_after "$C/Cargo.toml" '[workspace.dependencies]' 'itoa = "1"'
expect 7 deny 'cannot update the lock file'
restore

delete_lines_starting "$C/tools/repo-check/baseline/lock-packages.txt" 'zmij'
expect 8 guards 'W1' 'zmij'
restore

mkdir -p "$C/crates/pool-core"
: >"$C/crates/pool-core/.keep"
expect 9 guards 'W2' 'crates/pool-core'
restore

append "$C/apps/pool/src/main.rs" '// std::thread'
expect 10 guards 'W3' 'std::thread'
restore

delete_table "$C/apps/pool/Cargo.toml" '[lints]'
expect 11 guards 'W4'
restore

insert_after "$C/crates/pool-config/Cargo.toml" '[dependencies]' 'pool = { path = "../../apps/pool" }'
expect 12 guards 'W5' 'pool-config -> pool'
restore

unpin_checkout "$C/.github/workflows/ci.yml"
expect 13 guards 'C1'
restore

replace_first "$C/.github/workflows/ci.yml" '  docs:' '  documents:'
expect 14 guards 'C2'
restore

append "$C/README.md" '[broken](docs/missing-file.md)'
expect 15 docs 'D1' 'docs/missing-file.md'
restore

append "$C/README.md" '[broken](docs/glossary.md#no-such-heading)'
expect 16 docs 'D2' 'no-such-heading'
restore

rewrite "$C/docs/scope/003-control-and-session-data-model.md" awk 'NR == 1 { sub(/planned/, "done") } { print }'
expect 17 docs 'D3'
restore

append "$C/README.md" '`SEC-99`'
expect 18 docs 'D5'
restore

delete_lines_starting "$C/docs/security/security-requirements.md" '| SEC-14 |'
expect 19 docs 'D6' 'SEC-14'
restore

linux_skipped=0
if [ "$with_linux_build" -eq 1 ] && command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
  apply_case_4
  git -C "$COPY" -c user.name=prove -c user.email=prove@example.invalid -c commit.gpgsign=false -c core.hooksPath=/dev/null commit -q -am 'prove case 20'
  rc=0
  check linux-build || rc=$?
  if [ "$rc" -eq 0 ]; then
    note 20 linux-build "accepted the change (exit 0)"
  elif grep -qF 'unexpected exit' "$OUT"; then
    echo "PASS 20 linux-build rejected the change"
    matched 'unexpected exit'
  elif grep -qF 'test result: FAILED' "$OUT"; then
    echo "PASS 20 linux-build rejected the change"
    matched 'test result: FAILED'
  else
    note 20 linux-build "exit $rc, but the output contains neither unexpected exit nor test result: FAILED"
  fi
  restore
else
  linux_skipped=1
  echo "SKIPPED 20 linux-build"
fi

replace_first "$C/scripts/linux-build.sh" 'linux build check passed' 'linux build check skipped'
expect 21 guards 'W7'
restore

# Case 22: a dependency from a Git address, cloned from a folder on this machine.
SOURCE=$TMP/prove-source
mkdir -p "$SOURCE/src"
: >"$SOURCE/src/lib.rs"
cat >"$SOURCE/Cargo.toml" <<'SOURCE_MANIFEST'
[package]
name = "pool-prove-source"
version = "0.0.0"
edition = "2024"
license = "Apache-2.0"
SOURCE_MANIFEST
git -C "$SOURCE" init -q
git -C "$SOURCE" add Cargo.toml src/lib.rs
git -C "$SOURCE" -c user.name=prove -c user.email=prove@example.invalid -c commit.gpgsign=false -c core.hooksPath=/dev/null commit -q -m 'prove source'
insert_after "$C/crates/pool-config/Cargo.toml" '[dependencies]' "pool-prove-source = { git = \"file://$SOURCE\" }"
rc=0
(cd "$COPY" && cargo metadata --format-version 1 >/dev/null 2>"$OUT") || rc=$?
if [ "$rc" -ne 0 ]; then
  note 22 deny "cargo metadata could not add the Git dependency to Cargo.lock (exit $rc)"
else
  expect 22 deny 'source-not-allowed'
fi
restore

replace_line_starting "$C/Cargo.toml" 'serde_json = ' 'serde_json = "*"'
expect 23 deny 'error[wildcard]'
restore

replace_line "$C/.github/workflows/ci.yml" '    timeout-minutes: 15' '    continue-on-error: true'
expect 24 guards 'C3' 'continue-on-error'
restore

replace_first "$C/.github/workflows/ci.yml" 'ubuntu-24.04-arm' 'ubuntu-latest'
expect 25 guards 'C4'
restore

replace_line "$C/.github/workflows/ci.yml" '  contents: read' '  contents: write'
expect 26 guards 'C5'
restore

: >"$C/docs/Cargo.lock"
expect 27 guards 'W6' 'docs/Cargo.lock'
restore

append "$C/docs/security/security-requirements.md" '| SEC-14 | Fixture row used to prove the duplicate rule |'
expect 28 docs 'D4' 'SEC-14'
restore

if [ -n "$failed" ]; then
  echo "prove-checks: these cases were not rejected as expected:$failed"
  exit 1
fi
if [ "$linux_skipped" -eq 1 ]; then
  echo "all checks reject their bad change (linux-build not proven)"
else
  echo "all checks reject their bad change"
fi
