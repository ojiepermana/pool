#!/bin/sh
# Init script of the integration fixture of spec 0004, "The integration check".
# The official postgres image runs it once, as the database owner, with a
# temporary server on the local socket. The settings below take effect when
# the image starts the real server afterwards. Every name and password here
# is a test value of scripts/integration/ and is used nowhere else.
set -eu

# TLS with the certificate of the fixture, copied so that the key belongs to
# the database owner with mode 0600.
cp /certs/server.pem "$PGDATA/server.crt"
cp /certs/server.key "$PGDATA/server.key"
chmod 600 "$PGDATA/server.key"

cat >> "$PGDATA/postgresql.conf" <<'CONF'
ssl = on
ssl_cert_file = 'server.crt'
ssl_key_file = 'server.key'
password_encryption = 'scram-sha-256'
log_connections = on
log_statement = 'all'
CONF

# Only TLS with SCRAM from the network. The local socket stays for the image
# and for docker exec.
cat > "$PGDATA/pg_hba.conf" <<'HBA'
local   all  all                 trust
hostssl all  all  0.0.0.0/0      scram-sha-256
hostssl all  all  ::/0           scram-sha-256
HBA

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname postgres \
  -v monitor_password="$POOL_IT_MONITOR_PASSWORD" \
  -v rw_password="$POOL_IT_RW_PASSWORD" <<'SQL'
CREATE ROLE pz7_monitor LOGIN PASSWORD :'monitor_password';
CREATE ROLE pz7_rw LOGIN PASSWORD :'rw_password';
CREATE DATABASE pz7_appdb;
REVOKE ALL ON DATABASE pz7_appdb FROM PUBLIC;
GRANT CONNECT ON DATABASE pz7_appdb TO pz7_rw;
\connect pz7_appdb
REVOKE ALL ON SCHEMA public FROM PUBLIC;
GRANT USAGE ON SCHEMA public TO pz7_rw;
CREATE TABLE pz7_items (key integer PRIMARY KEY, value text NOT NULL);
ALTER TABLE pz7_items OWNER TO pz7_rw;
SQL
