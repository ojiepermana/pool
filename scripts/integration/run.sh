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
# How long the test command may run before it is ended, so that the cleanup
# always runs.
TEST_LIMIT_SECONDS=1800

# The trap is set before anything is made. Containers are removed by the ID
# that `docker run -d` printed. The test command and the watchdog are ended
# with every process under them, found by their parent process IDs: job
# control is not used, since dash turns it off without a terminal.
DIR=
IDS=
TEST_PID=
WATCHDOG=

# Prints "pid ppid" for every process: from ps where it exists, and from
# /proc on a Linux without ps (the pinned image has none).
process_table() {
  if command -v ps >/dev/null 2>&1; then
    ps -A -o pid= -o ppid=
  elif [ -r /proc/self/stat ]; then
    for stat in /proc/[0-9]*/stat; do
      line=$(cat "$stat" 2>/dev/null) || continue
      # The name in parentheses may hold spaces; the state and the parent
      # come after the last ") ".
      rest=${line##*) }
      set -- $rest
      echo "${line%% *} $2"
    done
  else
    return 1
  fi
}

# Prints $1 and every process under it.
process_tree() {
  table=$(process_table) || return 1
  printf '%s\n' "$table" | awk -v root="$1" '
    { parent[$1] = $2 }
    END {
      tree[root] = 1
      do {
        grown = 0
        for (pid in parent) {
          if (!(pid in tree) && (parent[pid] in tree)) { tree[pid] = 1; grown = 1 }
        }
      } while (grown)
      for (pid in tree) print pid
    }'
}

# Whether process $1 still runs: it exists and is not a zombie.
running() {
  if command -v ps >/dev/null 2>&1; then
    state=$(ps -o stat= -p "$1" 2>/dev/null) || return 1
  elif [ -r "/proc/$1/stat" ]; then
    line=$(cat "/proc/$1/stat" 2>/dev/null) || return 1
    rest=${line##*) }
    state=${rest%% *}
  else
    return 1
  fi
  case $state in
    "" | Z*) return 1 ;;
  esac
  return 0
}

# Ends $1 and every process under it. The tree is stopped first, until no
# new process appears in it, so that none escapes by its parent ending
# first. Then each gets SIGTERM, and after 5 seconds SIGKILL. Returns 1,
# after one line on stderr, when a process of the tree still runs.
end_tree() {
  [ -n "$1" ] || return 0
  if ! running "$1"; then
    wait "$1" 2>/dev/null || :
    return 0
  fi
  if ! command -v ps >/dev/null 2>&1 && [ ! -r /proc/self/stat ]; then
    echo "integration: neither ps nor /proc is there to end the processes of the tests" >&2
    return 1
  fi
  frozen=" "
  rounds=0
  while [ "$rounds" -lt 20 ]; do
    grown=no
    for pid in $(process_tree "$1"); do
      case $frozen in
        *" $pid "*) ;;
        *) kill -STOP "$pid" 2>/dev/null && frozen="$frozen$pid "
           grown=yes ;;
      esac
    done
    [ "$grown" = no ] && break
    rounds=$((rounds + 1))
  done
  for pid in $frozen; do kill -TERM "$pid" 2>/dev/null || :; done
  for pid in $frozen; do kill -CONT "$pid" 2>/dev/null || :; done
  wait_ended 5 || {
    for pid in $left; do kill -KILL "$pid" 2>/dev/null || :; done
    wait_ended 2
  } || {
    echo "integration: process$left of the tests could not be ended" >&2
    return 1
  }
  wait "$1" 2>/dev/null || :
  return 0
}

# Asks every second, for at most $1 seconds, whether a process of $frozen
# still runs. Leaves the ones that do in $left.
wait_ended() {
  waited=0
  while :; do
    left=
    for pid in $frozen; do
      running "$pid" && left="$left $pid"
    done
    [ -z "$left" ] && return 0
    [ "$waited" -ge "$1" ] && return 1
    sleep 1
    waited=$((waited + 1))
  done
}

cleanup() {
  ended=0
  end_tree "$WATCHDOG" || ended=1
  end_tree "$TEST_PID" || ended=1
  if [ -n "$IDS" ]; then
    docker rm -f -v $IDS >/dev/null 2>&1 || :
  fi
  if [ -n "$DIR" ]; then
    rm -rf "$DIR"
  fi
  if [ "$ended" -ne 0 ]; then
    exit 1
  fi
}
trap cleanup EXIT
trap 'exit 1' INT TERM HUP

DIR=$(mktemp -d "${TMPDIR:-/tmp}/pool-it.XXXXXX")
chmod 700 "$DIR"

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
  id=$(docker run -d --name "$name" \
    -p 127.0.0.1::5432 \
    --add-host=host.docker.internal:host-gateway \
    -e POSTGRES_PASSWORD="$SUPERUSER_PASSWORD" \
    -e POOL_IT_MONITOR_PASSWORD="$MONITOR_PASSWORD" \
    -e POOL_IT_RW_PASSWORD="$RW_PASSWORD" \
    -v "$ROOT/scripts/integration/initdb:/docker-entrypoint-initdb.d:ro" \
    -v "$DIR/certs:/certs:ro" \
    "$POOL_IT_IMAGE") || fail "container $name did not start"
  IDS="$IDS $id"
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
[ -n "$LISTEN_HOST" ] || fail "the address for the listener could not be read"

export POOL_IT_CONTAINER="$CONTAINER_1"
export POOL_IT_CONTAINER_2="$CONTAINER_2"
export POOL_IT_PG_PORT="$PORT_1"
export POOL_IT_PG_PORT_2="$PORT_2"
export POOL_IT_DIR="$DIR"
export POOL_IT_LISTEN_HOST="$LISTEN_HOST"
export POOL_IT_CLIENT_HOST=host.docker.internal

# 6. The tests, in the background, so that a signal to this script runs the
#    trap at once, and a watchdog that ends them after TEST_LIMIT_SECONDS.
#    Their output is kept to count the tests. POOL_IT_FILTER, when set, is
#    passed on as the filter of test names, to run one test again and again;
#    a filter that matches none fails below.
OUT="$DIR/test-output.txt"
if [ -n "${POOL_IT_FILTER:-}" ]; then
  echo "integration: POOL_IT_FILTER is set, only matching tests run" >&2
fi
sh -c '
  out=$1
  shift
  { cargo test -p pool --locked --test integration -- --ignored --test-threads=1 "$@"; echo "$?" > "$out.rc"; } 2>&1 | tee "$out"
  exit "$(cat "$out.rc" 2>/dev/null || echo 1)"
' sh "$OUT" ${POOL_IT_FILTER:+"$POOL_IT_FILTER"} &
TEST_PID=$!
(
  sleep "$TEST_LIMIT_SECONDS"
  echo "integration: the tests did not end within $TEST_LIMIT_SECONDS seconds" >&2
  end_tree "$TEST_PID"
) </dev/null >/dev/null &
WATCHDOG=$!
rc=0
wait "$TEST_PID" || rc=$?
TEST_PID=
end_tree "$WATCHDOG" || fail "the watchdog could not be ended"
WATCHDOG=
if [ "$rc" -ne 0 ]; then
  exit 1
fi
# A run in which no test passed is a failure: a filter or a build that runs
# nothing must not look green.
passed=$(sed -n 's/^test result: [A-Za-z]*\. \([0-9][0-9]*\) passed.*/\1/p' "$OUT" | awk '{ n += $1 } END { print n + 0 }')
if [ "$passed" -eq 0 ]; then
  fail "no test ran"
fi
exit 0
