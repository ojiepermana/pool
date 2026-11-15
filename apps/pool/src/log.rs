//! The log writer of the daemon (spec 0004, "Log events").
//!
//! A first party `tracing::Subscriber`: each event becomes one JSON line,
//! spans are ignored. The subscriber never writes the line itself. It puts
//! it into a bounded queue of 1024 lines with `try_send`, and one task takes
//! lines from the queue and writes them to stderr. When the queue is full
//! the line is dropped and counted, and the next line that fits is preceded
//! by the event `log.dropped` with the count.
//!
//! A line is one JSON object: `ts`, `level` and `event` first, in that order,
//! then the fields in the order the code gave them. A call without `event`
//! is written as `log.unnamed` with the level `error`. Events below `info`
//! are not written.

use std::fmt::{self, Write as _};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, oneshot};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Level, Metadata, Subscriber};

/// The number of lines the queue holds.
pub const QUEUE_LINES: usize = 1024;

/// What the writer task receives.
enum Item {
    /// One finished line, with its line feed.
    Line(String),
    /// Everything sent before has been written; tell the sender.
    Flush(oneshot::Sender<()>),
}

/// The subscriber: it turns events into lines and queues them.
pub struct JsonLog {
    queue: mpsc::Sender<Item>,
    dropped: AtomicU64,
    next_span: AtomicUsize,
}

/// The handle that waits until every queued line was written.
pub struct LogHandle {
    queue: mpsc::Sender<Item>,
}

impl LogHandle {
    /// Waits until every line queued before this call reached stderr.
    pub async fn flush(&self) {
        let (done, wait) = oneshot::channel();
        if self.queue.send(Item::Flush(done)).await.is_ok() {
            let _ = wait.await;
        }
    }
}

/// Installs the writer as the global subscriber, starts the task that
/// writes the queue to stderr, and installs the panic hook. Must be called
/// inside the runtime. Returns `None` when a subscriber was installed
/// already; the panic hook is then left as it was.
///
/// The hook writes the event `daemon.panicked` through the queue and
/// nothing else: the message and the place of a panic can hold a value, and
/// every line on stderr stays one JSON object (spec 0004, decision 86).
pub fn install() -> Option<LogHandle> {
    let (queue, mut lines) = mpsc::channel(QUEUE_LINES);
    let subscriber = JsonLog {
        queue: queue.clone(),
        dropped: AtomicU64::new(0),
        next_span: AtomicUsize::new(1),
    };
    tracing::subscriber::set_global_default(subscriber).ok()?;
    std::panic::set_hook(Box::new(|_| {
        tracing::error!(event = "daemon.panicked");
    }));
    tokio::spawn(async move {
        let mut stderr = tokio::io::stderr();
        while let Some(item) = lines.recv().await {
            match item {
                Item::Line(line) => {
                    let _ = stderr.write_all(line.as_bytes()).await;
                }
                Item::Flush(done) => {
                    let _ = stderr.flush().await;
                    let _ = done.send(());
                }
            }
        }
    });
    Some(LogHandle { queue })
}

impl JsonLog {
    /// Queues one line, after a `log.dropped` line when lines were lost.
    fn queue(&self, line: String) {
        let dropped = self.dropped.swap(0, Ordering::SeqCst);
        if dropped > 0 {
            let notice = Line::new(Level::WARN, Some("log.dropped"))
                .number("count", dropped)
                .finish();
            if self.queue.try_send(Item::Line(notice)).is_err() {
                self.dropped
                    .fetch_add(dropped.saturating_add(1), Ordering::SeqCst);
                return;
            }
        }
        if self.queue.try_send(Item::Line(line)).is_err() {
            self.dropped.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl Subscriber for JsonLog {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        *metadata.level() <= Level::INFO
    }

    fn new_span(&self, _span: &Attributes<'_>) -> Id {
        // Spans are ignored; each gets an ID so that `tracing` is content.
        let id = self.next_span.fetch_add(1, Ordering::Relaxed).max(1);
        Id::from_u64(u64::try_from(id).unwrap_or(1))
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut fields = Fields::default();
        event.record(&mut fields);
        self.queue(fields.into_line(*event.metadata().level()));
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

/// One field value: a string, a whole number or a boolean.
enum Value {
    Text(String),
    Signed(i64),
    Unsigned(u64),
    Bool(bool),
}

/// The fields of one event, in the order the code gave them.
#[derive(Default)]
struct Fields {
    event: Option<String>,
    values: Vec<(&'static str, Value)>,
}

impl Fields {
    fn into_line(self, level: Level) -> String {
        let (level, event) = match self.event {
            Some(event) => (level, event),
            None => (Level::ERROR, "log.unnamed".to_owned()),
        };
        let mut line = Line::new(level, Some(&event));
        for (name, value) in self.values {
            line = match value {
                Value::Text(text) => line.text(name, &text),
                Value::Signed(number) => line.signed(name, number),
                Value::Unsigned(number) => line.number(name, number),
                Value::Bool(flag) => line.flag(name, flag),
            };
        }
        line.finish()
    }

    fn push(&mut self, field: &Field, value: Value) {
        if field.name() == "event" {
            if let Value::Text(text) = value {
                self.event = Some(text);
            }
            return;
        }
        self.values.push((field.name(), value));
    }
}

impl Visit for Fields {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.push(field, Value::Text(value.to_owned()));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.push(field, Value::Signed(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.push(field, Value::Unsigned(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.push(field, Value::Bool(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        // `%value` and `?value` arrive here; both become their text.
        self.push(field, Value::Text(format!("{value:?}")));
    }
}

/// One JSON line under construction.
struct Line(String);

impl Line {
    fn new(level: Level, event: Option<&str>) -> Line {
        let level = match level {
            Level::ERROR => "error",
            Level::WARN => "warn",
            Level::INFO => "info",
            Level::DEBUG => "debug",
            Level::TRACE => "trace",
        };
        let mut line = String::with_capacity(160);
        line.push_str("{\"ts\":\"");
        line.push_str(&timestamp(SystemTime::now()));
        line.push_str("\",\"level\":\"");
        line.push_str(level);
        line.push_str("\",\"event\":");
        push_json_string(&mut line, event.unwrap_or("log.unnamed"));
        Line(line)
    }

    fn key(mut self, name: &str) -> Line {
        self.0.push(',');
        push_json_string(&mut self.0, name);
        self.0.push(':');
        self
    }

    fn text(self, name: &str, value: &str) -> Line {
        let mut line = self.key(name);
        push_json_string(&mut line.0, value);
        line
    }

    fn number(self, name: &str, value: u64) -> Line {
        let mut line = self.key(name);
        let _ = write!(line.0, "{value}");
        line
    }

    fn signed(self, name: &str, value: i64) -> Line {
        let mut line = self.key(name);
        let _ = write!(line.0, "{value}");
        line
    }

    fn flag(self, name: &str, value: bool) -> Line {
        let mut line = self.key(name);
        line.0.push_str(if value { "true" } else { "false" });
        line
    }

    fn finish(mut self) -> String {
        self.0.push_str("}\n");
        self.0
    }
}

/// A JSON string with `"`, `\` and control characters escaped.
fn push_json_string(out: &mut String, text: &str) {
    out.push('"');
    for scalar in text.chars() {
        match scalar {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if u32::from(c) < 0x20 || c == '\u{7f}' => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// UTC in the form `2026-11-15T08:56:01.123Z`, from the wall clock.
fn timestamp(now: SystemTime) -> String {
    let since = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = since.as_secs();
    let millis = since.subsec_millis();
    let days = i64::try_from(seconds / 86_400).unwrap_or(0);
    let rest = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{millis:03}Z",
        rest / 3600,
        (rest % 3600) / 60,
        rest % 60
    )
}

/// The proleptic Gregorian date of a day count since 1970-01-01.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (
        year,
        u32::try_from(month).unwrap_or(1),
        u32::try_from(day).unwrap_or(1),
    )
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn timestamps_are_utc_with_milliseconds() {
        let at = |seconds: u64, millis: u64| {
            timestamp(UNIX_EPOCH + Duration::from_secs(seconds) + Duration::from_millis(millis))
        };
        assert_eq!(at(0, 0), "1970-01-01T00:00:00.000Z");
        assert_eq!(at(1_763_196_961, 123), "2025-11-15T08:56:01.123Z");
        assert_eq!(at(951_782_400, 999), "2000-02-29T00:00:00.999Z");
        assert_eq!(at(4_102_444_799, 7), "2099-12-31T23:59:59.007Z");
    }

    #[test]
    fn a_line_has_ts_level_and_event_first_then_the_fields_in_order() {
        let mut fields = Fields::default();
        fields
            .values
            .push(("cluster_id", Value::Text("main".to_owned())));
        fields.event = Some("admission.changed".to_owned());
        fields
            .values
            .push(("snapshot_generation", Value::Unsigned(3)));
        fields.values.push(("recorded", Value::Bool(false)));
        fields.values.push(("delta", Value::Signed(-1)));
        let line = fields.into_line(Level::INFO);
        let (ts, rest) = line.split_at(32);
        assert!(
            ts.starts_with("{\"ts\":\"") && ts.ends_with("Z\""),
            "{line}"
        );
        assert_eq!(
            rest,
            ",\"level\":\"info\",\"event\":\"admission.changed\",\"cluster_id\":\"main\",\"snapshot_generation\":3,\"recorded\":false,\"delta\":-1}\n"
        );
    }

    #[test]
    fn an_event_without_a_name_is_log_unnamed_at_error() {
        let line = Fields::default().into_line(Level::INFO);
        assert!(
            line.ends_with(",\"level\":\"error\",\"event\":\"log.unnamed\"}\n"),
            "{line}"
        );
    }

    #[test]
    fn strings_are_escaped() {
        let mut out = String::new();
        push_json_string(&mut out, "a\"b\\c\nd\u{1}é");
        assert_eq!(out, "\"a\\\"b\\\\c\\nd\\u0001é\"");
    }

    #[test]
    fn a_full_queue_drops_and_counts_and_says_so_with_the_next_line() {
        let (queue, mut lines) = mpsc::channel(2);
        let log = JsonLog {
            queue,
            dropped: AtomicU64::new(0),
            next_span: AtomicUsize::new(1),
        };
        for name in ["a", "b", "c", "d"] {
            log.queue(Line::new(Level::INFO, Some(name)).finish());
        }
        assert_eq!(log.dropped.load(Ordering::SeqCst), 2);
        let mut seen = Vec::new();
        while let Ok(Item::Line(line)) = lines.try_recv() {
            seen.push(line);
        }
        assert_eq!(seen.len(), 2);
        log.queue(Line::new(Level::INFO, Some("e")).finish());
        let notice = match lines.try_recv() {
            Ok(Item::Line(line)) => line,
            _ => panic!("a log.dropped line"),
        };
        assert!(
            notice.ends_with(",\"level\":\"warn\",\"event\":\"log.dropped\",\"count\":2}\n"),
            "{notice}"
        );
        assert!(matches!(lines.try_recv(), Ok(Item::Line(line)) if line.contains("\"e\"")));
    }
}
