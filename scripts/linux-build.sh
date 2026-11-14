#!/bin/sh
# Linux build check of spec 0001 (0001 AC-2). Needs a running Docker daemon.
# The command below the marker line is copied from spec 0001, "Linux build". Do not edit it here.
set -eu
cd "$(dirname "$0")/.."
echo "runner: $(uname -sm)"
if ! docker info >/dev/null 2>&1; then
  echo "linux-build: the Docker daemon is not running" >&2
  exit 3
fi
if [ -n "$(git status --porcelain)" ]; then
  echo "linux-build: commit or remove local changes first; this check builds only the committed tree" >&2
  exit 3
fi
# --- spec 0001, Linux build ---
git archive --format=tar HEAD | docker run --rm -i --platform linux/arm64 \
  -e RUSTUP_TOOLCHAIN=1.100.0 -e CARGO_INCREMENTAL=0 -e CARGO_PROFILE_DEV_DEBUG=0 \
  rust:1.100.0-slim-bookworm@sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37 \
  sh -euc '
mkdir /work && tar -xf - -C /work && cd /work
grep -q "channel = \"1.100.0\"" rust-toolchain.toml
rustc -vV
rustc -vV | grep -qx "host: aarch64-unknown-linux-gnu"
cargo build --locked --release -p pool
cargo test --locked --workspace
sha256sum target/release/pool
./target/release/pool --version
for f in crates/pool-config/tests/fixtures/*.toml; do
  case "$f" in
    */valid-*) want=0 ;;
    *) want=1 ;;
  esac
  for m in human json; do
    rc=0
    ./target/release/pool check-config --config "$f" --format "$m" >/tmp/out 2>/tmp/err || rc=$?
    echo "== $f $m exit=$rc"
    if grep -q "CANARY-9f2c7b1e" /tmp/out /tmp/err; then echo "canary leaked"; exit 1; fi
    cat /tmp/out /tmp/err
    if [ "$rc" -ne "$want" ]; then echo "unexpected exit $rc, wanted $want"; exit 1; fi
  done
done
echo "linux build check passed"
'
