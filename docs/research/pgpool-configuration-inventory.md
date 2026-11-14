# Pgpool configuration coverage inventory

**Checked:** 2026-11-14. Corpus: the `V4_7_3` release tagged [streaming sample](https://raw.githubusercontent.com/pgpool/pgpool2/V4_7_3/src/sample/pgpool.conf.sample-stream). The source review returned the names below. Indexed examples are preserved as separate spellings; they do not establish a maximum supported node count. The corpus describes Pgpool configuration.

Look up a sample spelling (CFG) to find its nearest POOL feature ID (F) and any mapping limit. A runtime declaration (RT) points to the corresponding sample entries or, for a source-only name, directly to its feature. One feature can account for multiple Pgpool spellings. The [feature matrix](../feature-matrix.md) owns each feature’s disposition, priority, milestone and acceptance. A mapping makes no claim that POOL accepts the Pgpool key. The feature spec defines any public TOML key, its units, default, validation and reload behavior.

| ID | Family | Baseline option | POOL feature IDs and mapping limits |
|---|---|---|---|
| CFG-001 | Mode | `backend_clustering_mode` | F-094, F-095, F-096, F-097, F-098 |
| CFG-002 | Connections and identity | `listen_addresses` | F-115 |
| CFG-003 | Connections and identity | `port` | F-115 |
| CFG-004 | Connections and identity | `unix_socket_directories` | F-115 |
| CFG-005 | Connections and identity | `unix_socket_group` | F-115 |
| CFG-006 | Connections and identity | `unix_socket_permissions` | F-115 |
| CFG-007 | Connections and identity | `reserved_connections` | F-028 |
| CFG-008 | Connections and identity | `pcp_listen_addresses` | F-105. Protected administration transport. PCP wire compatibility is deferred. |
| CFG-009 | Connections and identity | `pcp_port` | F-105. Protected administration transport. PCP wire compatibility is deferred. |
| CFG-010 | Connections and identity | `pcp_socket_dir` | F-105. Protected administration transport. PCP wire compatibility is deferred. |
| CFG-011 | Connections and identity | `listen_backlog_multiplier` | F-031 |
| CFG-012 | Connections and identity | `serialize_accept` | F-118. Bound admission in the async runtime. |
| CFG-013 | Connections and identity | `backend_hostname0` | F-061 |
| CFG-014 | Connections and identity | `backend_port0` | F-061 |
| CFG-015 | Connections and identity | `backend_weight0` | F-048 |
| CFG-016 | Connections and identity | `backend_data_directory0` | F-079. Data directory access belongs to the external recovery workflow. |
| CFG-017 | Connections and identity | `backend_flag0` | F-063, F-075. Pool flags cannot authorize promotion. |
| CFG-018 | Connections and identity | `backend_application_name0` | F-055, F-061. An application name can label a monitored connection. Verify node identity independently. |
| CFG-019 | Connections and identity | `backend_hostname1` | F-061 |
| CFG-020 | Connections and identity | `backend_port1` | F-061 |
| CFG-021 | Connections and identity | `backend_weight1` | F-048 |
| CFG-022 | Connections and identity | `backend_data_directory1` | F-079. Data directory access belongs to the external recovery workflow. |
| CFG-023 | Connections and identity | `backend_flag1` | F-063, F-075. Pool flags cannot authorize promotion. |
| CFG-024 | Connections and identity | `backend_application_name1` | F-055, F-061. An application name can label a monitored connection. Verify node identity independently. |
| CFG-025 | Connections and identity | `enable_pool_hba` | F-102 |
| CFG-026 | Connections and identity | `pool_passwd` | F-103 |
| CFG-027 | Connections and identity | `authentication_timeout` | F-107 |
| CFG-028 | Connections and identity | `allow_clear_text_frontend_auth` | F-100. The initial frontend profile accepts SCRAM over TLS or an approved client certificate. Reject cleartext password authentication. |
| CFG-029 | Connections and identity | `ssl` | F-101 |
| CFG-030 | Connections and identity | `ssl_key` | F-101 |
| CFG-031 | Connections and identity | `ssl_cert` | F-101 |
| CFG-032 | Connections and identity | `ssl_ca_cert` | F-101 |
| CFG-033 | Connections and identity | `ssl_ca_cert_dir` | F-101 |
| CFG-034 | Connections and identity | `ssl_crl_file` | F-101 |
| CFG-035 | Connections and identity | `ssl_ciphers` | F-101 |
| CFG-036 | Connections and identity | `ssl_prefer_server_ciphers` | F-101 |
| CFG-037 | Connections and identity | `ssl_ecdh_curve` | F-101 |
| CFG-038 | Connections and identity | `ssl_dh_params_file` | F-101 |
| CFG-039 | Connections and identity | `ssl_passphrase_command` | F-101, F-103. Use protected key material or a secret provider. A Pgpool shell command is outside the configuration contract. |
| CFG-040 | Connections and identity | `process_management_mode` | F-118 |
| CFG-041 | Connections and identity | `process_management_strategy` | F-118 |
| CFG-042 | Connections and identity | `num_init_children` | F-118 |
| CFG-043 | Connections and identity | `min_spare_children` | F-118 |
| CFG-044 | Connections and identity | `max_spare_children` | F-118 |
| CFG-045 | Connections and identity | `max_pool` | F-027, F-118 |
| CFG-046 | Connections and identity | `child_life_time` | F-118 |
| CFG-047 | Connections and identity | `child_max_connections` | F-118 |
| CFG-048 | Connections and identity | `connection_life_time` | F-029 |
| CFG-049 | Connections and identity | `client_idle_limit` | F-029 |
| CFG-050 | Logs files and reset | `log_destination` | F-114 |
| CFG-051 | Logs files and reset | `log_line_prefix` | F-114 |
| CFG-052 | Logs files and reset | `log_connections` | F-114 |
| CFG-053 | Logs files and reset | `log_disconnections` | F-114 |
| CFG-054 | Logs files and reset | `log_pcp_processes` | F-105, F-106 |
| CFG-055 | Logs files and reset | `log_hostname` | F-106, F-114 |
| CFG-056 | Logs files and reset | `log_statement` | F-106. Default diagnostics redact SQL, parameters and credentials. |
| CFG-057 | Logs files and reset | `log_per_node_statement` | F-106. Default diagnostics redact SQL, parameters and credentials. |
| CFG-058 | Logs files and reset | `notice_per_node_statement` | F-106. Default diagnostics redact SQL, parameters and credentials. |
| CFG-059 | Logs files and reset | `log_client_messages` | F-106. Default diagnostics redact SQL, parameters and credentials. |
| CFG-060 | Logs files and reset | `log_backend_messages` | F-106. Default diagnostics redact SQL, parameters and credentials. |
| CFG-061 | Logs files and reset | `log_standby_delay` | F-111 |
| CFG-062 | Logs files and reset | `syslog_facility` | F-114 |
| CFG-063 | Logs files and reset | `syslog_ident` | F-114 |
| CFG-064 | Logs files and reset | `log_error_verbosity` | F-114 |
| CFG-065 | Logs files and reset | `client_min_messages` | F-114 |
| CFG-066 | Logs files and reset | `log_min_messages` | F-114 |
| CFG-067 | Logs files and reset | `logging_collector` | F-114 |
| CFG-068 | Logs files and reset | `log_directory` | F-114 |
| CFG-069 | Logs files and reset | `log_filename` | F-114 |
| CFG-070 | Logs files and reset | `log_file_mode` | F-114 |
| CFG-071 | Logs files and reset | `log_truncate_on_rotation` | F-114 |
| CFG-072 | Logs files and reset | `log_rotation_age` | F-114 |
| CFG-073 | Logs files and reset | `log_rotation_size` | F-114 |
| CFG-074 | Logs files and reset | `pid_file_name` | F-154 |
| CFG-075 | Logs files and reset | `work_dir` | F-164. POOL defines its own state paths. |
| CFG-076 | Logs files and reset | `connection_cache` | F-023 |
| CFG-077 | Logs files and reset | `reset_query_list` | F-030. Only return a backend to the pool after validated cleanup. |
| CFG-078 | Replication and routing | `replicate_select` | F-097. Depends on the middleware replication mode. |
| CFG-079 | Replication and routing | `insert_lock` | F-097, F-099. Depends on the middleware replication mode. |
| CFG-080 | Replication and routing | `lobj_lock_table` | F-097, F-099. Depends on the middleware replication mode. |
| CFG-081 | Replication and routing | `replication_stop_on_mismatch` | F-097, F-099. Depends on the middleware replication mode. |
| CFG-082 | Replication and routing | `failover_if_affected_tuples_mismatch` | F-097, F-099. Depends on the middleware replication mode. |
| CFG-083 | Replication and routing | `load_balance_mode` | F-034, F-048 |
| CFG-084 | Replication and routing | `ignore_leading_white_space` | F-033 |
| CFG-085 | Replication and routing | `read_only_function_list` | F-038, F-039 |
| CFG-086 | Replication and routing | `write_function_list` | F-038, F-039 |
| CFG-087 | Replication and routing | `primary_routing_query_pattern_list` | F-045, F-047 |
| CFG-088 | Replication and routing | `user_redirect_preference_list` | F-046 |
| CFG-089 | Replication and routing | `database_redirect_preference_list` | F-046 |
| CFG-090 | Replication and routing | `app_name_redirect_preference_list` | F-046. Application names cannot authorize replica access. |
| CFG-091 | Replication and routing | `allow_sql_comments` | F-047 |
| CFG-092 | Replication and routing | `disable_load_balance_on_write` | F-044. A primary session stays on its primary lease after a write. |
| CFG-093 | Replication and routing | `dml_adaptive_object_relationship_list` | F-039, F-044. A primary session stays on its primary lease after a write. |
| CFG-094 | Replication and routing | `statement_level_load_balance` | F-052. A replica session keeps its initial lease. |
| CFG-095 | Streaming and health | `sr_check_period` | F-055, F-057 |
| CFG-096 | Streaming and health | `sr_check_user` | F-055 |
| CFG-097 | Streaming and health | `sr_check_password` | F-055, F-103 |
| CFG-098 | Streaming and health | `sr_check_database` | F-055 |
| CFG-099 | Streaming and health | `delay_threshold` | F-053, F-057 |
| CFG-100 | Streaming and health | `delay_threshold_by_time` | F-053, F-057. Fresh comparable byte lag screens admission. A time value cannot establish read freshness. |
| CFG-101 | Streaming and health | `prefer_lower_delay_standby` | F-053, F-057. Fresh comparable byte lag screens admission. A time value cannot establish read freshness. |
| CFG-102 | Streaming and health | `follow_primary_command` | F-076, F-079. Following a new primary needs a separate checked operation. |
| CFG-103 | Streaming and health | `health_check_period` | F-055, F-059 |
| CFG-104 | Streaming and health | `health_check_timeout` | F-055, F-059 |
| CFG-105 | Streaming and health | `health_check_user` | F-055 |
| CFG-106 | Streaming and health | `health_check_password` | F-055, F-103 |
| CFG-107 | Streaming and health | `health_check_database` | F-055 |
| CFG-108 | Streaming and health | `health_check_max_retries` | F-055, F-059 |
| CFG-109 | Streaming and health | `health_check_retry_delay` | F-055, F-059 |
| CFG-110 | Streaming and health | `connect_timeout` | F-055, F-059 |
| CFG-111 | Streaming and health | `health_check_period0` | F-055, F-059 |
| CFG-112 | Streaming and health | `health_check_timeout0` | F-055, F-059 |
| CFG-113 | Streaming and health | `health_check_user0` | F-055 |
| CFG-114 | Streaming and health | `health_check_password0` | F-055, F-103 |
| CFG-115 | Streaming and health | `health_check_database0` | F-055 |
| CFG-116 | Streaming and health | `health_check_max_retries0` | F-055, F-059 |
| CFG-117 | Streaming and health | `health_check_retry_delay0` | F-055, F-059 |
| CFG-118 | Streaming and health | `connect_timeout0` | F-055, F-059 |
| CFG-119 | Failover and recovery | `failover_command` | F-067, F-069. A health event cannot authorize the fenced manual operation. |
| CFG-120 | Failover and recovery | `failback_command` | F-063, F-076. Explicit validated rejoin. |
| CFG-121 | Failover and recovery | `failover_on_backend_error` | F-060, F-075. A failure can withdraw eligibility. It cannot authorize promotion. |
| CFG-122 | Failover and recovery | `failover_on_backend_shutdown` | F-060, F-075. A failure can withdraw eligibility. It cannot authorize promotion. |
| CFG-123 | Failover and recovery | `detach_false_primary` | F-065, F-066 |
| CFG-124 | Failover and recovery | `search_primary_node_timeout` | F-065, F-066 |
| CFG-125 | Failover and recovery | `recovery_user` | F-079, F-103 |
| CFG-126 | Failover and recovery | `recovery_password` | F-079, F-103 |
| CFG-127 | Failover and recovery | `recovery_database` | F-079 |
| CFG-128 | Failover and recovery | `recovery_1st_stage_command` | F-079. Use a scoped, structured adapter for checked recovery. |
| CFG-129 | Failover and recovery | `recovery_2nd_stage_command` | F-079. Use a scoped, structured adapter for checked recovery. |
| CFG-130 | Failover and recovery | `recovery_timeout` | F-079 |
| CFG-131 | Failover and recovery | `client_idle_limit_in_recovery` | F-079 |
| CFG-132 | Failover and recovery | `auto_failback` | F-080. Automatic failback requires its own decision. |
| CFG-133 | Failover and recovery | `auto_failback_interval` | F-080. Automatic failback requires its own decision. |
| CFG-134 | Watchdog | `use_watchdog` | F-081, F-082 |
| CFG-135 | Watchdog | `trusted_servers` | F-083 |
| CFG-136 | Watchdog | `trusted_server_command` | F-083. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-137 | Watchdog | `hostname0` | F-081, F-082 |
| CFG-138 | Watchdog | `wd_port0` | F-081, F-082 |
| CFG-139 | Watchdog | `pgpool_port0` | F-081, F-082 |
| CFG-140 | Watchdog | `hostname1` | F-081, F-082 |
| CFG-141 | Watchdog | `wd_port1` | F-081, F-082 |
| CFG-142 | Watchdog | `pgpool_port1` | F-081, F-082 |
| CFG-143 | Watchdog | `hostname2` | F-081, F-082 |
| CFG-144 | Watchdog | `wd_port2` | F-081, F-082 |
| CFG-145 | Watchdog | `pgpool_port2` | F-081, F-082 |
| CFG-146 | Watchdog | `wd_listen_address` | F-081, F-082 |
| CFG-147 | Watchdog | `wd_listen_port` | F-081, F-082 |
| CFG-148 | Watchdog | `wd_priority` | F-081, F-082 |
| CFG-149 | Watchdog | `wd_authkey` | F-081, F-082 |
| CFG-150 | Watchdog | `wd_ipc_socket_dir` | F-081, F-082 |
| CFG-151 | Watchdog | `delegate_ip` | F-085 |
| CFG-152 | Watchdog | `if_cmd_path` | F-085. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-153 | Watchdog | `if_up_cmd` | F-085. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-154 | Watchdog | `if_down_cmd` | F-085. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-155 | Watchdog | `arping_path` | F-085. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-156 | Watchdog | `arping_cmd` | F-085. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-157 | Watchdog | `ping_path` | F-083 |
| CFG-158 | Watchdog | `clear_memqcache_on_escalation` | F-088, F-093. The cache decision governs invalidation on an HA transition. |
| CFG-159 | Watchdog | `wd_escalation_command` | F-086. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-160 | Watchdog | `wd_de_escalation_command` | F-086. The baseline command is outside the API. Any adapter must accept only structured, allowlisted actions. |
| CFG-161 | Watchdog | `failover_when_quorum_exists` | F-070, F-075, F-082. Quorum alone cannot authorize database promotion. A half quorum rule is outside the proposed policy. |
| CFG-162 | Watchdog | `failover_require_consensus` | F-070, F-075, F-082. Quorum alone cannot authorize database promotion. A half quorum rule is outside the proposed policy. |
| CFG-163 | Watchdog | `allow_multiple_failover_requests_from_node` | F-070, F-075, F-082. Quorum alone cannot authorize database promotion. A half quorum rule is outside the proposed policy. |
| CFG-164 | Watchdog | `enable_consensus_with_half_votes` | F-070, F-075, F-082. Quorum alone cannot authorize database promotion. A half quorum rule is outside the proposed policy. |
| CFG-165 | Watchdog | `wd_remove_shutdown_nodes` | F-082 |
| CFG-166 | Watchdog | `wd_lost_node_removal_timeout` | F-082 |
| CFG-167 | Watchdog | `wd_no_show_node_removal_timeout` | F-082 |
| CFG-168 | Watchdog | `wd_monitoring_interfaces_list` | F-083 |
| CFG-169 | Watchdog | `wd_lifecheck_method` | F-083 |
| CFG-170 | Watchdog | `wd_interval` | F-083 |
| CFG-171 | Watchdog | `heartbeat_hostname0` | F-083 |
| CFG-172 | Watchdog | `heartbeat_port0` | F-083 |
| CFG-173 | Watchdog | `heartbeat_device0` | F-083 |
| CFG-174 | Watchdog | `heartbeat_hostname1` | F-083 |
| CFG-175 | Watchdog | `heartbeat_port1` | F-083 |
| CFG-176 | Watchdog | `heartbeat_device1` | F-083 |
| CFG-177 | Watchdog | `heartbeat_hostname2` | F-083 |
| CFG-178 | Watchdog | `heartbeat_port2` | F-083 |
| CFG-179 | Watchdog | `heartbeat_device2` | F-083 |
| CFG-180 | Watchdog | `wd_heartbeat_listen_addresses` | F-083 |
| CFG-181 | Watchdog | `wd_heartbeat_listen_port` | F-083 |
| CFG-182 | Watchdog | `wd_heartbeat_keepalive` | F-083 |
| CFG-183 | Watchdog | `wd_heartbeat_deadtime` | F-083 |
| CFG-184 | Watchdog | `wd_life_point` | F-083 |
| CFG-185 | Watchdog | `wd_lifecheck_query` | F-083 |
| CFG-186 | Watchdog | `wd_lifecheck_dbname` | F-083 |
| CFG-187 | Watchdog | `wd_lifecheck_user` | F-083 |
| CFG-188 | Watchdog | `wd_lifecheck_password` | F-083 |
| CFG-189 | Relation metadata | `relcache_expire` | F-119. Catalog metadata has separate ownership from cached query results. |
| CFG-190 | Relation metadata | `relcache_size` | F-119. Catalog metadata has separate ownership from cached query results. |
| CFG-191 | Relation metadata | `check_temp_table` | F-040, F-119 |
| CFG-192 | Relation metadata | `check_unlogged_table` | F-040, F-119 |
| CFG-193 | Relation metadata | `enable_shared_relcache` | F-119. Catalog metadata has separate ownership from cached query results. |
| CFG-194 | Relation metadata | `relcache_query_target` | F-119. Catalog metadata has separate ownership from cached query results. |
| CFG-195 | Query cache | `memory_cache_enabled` | F-088 |
| CFG-196 | Query cache | `memqcache_method` | F-092 |
| CFG-197 | Query cache | `memqcache_memcached_host` | F-092 |
| CFG-198 | Query cache | `memqcache_memcached_port` | F-092 |
| CFG-199 | Query cache | `memqcache_total_size` | F-090 |
| CFG-200 | Query cache | `memqcache_max_num_cache` | F-090 |
| CFG-201 | Query cache | `memqcache_expire` | F-090 |
| CFG-202 | Query cache | `memqcache_auto_cache_invalidation` | F-091 |
| CFG-203 | Query cache | `memqcache_maxcache` | F-090 |
| CFG-204 | Query cache | `memqcache_cache_block_size` | F-090 |
| CFG-205 | Query cache | `memqcache_oiddir` | F-091 |
| CFG-206 | Query cache | `cache_safe_memqcache_table_list` | F-089 |
| CFG-207 | Query cache | `cache_unsafe_memqcache_table_list` | F-089 |

## Coverage accounting

This transcription contains **207 distinct spellings**, matching the research scan of this sample. The grouped counts are mode 1, connections/identity 48, logs/reset 28, replication/routing 17, streaming/health 24, failover/recovery 15, Watchdog 55, relation metadata 6, and query cache 13. The separate runtime census below accounts for canonical declarations and aliases. Defaults, assign/check hooks, reload behavior and interactions still need executable qualification before configuration parity can be claimed.

Parameter interaction, reload rules, HBA/PCP password files, SSL build options and recovery examples extend beyond this streaming sample. The tagged sample manifest is in the baseline analysis. Source archive hash and tag commit hash were not verified during this phase. Capture and verify archive hashes before building a reference binary.

## Canonical runtime declaration census

A direct lexical census of the tagged [runtime declaration source](https://raw.githubusercontent.com/pgpool/pgpool2/V4_7_3/src/config/pool_config_variables.c) found **195 typed declaration records and 186 unique parameter spellings**. Nine names have two records, including eight global/indexed health settings and backend_flag. The 207 streaming-sample spellings normalize to 181 of these names. The five additional names are listed below. This counts source declarations, not compiled availability, defaults or behavior tests.

The fetched source had 6,518 lines. Its file SHA-256 was `9c510aaf5f2b7e24008c0ea8ed9d2aa25c7c4f29c6c2ccfb278866074243cae3`. This is a digest of that source file, not the release archive or Git tag object. It is the authority for this census where renderer extracts differed in line count.

| Additional name | Source detail |
|---|---|
| `delegate_IP` | Hidden alias for `delegate_ip`. |
| `fail_over_on_backend_error` | Hidden alias for `failover_on_backend_error`. |
| `health_check_test` | Health test control. |
| `logdir` | Hidden alias for `work_dir`. |
| `replication_mode` | Retained boolean alongside the mode enum. |

The runtime also constructs grouped backend, watchdog, heartbeat and health_check views. They are presentation/grouping interfaces, not four extra flat parameter declarations. The current mode enum has five values. Slony mode was retired in the 4.7 release line; no Slony compatibility is proposed.

| ID | Pgpool canonical spelling | Declared kind(s) | Declared context(s) | Sample coverage or source-only feature ID |
|---|---|---|---|---|
| RT-001 | `allow_clear_text_frontend_auth` | bool | CFGCXT_RELOAD | CFG-028 |
| RT-002 | `allow_multiple_failover_requests_from_node` | bool | CFGCXT_INIT | CFG-163 |
| RT-003 | `allow_sql_comments` | bool | CFGCXT_SESSION | CFG-091 |
| RT-004 | `app_name_redirect_preference_list` | string | CFGCXT_RELOAD | CFG-090 |
| RT-005 | `arping_cmd` | string | CFGCXT_INIT | CFG-156 |
| RT-006 | `arping_path` | string | CFGCXT_INIT | CFG-155 |
| RT-007 | `authentication_timeout` | int | CFGCXT_INIT | CFG-027 |
| RT-008 | `auto_failback` | bool | CFGCXT_RELOAD | CFG-132 |
| RT-009 | `auto_failback_interval` | int | CFGCXT_RELOAD | CFG-133 |
| RT-010 | `backend_application_name` | string_array | CFGCXT_RELOAD | CFG-018, CFG-024 |
| RT-011 | `backend_clustering_mode` | enum | CFGCXT_INIT | CFG-001 |
| RT-012 | `backend_data_directory` | string_array | CFGCXT_RELOAD | CFG-016, CFG-022 |
| RT-013 | `backend_flag` | string_array | CFGCXT_RELOAD | CFG-017, CFG-023 |
| RT-014 | `backend_hostname` | string_array | CFGCXT_RELOAD | CFG-013, CFG-019 |
| RT-015 | `backend_port` | int_array | CFGCXT_RELOAD | CFG-014, CFG-020 |
| RT-016 | `backend_weight` | double_array | CFGCXT_RELOAD | CFG-015, CFG-021 |
| RT-017 | `cache_safe_memqcache_table_list` | string_list | CFGCXT_RELOAD | CFG-206 |
| RT-018 | `cache_unsafe_memqcache_table_list` | string_list | CFGCXT_RELOAD | CFG-207 |
| RT-019 | `check_temp_table` | enum | CFGCXT_RELOAD | CFG-191 |
| RT-020 | `check_unlogged_table` | bool | CFGCXT_SESSION | CFG-192 |
| RT-021 | `child_life_time` | int | CFGCXT_INIT | CFG-046 |
| RT-022 | `child_max_connections` | int | CFGCXT_INIT | CFG-047 |
| RT-023 | `clear_memqcache_on_escalation` | bool | CFGCXT_RELOAD | CFG-158 |
| RT-024 | `client_idle_limit` | int | CFGCXT_SESSION | CFG-049 |
| RT-025 | `client_idle_limit_in_recovery` | int | CFGCXT_SESSION | CFG-131 |
| RT-026 | `client_min_messages` | enum | CFGCXT_SESSION | CFG-065 |
| RT-027 | `connect_timeout` | int_array | CFGCXT_RELOAD | CFG-110, CFG-118 |
| RT-028 | `connection_cache` | bool | CFGCXT_INIT | CFG-076 |
| RT-029 | `connection_life_time` | int | CFGCXT_INIT | CFG-048 |
| RT-030 | `database_redirect_preference_list` | string | CFGCXT_RELOAD | CFG-089 |
| RT-031 | `delay_threshold` | long | CFGCXT_RELOAD | CFG-099 |
| RT-032 | `delay_threshold_by_time` | int | CFGCXT_RELOAD | CFG-100 |
| RT-033 | `delegate_IP` | string | CFGCXT_INIT | Source-only delta (F-161) |
| RT-034 | `delegate_ip` | string | CFGCXT_INIT | CFG-151 |
| RT-035 | `detach_false_primary` | bool | CFGCXT_RELOAD | CFG-123 |
| RT-036 | `disable_load_balance_on_write` | enum | CFGCXT_RELOAD | CFG-092 |
| RT-037 | `dml_adaptive_object_relationship_list` | string | CFGCXT_RELOAD | CFG-093 |
| RT-038 | `enable_consensus_with_half_votes` | bool | CFGCXT_INIT | CFG-164 |
| RT-039 | `enable_pool_hba` | bool | CFGCXT_RELOAD | CFG-025 |
| RT-040 | `enable_shared_relcache` | bool | CFGCXT_INIT | CFG-193 |
| RT-041 | `fail_over_on_backend_error` | bool | CFGCXT_RELOAD | Source-only delta (F-162) |
| RT-042 | `failback_command` | string | CFGCXT_RELOAD | CFG-120 |
| RT-043 | `failover_command` | string | CFGCXT_RELOAD | CFG-119 |
| RT-044 | `failover_if_affected_tuples_mismatch` | bool | CFGCXT_RELOAD | CFG-082 |
| RT-045 | `failover_on_backend_error` | bool | CFGCXT_RELOAD | CFG-121 |
| RT-046 | `failover_on_backend_shutdown` | bool | CFGCXT_RELOAD | CFG-122 |
| RT-047 | `failover_require_consensus` | bool | CFGCXT_INIT | CFG-162 |
| RT-048 | `failover_when_quorum_exists` | bool | CFGCXT_INIT | CFG-161 |
| RT-049 | `follow_primary_command` | string | CFGCXT_RELOAD | CFG-102 |
| RT-050 | `health_check_database` | string_array | CFGCXT_RELOAD | CFG-107, CFG-115 |
| RT-051 | `health_check_max_retries` | int_array | CFGCXT_RELOAD | CFG-108, CFG-116 |
| RT-052 | `health_check_password` | string_array | CFGCXT_RELOAD | CFG-106, CFG-114 |
| RT-053 | `health_check_period` | int_array | CFGCXT_RELOAD | CFG-103, CFG-111 |
| RT-054 | `health_check_retry_delay` | int_array | CFGCXT_RELOAD | CFG-109, CFG-117 |
| RT-055 | `health_check_test` | bool | CFGCXT_INIT | Source-only delta (F-163) |
| RT-056 | `health_check_timeout` | int_array | CFGCXT_RELOAD | CFG-104, CFG-112 |
| RT-057 | `health_check_user` | string_array | CFGCXT_RELOAD | CFG-105, CFG-113 |
| RT-058 | `heartbeat_device` | string_array | CFGCXT_RELOAD | CFG-173, CFG-176, CFG-179 |
| RT-059 | `heartbeat_hostname` | string_array | CFGCXT_RELOAD | CFG-171, CFG-174, CFG-177 |
| RT-060 | `heartbeat_port` | int_array | CFGCXT_RELOAD | CFG-172, CFG-175, CFG-178 |
| RT-061 | `hostname` | string_array | CFGCXT_RELOAD | CFG-137, CFG-140, CFG-143 |
| RT-062 | `if_cmd_path` | string | CFGCXT_INIT | CFG-152 |
| RT-063 | `if_down_cmd` | string | CFGCXT_INIT | CFG-154 |
| RT-064 | `if_up_cmd` | string | CFGCXT_INIT | CFG-153 |
| RT-065 | `ignore_leading_white_space` | bool | CFGCXT_RELOAD | CFG-084 |
| RT-066 | `insert_lock` | bool | CFGCXT_RELOAD | CFG-079 |
| RT-067 | `listen_addresses` | string_list | CFGCXT_INIT | CFG-002 |
| RT-068 | `listen_backlog_multiplier` | int | CFGCXT_INIT | CFG-011 |
| RT-069 | `load_balance_mode` | bool | CFGCXT_INIT | CFG-083 |
| RT-070 | `lobj_lock_table` | string | CFGCXT_RELOAD | CFG-080 |
| RT-071 | `log_backend_messages` | enum | CFGCXT_SESSION | CFG-060 |
| RT-072 | `log_client_messages` | bool | CFGCXT_SESSION | CFG-059 |
| RT-073 | `log_connections` | bool | CFGCXT_RELOAD | CFG-052 |
| RT-074 | `log_destination` | string | CFGCXT_RELOAD | CFG-050 |
| RT-075 | `log_directory` | string | CFGCXT_RELOAD | CFG-068 |
| RT-076 | `log_disconnections` | bool | CFGCXT_RELOAD | CFG-053 |
| RT-077 | `log_error_verbosity` | enum | CFGCXT_SESSION | CFG-064 |
| RT-078 | `log_file_mode` | int | CFGCXT_RELOAD | CFG-070 |
| RT-079 | `log_filename` | string | CFGCXT_RELOAD | CFG-069 |
| RT-080 | `log_hostname` | bool | CFGCXT_RELOAD | CFG-055 |
| RT-081 | `log_line_prefix` | string | CFGCXT_RELOAD | CFG-051 |
| RT-082 | `log_min_messages` | enum | CFGCXT_SESSION | CFG-066 |
| RT-083 | `log_pcp_processes` | bool | CFGCXT_RELOAD | CFG-054 |
| RT-084 | `log_per_node_statement` | bool | CFGCXT_SESSION | CFG-057 |
| RT-085 | `log_rotation_age` | int | CFGCXT_RELOAD | CFG-072 |
| RT-086 | `log_rotation_size` | int | CFGCXT_RELOAD | CFG-073 |
| RT-087 | `log_standby_delay` | enum | CFGCXT_RELOAD | CFG-061 |
| RT-088 | `log_statement` | bool | CFGCXT_SESSION | CFG-056 |
| RT-089 | `log_truncate_on_rotation` | bool | CFGCXT_RELOAD | CFG-071 |
| RT-090 | `logdir` | string | CFGCXT_INIT | Source-only delta (F-164) |
| RT-091 | `logging_collector` | bool | CFGCXT_INIT | CFG-067 |
| RT-092 | `max_pool` | int | CFGCXT_INIT | CFG-045 |
| RT-093 | `max_spare_children` | int | CFGCXT_RELOAD | CFG-044 |
| RT-094 | `memory_cache_enabled` | bool | CFGCXT_INIT | CFG-195 |
| RT-095 | `memqcache_auto_cache_invalidation` | bool | CFGCXT_RELOAD | CFG-202 |
| RT-096 | `memqcache_cache_block_size` | int | CFGCXT_INIT | CFG-204 |
| RT-097 | `memqcache_expire` | int | CFGCXT_INIT | CFG-201 |
| RT-098 | `memqcache_max_num_cache` | int | CFGCXT_INIT | CFG-200 |
| RT-099 | `memqcache_maxcache` | int | CFGCXT_INIT | CFG-203 |
| RT-100 | `memqcache_memcached_host` | string | CFGCXT_INIT | CFG-197 |
| RT-101 | `memqcache_memcached_port` | int | CFGCXT_INIT | CFG-198 |
| RT-102 | `memqcache_method` | enum | CFGCXT_INIT | CFG-196 |
| RT-103 | `memqcache_oiddir` | string | CFGCXT_INIT | CFG-205 |
| RT-104 | `memqcache_total_size` | long | CFGCXT_INIT | CFG-199 |
| RT-105 | `min_spare_children` | int | CFGCXT_RELOAD | CFG-043 |
| RT-106 | `notice_per_node_statement` | bool | CFGCXT_SESSION | CFG-058 |
| RT-107 | `num_init_children` | int | CFGCXT_INIT | CFG-042 |
| RT-108 | `pcp_listen_addresses` | string_list | CFGCXT_INIT | CFG-008 |
| RT-109 | `pcp_port` | int | CFGCXT_INIT | CFG-009 |
| RT-110 | `pcp_socket_dir` | string_list | CFGCXT_INIT | CFG-010 |
| RT-111 | `pgpool_port` | int_array | CFGCXT_RELOAD | CFG-139, CFG-142, CFG-145 |
| RT-112 | `pid_file_name` | string | CFGCXT_INIT | CFG-074 |
| RT-113 | `ping_path` | string | CFGCXT_INIT | CFG-157 |
| RT-114 | `pool_passwd` | string | CFGCXT_INIT | CFG-026 |
| RT-115 | `port` | int | CFGCXT_INIT | CFG-003 |
| RT-116 | `prefer_lower_delay_standby` | bool | CFGCXT_RELOAD | CFG-101 |
| RT-117 | `primary_routing_query_pattern_list` | string_list | CFGCXT_RELOAD | CFG-087 |
| RT-118 | `process_management_mode` | enum | CFGCXT_RELOAD | CFG-040 |
| RT-119 | `process_management_strategy` | enum | CFGCXT_RELOAD | CFG-041 |
| RT-120 | `read_only_function_list` | string_list | CFGCXT_RELOAD | CFG-085 |
| RT-121 | `recovery_1st_stage_command` | string | CFGCXT_RELOAD | CFG-128 |
| RT-122 | `recovery_2nd_stage_command` | string | CFGCXT_RELOAD | CFG-129 |
| RT-123 | `recovery_database` | string | CFGCXT_RELOAD | CFG-127 |
| RT-124 | `recovery_password` | string | CFGCXT_RELOAD | CFG-126 |
| RT-125 | `recovery_timeout` | int | CFGCXT_RELOAD | CFG-130 |
| RT-126 | `recovery_user` | string | CFGCXT_RELOAD | CFG-125 |
| RT-127 | `relcache_expire` | long | CFGCXT_INIT | CFG-189 |
| RT-128 | `relcache_query_target` | enum | CFGCXT_RELOAD | CFG-194 |
| RT-129 | `relcache_size` | int | CFGCXT_INIT | CFG-190 |
| RT-130 | `replicate_select` | bool | CFGCXT_RELOAD | CFG-078 |
| RT-131 | `replication_mode` | bool | CFGCXT_INIT | Source-only delta (F-165) |
| RT-132 | `replication_stop_on_mismatch` | bool | CFGCXT_RELOAD | CFG-081 |
| RT-133 | `reserved_connections` | int | CFGCXT_INIT | CFG-007 |
| RT-134 | `reset_query_list` | string_list | CFGCXT_RELOAD | CFG-077 |
| RT-135 | `search_primary_node_timeout` | int | CFGCXT_RELOAD | CFG-124 |
| RT-136 | `serialize_accept` | bool | CFGCXT_INIT | CFG-012 |
| RT-137 | `sr_check_database` | string | CFGCXT_RELOAD | CFG-098 |
| RT-138 | `sr_check_password` | string | CFGCXT_RELOAD | CFG-097 |
| RT-139 | `sr_check_period` | int | CFGCXT_RELOAD | CFG-095 |
| RT-140 | `sr_check_user` | string | CFGCXT_RELOAD | CFG-096 |
| RT-141 | `ssl` | bool | CFGCXT_INIT | CFG-029 |
| RT-142 | `ssl_ca_cert` | string | CFGCXT_INIT | CFG-032 |
| RT-143 | `ssl_ca_cert_dir` | string | CFGCXT_INIT | CFG-033 |
| RT-144 | `ssl_cert` | string | CFGCXT_INIT | CFG-031 |
| RT-145 | `ssl_ciphers` | string | CFGCXT_INIT | CFG-035 |
| RT-146 | `ssl_crl_file` | string | CFGCXT_INIT | CFG-034 |
| RT-147 | `ssl_dh_params_file` | string | CFGCXT_INIT | CFG-038 |
| RT-148 | `ssl_ecdh_curve` | string | CFGCXT_INIT | CFG-037 |
| RT-149 | `ssl_key` | string | CFGCXT_INIT | CFG-030 |
| RT-150 | `ssl_passphrase_command` | string | CFGCXT_INIT | CFG-039 |
| RT-151 | `ssl_prefer_server_ciphers` | bool | CFGCXT_INIT | CFG-036 |
| RT-152 | `statement_level_load_balance` | bool | CFGCXT_RELOAD | CFG-094 |
| RT-153 | `syslog_facility` | enum | CFGCXT_RELOAD | CFG-062 |
| RT-154 | `syslog_ident` | string | CFGCXT_RELOAD | CFG-063 |
| RT-155 | `trusted_server_command` | string | CFGCXT_RELOAD | CFG-136 |
| RT-156 | `trusted_servers` | string | CFGCXT_INIT | CFG-135 |
| RT-157 | `unix_socket_directories` | string_list | CFGCXT_INIT | CFG-004 |
| RT-158 | `unix_socket_group` | string | CFGCXT_INIT | CFG-005 |
| RT-159 | `unix_socket_permissions` | int | CFGCXT_INIT | CFG-006 |
| RT-160 | `use_watchdog` | bool | CFGCXT_INIT | CFG-134 |
| RT-161 | `user_redirect_preference_list` | string | CFGCXT_RELOAD | CFG-088 |
| RT-162 | `wd_authkey` | string | CFGCXT_RELOAD | CFG-149 |
| RT-163 | `wd_de_escalation_command` | string | CFGCXT_RELOAD | CFG-160 |
| RT-164 | `wd_escalation_command` | string | CFGCXT_RELOAD | CFG-159 |
| RT-165 | `wd_heartbeat_deadtime` | int | CFGCXT_INIT | CFG-183 |
| RT-166 | `wd_heartbeat_keepalive` | int | CFGCXT_INIT | CFG-182 |
| RT-167 | `wd_heartbeat_listen_addresses` | string | CFGCXT_INIT | CFG-180 |
| RT-168 | `wd_heartbeat_listen_port` | int | CFGCXT_INIT | CFG-181 |
| RT-169 | `wd_interval` | int | CFGCXT_INIT | CFG-170 |
| RT-170 | `wd_ipc_socket_dir` | string | CFGCXT_INIT | CFG-150 |
| RT-171 | `wd_life_point` | int | CFGCXT_INIT | CFG-184 |
| RT-172 | `wd_lifecheck_dbname` | string | CFGCXT_RELOAD | CFG-186 |
| RT-173 | `wd_lifecheck_method` | enum | CFGCXT_INIT | CFG-169 |
| RT-174 | `wd_lifecheck_password` | string | CFGCXT_RELOAD | CFG-188 |
| RT-175 | `wd_lifecheck_query` | string | CFGCXT_INIT | CFG-185 |
| RT-176 | `wd_lifecheck_user` | string | CFGCXT_RELOAD | CFG-187 |
| RT-177 | `wd_listen_address` | string | CFGCXT_INIT | CFG-146 |
| RT-178 | `wd_listen_port` | int | CFGCXT_INIT | CFG-147 |
| RT-179 | `wd_lost_node_removal_timeout` | int | CFGCXT_RELOAD | CFG-166 |
| RT-180 | `wd_monitoring_interfaces_list` | string_list | CFGCXT_INIT | CFG-168 |
| RT-181 | `wd_no_show_node_removal_timeout` | int | CFGCXT_RELOAD | CFG-167 |
| RT-182 | `wd_port` | int_array | CFGCXT_RELOAD | CFG-138, CFG-141, CFG-144 |
| RT-183 | `wd_priority` | int | CFGCXT_INIT | CFG-148 |
| RT-184 | `wd_remove_shutdown_nodes` | bool | CFGCXT_RELOAD | CFG-165 |
| RT-185 | `work_dir` | string | CFGCXT_INIT | CFG-075 |
| RT-186 | `write_function_list` | string_list | CFGCXT_RELOAD | CFG-086 |

## Remaining behavioral qualification

The lexical runtime parameter census is accounted for. Every setting's accepted values, units, default, assign/check hooks, reload context, platform/build feature and runtime interaction still needs release-pinned behavioral qualification before configuration compatibility can be claimed. All other examples, historical aliases outside this table, extension SQL implementations, error paths and regression histories are not certified by this census. The 4.7.2 release-notes page could not be retrieved through the research renderer; its specific changes remain unverified. No full parity claim is made.

## Other configuration surfaces

The runtime table is not the entire configuration interface. HBA records select by transport, database, user and address before an authentication method and its options. PCP credentials and `.pcppass` are separate files; password file permissions and secret handling need their own tests. POOL will define a new authenticated administration contract rather than accept these formats automatically.

The [authentication reference](https://www.pgpool.net/docs/4.7/en/html/auth-methods.html) includes trust, password, MD5, SCRAM, certificate, PAM and LDAP behavior. LDAP adds `ldapserver`, `ldapport`, `ldapscheme`, `ldaptls`, `ldapprefix`, `ldapsuffix`, `ldapbasedn`, `ldapbinddn`, `ldapbindpasswd`, `ldapsearchattribute`, `ldapsearchfilter`, `ldapurl` and `backend_use_passwd`. These are HBA options, not extra flat runtime declarations. PAM and LDAP have optional build dependencies. Their POOL disposition is deferred; default deny access rules, SCRAM/TLS and explicit identity mapping come first. No legacy authentication fallback or copied password encryption format is planned for V1.
