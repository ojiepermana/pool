//! Messages after the first packets: a tag byte, then a length that counts
//! itself and the body but not the tag (spec 0004, "Messages after login").
//!
//! [`HeaderReader`] reads only the five header bytes, for the relay that
//! copies a body through a fixed buffer and never holds a whole message.
//! [`MessageReader`] reads a whole message under a limit, for the small
//! messages of authentication and backend startup.

use crate::sqlstate::PoolError;
use crate::{Malformed, be_u32};

/// The size of a message header: one tag byte and a four byte length.
pub const HEADER_LEN: usize = 5;

/// A checked message header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// The tag byte.
    pub tag: u8,
    /// The length field: four bytes of length plus the body.
    pub len: u32,
}

impl Header {
    /// Reads a header from its five bytes. A length below 4 is [`Malformed`].
    pub fn parse(bytes: [u8; HEADER_LEN]) -> Result<Self, Malformed> {
        let len = be_u32(&bytes[1..]).ok_or(Malformed)?;
        if len < 4 {
            return Err(Malformed);
        }
        Ok(Self { tag: bytes[0], len })
    }

    /// The number of body bytes that follow the header.
    pub fn body_len(self) -> u32 {
        self.len - 4
    }
}

/// Reads the five header bytes of a message, from pieces of any size.
#[derive(Debug, Default, Clone)]
pub struct HeaderReader {
    bytes: [u8; HEADER_LEN],
    filled: usize,
}

impl HeaderReader {
    /// A reader that has seen nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// How many more bytes the header needs. A caller that reads exactly this many never reads past the header.
    pub fn wanted(&self) -> usize {
        HEADER_LEN - self.filled
    }

    /// Takes bytes from the start of `input`, never more than [`Self::wanted`].
    ///
    /// Returns how many bytes it took and, once the header is complete,
    /// the header or [`Malformed`]. The reader then starts over.
    pub fn feed(&mut self, input: &[u8]) -> (usize, Option<Result<Header, Malformed>>) {
        let take = self.wanted().min(input.len());
        self.bytes[self.filled..self.filled + take].copy_from_slice(&input[..take]);
        self.filled += take;
        if self.filled < HEADER_LEN {
            return (take, None);
        }
        self.filled = 0;
        (take, Some(Header::parse(self.bytes)))
    }
}

/// A whole message: its tag and its body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The tag byte.
    pub tag: u8,
    /// The body, without tag and length.
    pub body: Vec<u8>,
}

/// Why a whole message could not be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    /// The length field is below 4.
    Malformed,
    /// The body is longer than the limit of the reader. Nothing was reserved for it.
    TooLarge,
}

/// Reads whole messages under a limit on the body, from pieces of any size.
///
/// The body is reserved only after its length has been checked against the limit.
#[derive(Debug, Clone)]
pub struct MessageReader {
    limit: u32,
    header: HeaderReader,
    current: Option<(u8, Vec<u8>, usize)>,
}

impl MessageReader {
    /// A reader that accepts bodies of at most `limit` bytes.
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            header: HeaderReader::new(),
            current: None,
        }
    }

    /// How many more bytes the current message needs, as far as known now.
    pub fn wanted(&self) -> usize {
        match &self.current {
            Some((_, body, len)) => len - body.len(),
            None => self.header.wanted(),
        }
    }

    /// Takes bytes from the start of `input`, never more than the current message needs.
    ///
    /// Returns how many bytes it took and, once a message is complete, the
    /// message or the reason it was refused. After a refusal the stream
    /// cannot be read further.
    pub fn feed(&mut self, input: &[u8]) -> (usize, Option<Result<Message, FrameError>>) {
        let mut used = 0;
        if self.current.is_none() {
            let (taken, header) = self.header.feed(input);
            used += taken;
            match header {
                None => return (used, None),
                Some(Err(Malformed)) => return (used, Some(Err(FrameError::Malformed))),
                Some(Ok(header)) => {
                    if header.body_len() > self.limit {
                        return (used, Some(Err(FrameError::TooLarge)));
                    }
                    let len = header.body_len() as usize;
                    self.current = Some((header.tag, Vec::with_capacity(len), len));
                }
            }
        }
        let Some((tag, body, len)) = self.current.as_mut() else {
            return (used, None);
        };
        let rest = &input[used..];
        let take = (*len - body.len()).min(rest.len());
        body.extend_from_slice(&rest[..take]);
        used += take;
        if body.len() < *len {
            return (used, None);
        }
        let tag = *tag;
        let body = std::mem::take(body);
        self.current = None;
        (used, Some(Ok(Message { tag, body })))
    }
}

/// What POOL does with a message header from the client after login.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendAction {
    /// `Query`: send the header and `body_len` bytes on, then a query is in flight.
    Query {
        /// The bytes of the body that follow.
        body_len: u32,
    },
    /// `Terminate`: send it on and close both connections.
    Terminate {
        /// The bytes of the body that follow.
        body_len: u32,
    },
    /// Refuse with this error and close the backend.
    Refuse(PoolError),
}

/// Classifies a client message header after `ReadyForQuery`, spec 0004 "Messages after login".
///
/// `max_len` is `limits.frontend_message_max_bytes`, compared with the length field.
pub fn classify_frontend(bytes: [u8; HEADER_LEN], max_len: u32) -> FrontendAction {
    let Ok(header) = Header::parse(bytes) else {
        return FrontendAction::Refuse(PoolError::ProtocolViolation);
    };
    match header.tag {
        b'Q' if header.len > max_len => FrontendAction::Refuse(PoolError::MessageTooLarge),
        b'Q' => FrontendAction::Query {
            body_len: header.body_len(),
        },
        b'X' => FrontendAction::Terminate {
            body_len: header.body_len(),
        },
        b'P' | b'B' | b'D' | b'E' | b'C' | b'H' | b'S' | b'F' | b'd' | b'c' | b'f' => {
            FrontendAction::Refuse(PoolError::FeatureNotSupported)
        }
        _ => FrontendAction::Refuse(PoolError::ProtocolViolation),
    }
}

/// What POOL does with a message header from the backend during a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendAction {
    /// Send the header and `body_len` bytes to the client unchanged.
    Forward {
        /// The bytes of the body that follow.
        body_len: u32,
    },
    /// `ReadyForQuery` of length 5: send it to the client; its one body byte is the status.
    ReadyForQuery,
    /// A COPY response: send nothing, give the client `0A000` and close the backend.
    CopyResponse,
    /// Close the backend; the session ends as backend loss.
    Lost,
}

/// Classifies a backend message header during a session, spec 0004 "Messages after login".
pub fn classify_backend(bytes: [u8; HEADER_LEN]) -> BackendAction {
    let Ok(header) = Header::parse(bytes) else {
        return BackendAction::Lost;
    };
    match header.tag {
        b'T' | b'D' | b'C' | b'I' | b'E' | b'N' | b'S' | b'A' => BackendAction::Forward {
            body_len: header.body_len(),
        },
        b'Z' if header.len == 5 => BackendAction::ReadyForQuery,
        b'G' | b'H' | b'W' => BackendAction::CopyResponse,
        _ => BackendAction::Lost,
    }
}

/// Builds a message from its tag and body.
///
/// # Panics
///
/// When the body is longer than `u32::MAX - 4` bytes; every caller builds small messages.
pub fn encode_message(tag: u8, body: &[u8]) -> Vec<u8> {
    let len = u32::try_from(body.len() + 4).expect("message body below 4 GiB");
    let mut out = Vec::with_capacity(body.len() + HEADER_LEN);
    out.push(tag);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(body);
    out
}

/// `Query`: one SQL string, ended by a zero byte.
pub fn encode_query(sql: &str) -> Vec<u8> {
    let mut body = Vec::with_capacity(sql.len() + 1);
    body.extend_from_slice(sql.as_bytes());
    body.push(0);
    encode_message(b'Q', &body)
}

/// `Terminate`.
pub fn encode_terminate() -> Vec<u8> {
    encode_message(b'X', &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(tag: u8, len: u32) -> [u8; 5] {
        let l = len.to_be_bytes();
        [tag, l[0], l[1], l[2], l[3]]
    }

    #[test]
    fn frontend_rows() {
        assert_eq!(
            classify_frontend(header(b'Q', 9), 1024),
            FrontendAction::Query { body_len: 5 }
        );
        assert_eq!(
            classify_frontend(header(b'Q', 1024), 1024),
            FrontendAction::Query { body_len: 1020 }
        );
        assert_eq!(
            classify_frontend(header(b'Q', 1025), 1024),
            FrontendAction::Refuse(PoolError::MessageTooLarge)
        );
        assert_eq!(
            classify_frontend(header(b'X', 4), 1024),
            FrontendAction::Terminate { body_len: 0 }
        );
        for tag in *b"PBDECHSFdcf" {
            assert_eq!(
                classify_frontend(header(tag, 4), 1024),
                FrontendAction::Refuse(PoolError::FeatureNotSupported),
                "{}",
                char::from(tag)
            );
        }
        for tag in [b'p', b'Z', b'x', 0, 255] {
            assert_eq!(
                classify_frontend(header(tag, 4), 1024),
                FrontendAction::Refuse(PoolError::ProtocolViolation)
            );
        }
        for len in [0, 1, 3] {
            assert_eq!(
                classify_frontend(header(b'Q', len), 1024),
                FrontendAction::Refuse(PoolError::ProtocolViolation)
            );
        }
    }

    #[test]
    fn backend_rows() {
        for tag in *b"TDCIENSA" {
            assert_eq!(
                classify_backend(header(tag, 10)),
                BackendAction::Forward { body_len: 6 }
            );
        }
        assert_eq!(
            classify_backend(header(b'Z', 5)),
            BackendAction::ReadyForQuery
        );
        assert_eq!(classify_backend(header(b'Z', 6)), BackendAction::Lost);
        assert_eq!(classify_backend(header(b'Z', 4)), BackendAction::Lost);
        for tag in *b"GHW" {
            assert_eq!(
                classify_backend(header(tag, 7)),
                BackendAction::CopyResponse
            );
        }
        for tag in *b"KRv12stnc" {
            assert_eq!(classify_backend(header(tag, 8)), BackendAction::Lost);
        }
        assert_eq!(classify_backend(header(b'D', 3)), BackendAction::Lost);
    }

    #[test]
    fn a_reader_reserves_nothing_for_a_body_over_its_limit() {
        let mut reader = MessageReader::new(16);
        let (used, out) = reader.feed(&header(b'p', u32::MAX));
        assert_eq!(used, 5);
        assert_eq!(out, Some(Err(FrameError::TooLarge)));
        let mut reader = MessageReader::new(16);
        assert_eq!(
            reader.feed(&header(b'p', 3)).1,
            Some(Err(FrameError::Malformed))
        );
        let mut reader = MessageReader::new(16);
        let msg = encode_message(b'p', &[7; 16]);
        assert_eq!(
            reader.feed(&msg),
            (
                21,
                Some(Ok(Message {
                    tag: b'p',
                    body: vec![7; 16]
                }))
            )
        );
    }

    #[test]
    fn a_reader_takes_only_what_the_message_needs() {
        let mut two = encode_terminate();
        two.extend_from_slice(&encode_query("select 1"));
        let mut reader = MessageReader::new(64);
        let (used, out) = reader.feed(&two);
        assert_eq!(used, 5);
        assert_eq!(
            out,
            Some(Ok(Message {
                tag: b'X',
                body: vec![]
            }))
        );
        let (used, out) = reader.feed(&two[5..]);
        assert_eq!(used, two.len() - 5);
        assert_eq!(out.and_then(Result::ok).map(|m| m.tag), Some(b'Q'));
    }
}
