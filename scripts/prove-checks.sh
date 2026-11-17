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

# Appends a row after the last row of the feature table of the scope index, the table under its first
# "| # | Feature |" header.
append_feature_row() {
  rewrite "$1" awk -v l="$2" '
    state == 1 && index($0, "|") != 1 { print l; state = 2 }
    { print }
    state == 0 && index($0, "| # | Feature |") == 1 { state = 1 }
    END { if (state == 1) print l }'
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

# Add a dependency to Cargo's existing canonical array for one package.
add_locked_dependency() {
  rewrite "$1" awk -v package="$2" -v dependency="$3" '
    /^\[\[package\]\]/ { inside = 0 }
    $0 == "name = \"" package "\"" { inside = 1 }
    { print }
    inside && /^dependencies = \[/ { print " \"" dependency "\","; inside = 0 }'
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

# Since spec 0004 step 3, apps/pool-cli is the one forbidden path.
mkdir -p "$C/apps/pool-cli"
: >"$C/apps/pool-cli/.keep"
expect 9 guards 'W2' 'apps/pool-cli'
restore

append "$C/apps/pool/src/main.rs" '// std::thread'
expect 10 guards 'W3' 'std::thread'
restore

delete_table "$C/apps/pool/Cargo.toml" '[lints]'
expect 11 guards 'W4'
restore

add_locked_dependency "$C/Cargo.lock" 'pool-config' 'pool'
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

# Case 17: a feature of the proof's own, so that the case does not depend on the status of a real feature
# (spec 0004, "Proof script and lockfile check"). Its row says planned and its file says done.
append_feature_row "$C/docs/scope/index.md" '| 99 | [Proof case](099-proof-case.md) | Proof | planned |'
printf '%s\n' '# 99. Proof case · done' >"$C/docs/scope/099-proof-case.md"
expect 17 docs 'D3' '099-proof-case.md'
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

# Case 29: a local package that is no member, followed by a table with a source line, which ends the entry
# (spec 0004, "Proof script and lockfile check"). The baseline copy names it, so only W4 objects.
cat >>"$C/Cargo.lock" <<'LOCK_ENTRY'

[[package]]
name = "pool-prove-ghost"
version = "0.0.0"

[[patch.unused]]
name = "pool-prove-unused"
version = "0.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
LOCK_ENTRY
append "$C/tools/repo-check/baseline/lock-packages.txt" 'pool-prove-ghost'
expect 29 guards 'W4' 'pool-prove-ghost'
restore

# Cases 30 to 35: one per group of the source guard of spec 0004 (0004 AC-2, "Source guard").
# 30: a socket or the runtime outside pool-core and apps/pool.
append "$C/crates/pool-control/src/lib.rs" '// TcpStream'
expect 30 guards 'W3' 'crates/pool-control/src/lib.rs' 'TcpStream'
restore

# 31: a thread or a socket kind that no path allows, also where sockets are allowed.
append "$C/apps/pool/src/main.rs" '// UnixStream'
expect 31 guards 'W3' 'apps/pool/src/main.rs' 'UnixStream'
restore

# 32: a clock, a file call or an environment read in pool-control (0003 AC-11, Follow-up 4 point t6).
append "$C/crates/pool-control/src/lib.rs" '// Instant::now'
expect 32 guards 'W3' 'crates/pool-control/src/lib.rs' 'Instant::now'
restore

# 33: rusqlite in pool-control outside journal.rs.
append "$C/crates/pool-control/src/lib.rs" '// rusqlite'
expect 33 guards 'W3' 'crates/pool-control/src/lib.rs' 'rusqlite'
restore

# 34: a journal value named outside pool-control (point m2, 0003 AC-19).
append "$C/apps/pool/src/main.rs" '// StoreProbe'
expect 34 guards 'W3' 'apps/pool/src/main.rs' 'StoreProbe'
restore

# 35: writer admission called outside pool-control (point m3, the guard half of 0003 AC-18).
append "$C/crates/pool-config/src/lib.rs" '// writer_admission'
expect 35 guards 'W3' 'crates/pool-config/src/lib.rs' 'writer_admission'
restore

# 36: rusqlite named outside pool-control, in apps/pool (decision 93, finding g2: one writer in every crate).
append "$C/apps/pool/src/main.rs" '// rusqlite'
expect 36 guards 'W3' 'apps/pool/src/main.rs' 'rusqlite'
restore

# Scope 005 inherits the tooling debt of spec 0004 Follow-up 14. Each wrong
# change below has a baseline (the unchanged-copy check above) and its own refusal.
append "$C/crates/pool-config/src/lib.rs" 'pub fn prove_grouped_thread() { use std::{thread}; let _ = thread::spawn(|| ()); }'
expect 37 clippy 'disallowed_methods' 'std::thread::spawn'
restore

append "$C/crates/pool-config/src/lib.rs" 'pub fn prove_grouped_socket(_: std::net::UdpSocket) {}'
expect 38 clippy 'disallowed_types' 'std::net::UdpSocket'
restore

append "$C/crates/pool-control/src/lib.rs" 'pub fn prove_grouped_clock() { use std::{time}; let _ = time::Instant::now(); }'
expect 39 clippy 'disallowed_methods' 'std::time::Instant::now'
restore

append "$C/crates/pool-control/src/lib.rs" 'pub fn prove_path_read(path: &std::path::Path) { let _ = path.metadata(); }'
expect 40 clippy 'disallowed_methods' 'std::path::Path::metadata'
restore

for direction in core-control control-core; do
  case "$direction" in
    core-control) from=pool-core; to=pool-control; case_id=41 ;;
    control-core) from=pool-control; to=pool-core; case_id=42 ;;
  esac
  add_locked_dependency "$C/Cargo.lock" "$from" "$to"
  append "$C/tools/repo-check/baseline/internal-dependencies.txt" "$from -> $to"
  expect "$case_id" guards 'W5' "$from -> $to" 'core/control exclusion'
  restore
done

append "$C/tools/repo-check/baseline/forbidden-source-patterns.txt" 'allow crates/pool-control std::fs'
expect 43 guards 'W3' 'deny crates/pool-control/src std::fs is hidden by allow'
restore

# Split the header so this proof script itself does not contain a private key block.
printf '%s%s\n' '-----BEGIN ' 'RSA PRIVATE KEY-----' >"$C/prove-private-key.pem"
expect 44 guards 'W9' 'prove-private-key.pem' 'private key block'
restore

replace_first "$C/crates/pool-core/tests/fixtures/node.key" 'MIGH' 'MIXX'
expect 45 guards 'W9' 'node.key' 'private key block'
restore

case_id=46
for key in AWS_LC_SYS_USE_SYSTEM \
  AWS_LC_SYS_USE_SYSTEM_aarch64_unknown_linux_gnu \
  AWS_LC_SYS_USE_SYSTEM_x86_64_unknown_linux_gnu \
  AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin \
  AWS_LC_SYS_USE_SYSTEM_x86_64_apple_darwin \
  LIBSQLITE3_SYS_USE_PKG_CONFIG; do
  replace_line_starting "$C/.cargo/config.toml" "$key = " "$key = { value = \"0\", force = false }"
  expect "$case_id" guards 'W8' '.cargo/config.toml' 'six native-library settings'
  restore
  case_id=$((case_id + 1))
done

append "$C/.github/workflows/ci.yml" '# POOL_IT_FILTER=one is deliberately forbidden even in comments'
expect 52 guards 'C5' 'POOL_IT_FILTER'
restore

replace_first "$C/NOTICE" 'aws-lc-sys 0.47.0' 'aws-lc-sys 0.46.0'
expect 53 guards 'W10' 'aws-lc-sys' 'Cargo.lock'
restore

replace_first "$C/NOTICE" 'SQLite 3.54.1' 'SQLite 3.54.0'
expect 54 guards 'W10' 'SQLite 3.54.1'
restore

replace_first "$C/NOTICE" 'AWS-LC 5.12.0' 'AWS-LC 0.47.0'
expect 55 guards 'W10' 'AWS-LC 5.12.0'
restore

replace_first "$C/NOTICE" 'libsqlite3-sys 0.40.0' 'libsqlite3-sys 0.39.0'
expect 56 guards 'W10' 'libsqlite3-sys' 'Cargo.lock'
restore

# The resolved thread restriction applies in integration-test compilation units too.
append "$C/crates/pool-control/tests/controller.rs" 'pub fn prove_test_grouped_thread() { use std::{thread}; let _ = thread::spawn(|| ()); }'
expect 57 clippy 'disallowed_methods' 'std::thread::spawn'
restore

# A non-private PEM header and binary prefix cannot hide a later private header.
printf '\377%s%s%s%s\n' '-----BEGIN ' 'CERTIFICATE----- ' '-----BEGIN ' 'PRIVATE KEY-----' >"$C/prove-multiple-headers.dat"
expect 58 guards 'W9' 'prove-multiple-headers.dat' 'private key block'
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
