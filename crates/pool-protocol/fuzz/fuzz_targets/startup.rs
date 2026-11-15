#![no_main]
//! The first packet reader and the parameter parser (spec 0004, "Fuzzing").
//!
//! The first byte says whether the connection is already over TLS, the next
//! two say where to cut the rest into three pieces. Feeding the pieces, the
//! whole input and single bytes must give the same results, and nothing may
//! panic or reserve memory from a length field.

use libfuzzer_sys::fuzz_target;
use pool_protocol::sqlstate::PoolError;
use pool_protocol::startup::{FirstPacket, FirstPacketReader, encode_ssl_request, parse_startup};

const MAX: u32 = 8192;

fn run(tls: bool, pieces: &[&[u8]]) -> Vec<Result<FirstPacket, PoolError>> {
    let mut reader = FirstPacketReader::new(MAX);
    if tls {
        assert_eq!(
            reader.feed(&encode_ssl_request()).1,
            Some(Ok(FirstPacket::SslRequest))
        );
    }
    let mut out = Vec::new();
    for piece in pieces {
        let mut rest = *piece;
        while !rest.is_empty() {
            let (used, result) = reader.feed(rest);
            assert!(used <= rest.len());
            rest = &rest[used..];
            match result {
                Some(result) => {
                    let stop = result.is_err();
                    out.push(result);
                    if stop {
                        return out;
                    }
                }
                None => assert!(
                    rest.is_empty(),
                    "a reader without a result takes all it is given"
                ),
            }
        }
    }
    out
}

fuzz_target!(|data: &[u8]| {
    let Some((&flags, rest)) = data.split_first() else {
        return;
    };
    let tls = flags & 1 == 1;
    let (cuts, input) = rest.split_at(rest.len().min(2));
    let a = usize::from(cuts.first().copied().unwrap_or(0)) % (input.len() + 1);
    let b = a + usize::from(cuts.get(1).copied().unwrap_or(0)) % (input.len() - a + 1);
    let whole = run(tls, &[input]);
    assert_eq!(run(tls, &[&input[..a], &input[a..b], &input[b..]]), whole);
    let singles: Vec<&[u8]> = input.chunks(1).collect();
    assert_eq!(run(tls, &singles), whole);
    // The parameter parser on its own, on any body.
    let _ = parse_startup(u16::from(flags), input);
});
