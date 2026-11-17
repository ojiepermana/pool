//! Standard base64 with padding (RFC 4648 section 4), as SCRAM uses it.
//!
//! The decoder is strict: only the standard alphabet, a length that is a
//! multiple of four, padding only at the end, and no bits set in the
//! padding. It allocates at most three quarters of its input length.

use crate::Malformed;

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encodes bytes as base64 text.
pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = (u32::from(b0) << 16) | (u32::from(b1) << 8) | u32::from(b2);
        out.push(char::from(ALPHABET[(n >> 18) as usize & 63]));
        out.push(char::from(ALPHABET[(n >> 12) as usize & 63]));
        out.push(if chunk.len() > 1 {
            char::from(ALPHABET[(n >> 6) as usize & 63])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(ALPHABET[n as usize & 63])
        } else {
            '='
        });
    }
    out
}

fn value(byte: u8) -> Option<u32> {
    let v = match byte {
        b'A'..=b'Z' => byte - b'A',
        b'a'..=b'z' => byte - b'a' + 26,
        b'0'..=b'9' => byte - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => return None,
    };
    Some(u32::from(v))
}

/// Decodes base64 text. Any deviation from the strict form is [`Malformed`].
pub fn decode(text: &[u8]) -> Result<Vec<u8>, Malformed> {
    if !text.len().is_multiple_of(4) {
        return Err(Malformed);
    }
    let mut out = Vec::with_capacity(text.len() / 4 * 3);
    let quads = text.len() / 4;
    for (index, quad) in text.chunks(4).enumerate() {
        let last = index + 1 == quads;
        let pad = quad.iter().rev().take_while(|&&b| b == b'=').count();
        if pad > 2 || (pad > 0 && !last) {
            return Err(Malformed);
        }
        let mut n = 0u32;
        for &byte in &quad[..4 - pad] {
            n = (n << 6) | value(byte).ok_or(Malformed)?;
        }
        n <<= 6 * pad as u32;
        let bytes = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        match pad {
            0 => out.extend_from_slice(&bytes),
            1 if bytes[2] == 0 => out.extend_from_slice(&bytes[..2]),
            2 if bytes[1] == 0 && bytes[2] == 0 => out.push(bytes[0]),
            _ => return Err(Malformed),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_4648_vectors_round_trip() {
        for (plain, text) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(encode(plain.as_bytes()), text);
            assert_eq!(decode(text.as_bytes()), Ok(plain.as_bytes().to_vec()));
        }
        // The SCRAM channel binding of a client without binding.
        assert_eq!(encode(b"n,,"), "biws");
    }

    #[test]
    fn every_byte_round_trips() {
        let all: Vec<u8> = (0..=255).collect();
        for len in 0..all.len() {
            assert_eq!(
                decode(encode(&all[..len]).as_bytes()),
                Ok(all[..len].to_vec())
            );
        }
    }

    #[test]
    fn strict_form_only() {
        for bad in [
            "Zg", "Zg=", "Zg===", "Z===", "====", "Zm9v=Zg=", "Zg==Zg==", "Zh==", "Zm9=", "Zm-v",
            "Zm_v", "Zm9v\n", " Zm9v", "Zm 9",
        ] {
            assert_eq!(decode(bad.as_bytes()), Err(Malformed), "{bad}");
        }
    }
}
