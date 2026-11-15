#!/bin/sh
# The integration check of spec 0004, "The integration check". Called by
# scripts/check.sh integration, which passes the pinned image in POOL_IT_IMAGE.
#
# It starts two PostgreSQL containers from that image, runs the tests of
# apps/pool/tests/integration.rs against them and removes every container and
# the temporary directory it made, on every exit. The tests contact nothing
# else: they read the addresses from the environment set below.
#
# Exit status: 0 the tests passed, 1 they failed or the fixture could not be
# set up, 3 no Docker daemon.
set -eu

: "${POOL_IT_IMAGE:?POOL_IT_IMAGE is set by scripts/check.sh}"

CDPATH= cd -- "$(dirname -- "$0")/../.."
ROOT=$(pwd)

if ! docker info >/dev/null 2>&1; then
  echo "integration: the Docker daemon is not running; start it and run scripts/check.sh integration again" >&2
  exit 3
fi

# Test values of the fixture. They are used nowhere else.
SUPERUSER_PASSWORD=pz7-superuser-not-a-secret
MONITOR_PASSWORD=pz7-monitor-not-a-secret
RW_PASSWORD=pz7-rw-not-a-secret
PRINCIPAL_PASSWORD=pz7-principal-not-a-secret

CONTAINER_1="pool-it-1-$$"
CONTAINER_2="pool-it-2-$$"
DIR=$(mktemp -d "${TMPDIR:-/tmp}/pool-it.XXXXXX")
chmod 700 "$DIR"

cleanup() {
  docker rm -f -v "$CONTAINER_1" "$CONTAINER_2" >/dev/null 2>&1 || :
  rm -rf "$DIR"
}
trap cleanup EXIT
trap 'exit 1' INT TERM HUP

fail() {
  echo "integration: $1" >&2
  exit 1
}

# 1. The certificates of the fixture, once, with openssl of the same image:
#    a test CA, the server certificate for pg.pool.test (both containers get
#    the same one) and the certificate of POOL for host.docker.internal.
mkdir -p "$DIR/certs"
chmod 755 "$DIR/certs"
docker run --rm --user "$(id -u):$(id -g)" -v "$DIR/certs:/certs" --entrypoint sh "$POOL_IT_IMAGE" -c '
  set -eu
  cd /certs
  openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes -days 2 \
    -subj "/CN=pz7 test ca" -keyout ca.key -out ca.pem 2>/dev/null
  for name in pg.pool.test host.docker.internal; do
    openssl req -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 -nodes \
      -subj "/CN=$name" -keyout "$name.key" -out "$name.csr" 2>/dev/null
    printf "subjectAltName=DNS:%s\nbasicConstraints=CA:FALSE\nextendedKeyUsage=serverAuth\n" "$name" > "$name.ext"
    openssl x509 -req -in "$name.csr" -CA ca.pem -CAkey ca.key -CAcreateserial -days 2 \
      -extfile "$name.ext" -out "$name.pem" 2>/dev/null
  done
  cp pg.pool.test.pem server.pem
  cp pg.pool.test.key server.key
  rm -f ./*.csr ./*.ext ./*.srl ca.key
  chmod 644 ./*
' || fail "the certificates could not be made"

# 2. Two servers from the same image, with the same certificate, roles and
#    passwords. Only the system identifier differs, which initdb makes.
for name in "$CONTAINER_1" "$CONTAINER_2"; do
  docker run -d --name "$name" \
    -p 127.0.0.1::5432 \
    --add-host=host.docker.internal:host-gateway \
    -e POSTGRES_PASSWORD="$SUPERUSER_PASSWORD" \
    -e POOL_IT_MONITOR_PASSWORD="$MONITOR_PASSWORD" \
    -e POOL_IT_RW_PASSWORD="$RW_PASSWORD" \
    -v "$ROOT/scripts/integration/initdb:/docker-entrypoint-initdb.d:ro" \
    -v "$DIR/certs:/certs:ro" \
    "$POOL_IT_IMAGE" >/dev/null || fail "container $name did not start"
done

# 3. Wait until both answer over TCP, which they do only after the init
#    scripts ran and the real server started.
for name in "$CONTAINER_1" "$CONTAINER_2"; do
  tries=0
  until docker exec "$name" pg_isready -q -h 127.0.0.1 -p 5432 >/dev/null 2>&1; do
    tries=$((tries + 1))
    if [ "$tries" -ge 120 ]; then
      docker logs "$name" >&2 2>&1 || :
      fail "container $name was not ready after 60 seconds"
    fi
    sleep 0.5
  done
done

port_of() {
  docker port "$1" 5432/tcp | head -n 1 | sed 's/.*://'
}
PORT_1=$(port_of "$CONTAINER_1")
PORT_2=$(port_of "$CONTAINER_2")
[ -n "$PORT_1" ] && [ -n "$PORT_2" ] || fail "the published ports could not be read"

# 4. The secret files, mode 0600. The verifier of the principal comes from a
#    role made with NOLOGIN and dropped at once, so the principal can log in
#    to PostgreSQL under no name.
umask 077
printf '%s\n' "$MONITOR_PASSWORD" > "$DIR/monitor.password"
printf '%s\n' "$RW_PASSWORD" > "$DIR/rw.password"
# The password the client of the walking fixture types. POOL never reads it.
printf '%s\n' "$PRINCIPAL_PASSWORD" > "$DIR/principal.password"
docker exec "$CONTAINER_1" psql -q -v ON_ERROR_STOP=1 -U postgres -d postgres \
  -c "CREATE ROLE pz7_verifier_source NOLOGIN PASSWORD '$PRINCIPAL_PASSWORD'" >/dev/null \
  || fail "the verifier role could not be made"
docker exec "$CONTAINER_1" psql -At -v ON_ERROR_STOP=1 -U postgres -d postgres \
  -c "SELECT rolpassword FROM pg_authid WHERE rolname = 'pz7_verifier_source'" > "$DIR/principal.verifier" \
  || fail "the verifier could not be read"
docker exec "$CONTAINER_1" psql -q -v ON_ERROR_STOP=1 -U postgres -d postgres \
  -c "DROP ROLE pz7_verifier_source" >/dev/null || fail "the verifier role could not be dropped"
chmod 600 "$DIR/monitor.password" "$DIR/rw.password" "$DIR/principal.password" "$DIR/principal.verifier"
cp "$DIR/certs/host.docker.internal.key" "$DIR/pool.key"
chmod 600 "$DIR/pool.key"
umask 022

# 5. The addresses for the tests.
case "$(docker info --format '{{.OperatingSystem}}' 2>/dev/null)" in
  *"Docker Desktop"*) LISTEN_HOST=127.0.0.1 ;;
  *) LISTEN_HOST=$(docker network inspect bridge --format '{{(index .IPAM.Config 0).Gateway}}') ;;
esac

export POOL_IT_CONTAINER="$CONTAINER_1"
export POOL_IT_CONTAINER_2="$CONTAINER_2"
export POOL_IT_PG_PORT="$PORT_1"
export POOL_IT_PG_PORT_2="$PORT_2"
export POOL_IT_DIR="$DIR"
export POOL_IT_LISTEN_HOST="$LISTEN_HOST"
export POOL_IT_CLIENT_HOST=host.docker.internal

rc=0
cargo test -p pool --locked --test integration -- --ignored --test-threads=1 || rc=$?
if [ "$rc" -ne 0 ]; then
  exit 1
fi
exit 0
