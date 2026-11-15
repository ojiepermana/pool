#![no_main]
//! The header reader and the whole message reader for client and backend
//! messages, and the classification after login (spec 0004, "Fuzzing").
//!
//! The first two bytes say where to cut the rest into three pieces. The
//! pieces, the whole input and single bytes must give the same messages and
//! the same classification, and nothing may panic or reserve memory from a
//! length field.

use libfuzzer_sys::fuzz_target;
use pool_protocol::message::{
    BackendAction, FrameError, FrontendAction, HEADER_LEN, HeaderReader, Message, MessageReader,
    classify_backend, classify_frontend,
};

const LIMIT: u32 = 1024;

fn messages(pieces: &[&[u8]]) -> Vec<Result<Message, FrameError>> {
    let mut reader = MessageReader::new(LIMIT);
    let mut out = Vec::new();
    for piece in pieces {
        let mut rest = *piece;
        while !rest.is_empty() {
            let (used, result) = reader.feed(rest);
            assert!(used <= rest.len());
            rest = &rest[used..];
            if let Some(result) = result {
                let stop = result.is_err();
                out.push(result);
                if stop {
                    return out;
                }
            }
        }
    }
    out
}

fn headers(pieces: &[&[u8]]) -> Vec<(FrontendAction, BackendAction)> {
    let mut reader = HeaderReader::new();
    let mut out = Vec::new();
    for piece in pieces {
        let mut rest = *piece;
        while !rest.is_empty() {
            let (used, result) = reader.feed(rest);
            rest = &rest[used..];
            if let Some(Ok(header)) = result {
                let len = header.len.to_be_bytes();
                let raw = [header.tag, len[0], len[1], len[2], len[3]];
                out.push((classify_frontend(raw, LIMIT), classify_backend(raw)));
            } else if let Some(Err(_)) = result {
                return out;
            }
        }
    }
    out
}

fuzz_target!(|data: &[u8]| {
    let (cuts, input) = data.split_at(data.len().min(2));
    let a = usize::from(cuts.first().copied().unwrap_or(0)) % (input.len() + 1);
    let b = a + usize::from(cuts.get(1).copied().unwrap_or(0)) % (input.len() - a + 1);
    let pieces: [&[u8]; 3] = [&input[..a], &input[a..b], &input[b..]];
    let singles: Vec<&[u8]> = input.chunks(1).collect();
    let whole = messages(&[input]);
    assert_eq!(messages(&pieces), whole);
    assert_eq!(messages(&singles), whole);
    let classified = headers(&[input]);
    assert_eq!(headers(&pieces), classified);
    assert_eq!(headers(&singles), classified);
    if let Ok(first) = <[u8; HEADER_LEN]>::try_from(input.get(..HEADER_LEN).unwrap_or(&[])) {
        let _ = (classify_frontend(first, u32::MAX), classify_backend(first));
    }
});
