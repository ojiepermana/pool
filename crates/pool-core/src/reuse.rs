//! Bounded, quarantined PostgreSQL cleanup. No result from these queries reaches a client.
use std::collections::HashMap;

use pool_protocol::auth::{self, TransactionStatus};
use pool_protocol::message::encode_query;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::backend::{BackendConnection, BackendError, data_row};
use crate::wire::{read_message, write_all};

/// The single settings and trusted-server qualification query used at creation and every handoff.
pub const BASELINE_QUERY: &str = "SELECT pg_catalog.current_setting('application_name'), pg_catalog.current_setting('client_encoding'), pg_catalog.current_setting('DateStyle'), pg_catalog.current_setting('IntervalStyle'), pg_catalog.current_setting('TimeZone'), pg_catalog.current_setting('standard_conforming_strings'), pg_catalog.current_setting('default_transaction_read_only'), pg_catalog.current_setting('search_path'), session_user::pg_catalog.text, current_user::pg_catalog.text, CASE WHEN (NOT EXISTS (SELECT 1 FROM pg_catalog.pg_extension WHERE extname::pg_catalog.text OPERATOR(pg_catalog.<>) 'plpgsql') AND NOT EXISTS (SELECT 1 FROM pg_catalog.pg_event_trigger) AND pg_catalog.current_setting('shared_preload_libraries') OPERATOR(pg_catalog.=) '' AND pg_catalog.current_setting('session_preload_libraries') OPERATOR(pg_catalog.=) '' AND pg_catalog.current_setting('local_preload_libraries') OPERATOR(pg_catalog.=) '') THEN 't' ELSE 'f' END";
const REQUIRED: &[&[u8]] = &[
    b"server_version",
    b"server_encoding",
    b"client_encoding",
    b"application_name",
    b"is_superuser",
    b"session_authorization",
    b"DateStyle",
    b"IntervalStyle",
    b"TimeZone",
    b"search_path",
    b"integer_datetimes",
    b"standard_conforming_strings",
    b"default_transaction_read_only",
    b"in_hot_standby",
];

pub(crate) fn startup_eligible<S>(backend: &BackendConnection<S>) -> bool {
    let mut seen = HashMap::new();
    backend.parameters.iter().all(|p| {
        (REQUIRED.contains(&p.name.as_slice()) || p.name == b"scram_iterations")
            && seen.insert(p.name.as_slice(), ()).is_none()
    }) && REQUIRED.iter().all(|name| seen.contains_key(name))
}

fn startup_matches_row<S>(backend: &BackendConnection<S>, row: &[String]) -> bool {
    [
        "application_name",
        "client_encoding",
        "DateStyle",
        "IntervalStyle",
        "TimeZone",
        "standard_conforming_strings",
        "default_transaction_read_only",
        "search_path",
    ]
    .iter()
    .enumerate()
    .all(|(index, name)| {
        backend
            .parameters
            .iter()
            .any(|p| p.name == name.as_bytes() && p.value == row[index].as_bytes())
    }) && backend
        .parameters
        .iter()
        .any(|p| p.name == b"session_authorization" && p.value == row[8].as_bytes())
}

/// One cumulative reset budget, including all headers and all commands.
#[derive(Default)]
struct Budget {
    bytes: usize,
    messages: usize,
    changed: HashMap<Vec<u8>, Vec<u8>>,
}

impl Budget {
    async fn read<S: AsyncRead + Unpin>(
        &mut self,
        stream: &mut S,
    ) -> Result<pool_protocol::message::Message, BackendError> {
        if self.messages >= 256 || self.bytes + 5 > 65536 {
            return Err(BackendError::Protocol);
        }
        let max = (65536 - self.bytes - 5).min(16384) as u32;
        let message = read_message(stream, max).await?;
        self.messages += 1;
        self.bytes += message.body.len() + 5;
        Ok(message)
    }
    fn parameter<S>(
        &mut self,
        backend: &BackendConnection<S>,
        body: &[u8],
    ) -> Result<(), BackendError> {
        let parameter = auth::parse_parameter_status(body).map_err(|_| BackendError::Protocol)?;
        if !backend.parameters.iter().any(|p| p.name == parameter.name) {
            return Err(BackendError::Protocol);
        }
        self.changed.insert(parameter.name, parameter.value);
        Ok(())
    }
    fn restored<S>(&self, backend: &BackendConnection<S>) -> bool {
        self.changed.iter().all(|(name, value)| {
            backend
                .parameters
                .iter()
                .any(|p| p.name == *name && p.value == *value)
        })
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> BackendConnection<S> {
    pub(crate) async fn qualify_reuse(
        &mut self,
        login: &str,
    ) -> Result<Option<Vec<String>>, BackendError> {
        if !startup_eligible(self) {
            return Ok(None);
        }
        let mut budget = Budget::default();
        let row = self.baseline_row(&mut budget).await?;
        Ok(row.filter(|row| {
            row[1] == "UTF8"
                && row[5] == "on"
                && row[6] == "off"
                && row[8] == login
                && row[9] == login
                && row[10] == "t"
                && matches!(
                    row[7].as_str(),
                    "\"$user\", public" | "pg_catalog" | "public" | "\"$user\""
                )
                && budget.restored(self)
                && startup_matches_row(self, row)
        }))
    }

    pub(crate) async fn validate_reuse(&mut self, expected: &[String]) -> Result<(), BackendError> {
        let mut budget = Budget::default();
        let row = self.baseline_row(&mut budget).await?;
        if row.as_deref() == Some(expected) && budget.restored(self) {
            Ok(())
        } else {
            Err(BackendError::Protocol)
        }
    }

    pub(crate) async fn cleanup_reuse(&mut self, expected: &[String]) -> Result<(), BackendError> {
        let mut budget = Budget::default();
        if self.status != TransactionStatus::Idle {
            self.reset_command("ROLLBACK", b"ROLLBACK\0", false, &mut budget)
                .await?;
        }
        self.reset_command("DISCARD ALL", b"DISCARD ALL\0", true, &mut budget)
            .await?;
        let row = self.baseline_row(&mut budget).await?;
        if row.as_deref() == Some(expected) && budget.restored(self) {
            Ok(())
        } else {
            Err(BackendError::Protocol)
        }
    }

    async fn reset_command(
        &mut self,
        sql: &str,
        completion: &[u8],
        discard: bool,
        budget: &mut Budget,
    ) -> Result<(), BackendError> {
        write_all(&mut self.stream, &encode_query(sql)).await?;
        let mut completed = false;
        loop {
            let message = budget.read(&mut self.stream).await?;
            match message.tag {
                b'C' if !completed && message.body == completion => completed = true,
                b'N' => notice(&message.body)?,
                b'A' if discard => {
                    notification(&message.body)?;
                }
                b'S' if discard => budget.parameter(self, &message.body)?,
                b'Z' if completed => {
                    self.status = auth::parse_ready_for_query(&message.body)
                        .map_err(|_| BackendError::Protocol)?;
                    return if self.status == TransactionStatus::Idle {
                        Ok(())
                    } else {
                        Err(BackendError::NotIdle)
                    };
                }
                _ => return Err(BackendError::Protocol),
            }
        }
    }

    async fn baseline_row(
        &mut self,
        budget: &mut Budget,
    ) -> Result<Option<Vec<String>>, BackendError> {
        write_all(&mut self.stream, &encode_query(BASELINE_QUERY)).await?;
        let mut description = false;
        let mut row = None;
        let mut complete = false;
        let mut error = false;
        loop {
            let message = budget.read(&mut self.stream).await?;
            match message.tag {
                b'T' if !description && row.is_none() && !complete => {
                    validate_description(&message.body)?;
                    description = true;
                }
                b'D' if description && row.is_none() && !complete => {
                    let values = data_row(&message.body)?;
                    if values.len() != 11 {
                        return Err(BackendError::Protocol);
                    }
                    row = Some(values.into_iter().collect::<Option<Vec<_>>>()).flatten();
                    if row.is_none() {
                        error = true;
                    }
                }
                b'C' if description && !complete && message.body == b"SELECT 1\0" => {
                    complete = true
                }
                b'E' if !complete && !error => {
                    notice(&message.body)?;
                    error = true;
                }
                b'N' => notice(&message.body)?,
                b'S' => budget.parameter(self, &message.body)?,
                b'Z' if complete || error => {
                    self.status = auth::parse_ready_for_query(&message.body)
                        .map_err(|_| BackendError::Protocol)?;
                    if self.status != TransactionStatus::Idle {
                        return Err(BackendError::NotIdle);
                    }
                    return Ok(if error { None } else { row });
                }
                _ => return Err(BackendError::Protocol),
            }
        }
    }
}

fn validate_description(body: &[u8]) -> Result<(), BackendError> {
    if body.get(..2) != Some(&11u16.to_be_bytes()) {
        return Err(BackendError::Protocol);
    }
    let mut rest = &body[2..];
    for _ in 0..11 {
        let nul = rest
            .iter()
            .position(|b| *b == 0)
            .ok_or(BackendError::Protocol)?;
        std::str::from_utf8(&rest[..nul]).map_err(|_| BackendError::Protocol)?;
        rest = rest.get(nul + 1..).ok_or(BackendError::Protocol)?;
        let fields = rest.get(..18).ok_or(BackendError::Protocol)?;
        if fields[6..10] != 25u32.to_be_bytes() || fields[16..18] != [0, 0] {
            return Err(BackendError::Protocol);
        }
        rest = &rest[18..];
    }
    if rest.is_empty() {
        Ok(())
    } else {
        Err(BackendError::Protocol)
    }
}

pub(crate) fn notification(body: &[u8]) -> Result<(), BackendError> {
    let rest = body.get(4..).ok_or(BackendError::Protocol)?;
    let mut fields = rest.split(|b| *b == 0);
    match (fields.next(), fields.next(), fields.next(), fields.next()) {
        (Some(channel), Some(payload), Some(empty), None)
            if !channel.is_empty()
                && empty.is_empty()
                && std::str::from_utf8(channel).is_ok()
                && std::str::from_utf8(payload).is_ok() =>
        {
            Ok(())
        }
        _ => Err(BackendError::Protocol),
    }
}

/// PostgreSQL error/notice field framing, without retaining or reporting peer text.
pub(crate) fn notice(mut body: &[u8]) -> Result<(), BackendError> {
    let mut seen = [false; 256];
    while let Some((&tag, tail)) = body.split_first() {
        if tag == 0 {
            return if tail.is_empty() && seen[usize::from(b'S')] && seen[usize::from(b'M')] {
                Ok(())
            } else {
                Err(BackendError::Protocol)
            };
        }
        if !b"SVCMDHPpqWstcdnFLR".contains(&tag) || seen[usize::from(tag)] {
            return Err(BackendError::Protocol);
        }
        seen[usize::from(tag)] = true;
        let nul = tail
            .iter()
            .position(|b| *b == 0)
            .ok_or(BackendError::Protocol)?;
        std::str::from_utf8(&tail[..nul]).map_err(|_| BackendError::Protocol)?;
        body = &tail[nul + 1..];
    }
    Err(BackendError::Protocol)
}
