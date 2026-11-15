//! The SCRAM-SHA-256 message text (RFC 5802, RFC 7677), without the crypto.
//!
//! POOL is a SCRAM server toward clients and a SCRAM client toward nodes. It
//! never does channel binding in this slice: as a server it accepts only the
//! headers `n,,` and `y,,`, and as a client it sends `n,,` (spec 0004,
//! "Frontend authentication" and "Backend connection"). The keys, the
//! proof and the signature are computed by the caller; this module only
//! reads and writes the text around them.

use crate::{Malformed, base64};

/// The length of a SCRAM-SHA-256 proof and of a server signature.
pub const PROOF_LEN: usize = 32;
/// The highest iteration count that POOL accepts from a node.
pub const MAX_ITERATIONS: u32 = 600_000;

/// The GS2 header that a client sent in its first message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gs2Header {
    /// `n,,`: the client does not support channel binding.
    NoBinding,
    /// `y,,`: the client supports it but thinks the server does not.
    BindingNotOffered,
}

impl Gs2Header {
    /// The header text, which the client-final message must repeat in base64.
    pub fn text(self) -> &'static str {
        match self {
            Self::NoBinding => "n,,",
            Self::BindingNotOffered => "y,,",
        }
    }
}

/// A client-first message that POOL accepts as a server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientFirst {
    /// The GS2 header.
    pub header: Gs2Header,
    /// The client-first-message-bare, which goes into the auth message.
    pub bare: String,
    /// The client nonce.
    pub nonce: String,
}

fn is_printable(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| matches!(b, 0x21..=0x2b | 0x2d..=0x7e))
}

fn attribute(part: &str, name: char) -> Option<&str> {
    let rest = part.strip_prefix(name)?;
    rest.strip_prefix('=')
}

/// Parses the client-first message of a `SASLInitialResponse`.
///
/// It must start with `n,,` or `y,,`. A request for channel binding
/// (`p=`), an authorization identity, a reserved `m=` attribute or an
/// extension is [`Malformed`], and so is a message that is not UTF-8 text.
/// The `n=` attribute is required by the grammar; the user name in it is
/// read up to the next comma and ignored, as PostgreSQL ignores it.
pub fn parse_client_first(data: &[u8]) -> Result<ClientFirst, Malformed> {
    let text = std::str::from_utf8(data).map_err(|_| Malformed)?;
    let (header, bare) = if let Some(bare) = text.strip_prefix("n,,") {
        (Gs2Header::NoBinding, bare)
    } else if let Some(bare) = text.strip_prefix("y,,") {
        (Gs2Header::BindingNotOffered, bare)
    } else {
        return Err(Malformed);
    };
    let mut parts = bare.split(',');
    // The user name is read up to the next comma and ignored, whatever it holds, as PostgreSQL does.
    parts
        .next()
        .and_then(|p| attribute(p, 'n'))
        .ok_or(Malformed)?;
    let nonce = parts
        .next()
        .and_then(|p| attribute(p, 'r'))
        .ok_or(Malformed)?;
    if !is_printable(nonce) || parts.next().is_some() {
        return Err(Malformed);
    }
    Ok(ClientFirst {
        header,
        bare: bare.to_string(),
        nonce: nonce.to_string(),
    })
}

/// The client-first message that POOL sends to a node: `n,,n=,r=<nonce>`.
pub fn encode_client_first(nonce: &str) -> String {
    format!("n,,{}", client_first_bare(nonce))
}

/// The bare part of [`encode_client_first`], which goes into the auth message.
pub fn client_first_bare(nonce: &str) -> String {
    format!("n=,r={nonce}")
}

/// A server-first message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerFirst {
    /// The whole nonce: the client nonce followed by the server nonce.
    pub nonce: String,
    /// The salt, decoded.
    pub salt: Vec<u8>,
    /// The iteration count.
    pub iterations: u32,
    /// The message text, which goes into the auth message.
    pub text: String,
}

/// The server-first message: `r=<nonce>,s=<salt>,i=<count>`.
pub fn encode_server_first(nonce: &str, salt: &[u8], iterations: u32) -> String {
    format!("r={nonce},s={},i={iterations}", base64::encode(salt))
}

/// Parses the server-first message of a node, as POOL the client.
///
/// The nonce must start with `client_nonce` and be longer, the salt must
/// be non empty base64, and the count must be 1 to [`MAX_ITERATIONS`].
/// A reserved `m=` attribute or an extension is [`Malformed`].
pub fn parse_server_first(data: &[u8], client_nonce: &str) -> Result<ServerFirst, Malformed> {
    let text = std::str::from_utf8(data).map_err(|_| Malformed)?;
    let mut parts = text.split(',');
    let nonce = parts
        .next()
        .and_then(|p| attribute(p, 'r'))
        .ok_or(Malformed)?;
    let salt = parts
        .next()
        .and_then(|p| attribute(p, 's'))
        .ok_or(Malformed)?;
    let count = parts
        .next()
        .and_then(|p| attribute(p, 'i'))
        .ok_or(Malformed)?;
    if parts.next().is_some()
        || !is_printable(nonce)
        || !nonce.starts_with(client_nonce)
        || nonce.len() <= client_nonce.len()
    {
        return Err(Malformed);
    }
    let salt = base64::decode(salt.as_bytes())?;
    if salt.is_empty()
        || count.is_empty()
        || count.len() > 7
        || !count.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Malformed);
    }
    let iterations: u32 = count.parse().map_err(|_| Malformed)?;
    if !(1..=MAX_ITERATIONS).contains(&iterations) {
        return Err(Malformed);
    }
    Ok(ServerFirst {
        nonce: nonce.to_string(),
        salt,
        iterations,
        text: text.to_string(),
    })
}

/// A client-final message that POOL accepts as a server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientFinal {
    /// The client-final-message-without-proof, which goes into the auth message.
    pub without_proof: String,
    /// The client proof.
    pub proof: [u8; PROOF_LEN],
}

/// Parses the client-final message of a `SASLResponse`, as POOL the server.
///
/// The channel binding must be the base64 of `header`, the nonce must be
/// exactly `nonce` (the whole nonce of the server-first message), and the
/// proof must be 32 bytes. An extension is [`Malformed`].
pub fn parse_client_final(
    data: &[u8],
    header: Gs2Header,
    nonce: &str,
) -> Result<ClientFinal, Malformed> {
    let text = std::str::from_utf8(data).map_err(|_| Malformed)?;
    let (without_proof, proof) = text.rsplit_once(",p=").ok_or(Malformed)?;
    let mut parts = without_proof.split(',');
    let binding = parts
        .next()
        .and_then(|p| attribute(p, 'c'))
        .ok_or(Malformed)?;
    let sent_nonce = parts
        .next()
        .and_then(|p| attribute(p, 'r'))
        .ok_or(Malformed)?;
    if parts.next().is_some() || sent_nonce != nonce {
        return Err(Malformed);
    }
    if base64::decode(binding.as_bytes())? != header.text().as_bytes() {
        return Err(Malformed);
    }
    let proof: [u8; PROOF_LEN] = base64::decode(proof.as_bytes())?
        .try_into()
        .map_err(|_| Malformed)?;
    Ok(ClientFinal {
        without_proof: without_proof.to_string(),
        proof,
    })
}

/// The client-final-message-without-proof that POOL sends: `c=biws,r=<nonce>`.
pub fn client_final_without_proof(nonce: &str) -> String {
    format!(
        "c={},r={nonce}",
        base64::encode(Gs2Header::NoBinding.text().as_bytes())
    )
}

/// The client-final message: the part without proof, then `,p=<proof>`.
pub fn encode_client_final(without_proof: &str, proof: &[u8; PROOF_LEN]) -> String {
    format!("{without_proof},p={}", base64::encode(proof))
}

/// The auth message that both signatures are computed over.
pub fn auth_message(
    client_first_bare: &str,
    server_first: &str,
    client_final_without_proof: &str,
) -> String {
    format!("{client_first_bare},{server_first},{client_final_without_proof}")
}

/// A server-final message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerFinal {
    /// `v=`: the server signature.
    Verifier([u8; PROOF_LEN]),
    /// `e=`: the server refused. Its text is not kept.
    Error,
}

/// The server-final message with the server signature: `v=<signature>`.
pub fn encode_server_final(signature: &[u8; PROOF_LEN]) -> String {
    format!("v={}", base64::encode(signature))
}

/// Parses the server-final message of a node, as POOL the client.
pub fn parse_server_final(data: &[u8]) -> Result<ServerFinal, Malformed> {
    if data.starts_with(b"e=") {
        return Ok(ServerFinal::Error);
    }
    let rest = data.strip_prefix(b"v=").ok_or(Malformed)?;
    let signature: [u8; PROOF_LEN] = base64::decode(rest)?.try_into().map_err(|_| Malformed)?;
    Ok(ServerFinal::Verifier(signature))
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 7677 section 3, with the user name the RFC uses.
    const CLIENT_FIRST: &str = "n,,n=user,r=rOprNGfwEbeRWgbNEkqO";
    const SERVER_FIRST: &str =
        "r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096";
    const CLIENT_FINAL: &str = "c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,p=dHzbZapWIk4jUhN+Ute9ytag9zjfMHgsqmmiz7AndVQ=";
    const SERVER_FINAL: &str = "v=6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4=";

    #[test]
    fn rfc_7677_exchange_parses() {
        let first = parse_client_first(CLIENT_FIRST.as_bytes()).expect("client first");
        assert_eq!(first.header, Gs2Header::NoBinding);
        assert_eq!(first.bare, "n=user,r=rOprNGfwEbeRWgbNEkqO");
        assert_eq!(first.nonce, "rOprNGfwEbeRWgbNEkqO");
        let server =
            parse_server_first(SERVER_FIRST.as_bytes(), &first.nonce).expect("server first");
        assert_eq!(server.iterations, 4096);
        assert_eq!(server.salt.len(), 16);
        assert_eq!(
            encode_server_first(&server.nonce, &server.salt, 4096),
            SERVER_FIRST
        );
        let fin = parse_client_final(CLIENT_FINAL.as_bytes(), first.header, &server.nonce)
            .expect("client final");
        assert_eq!(fin.without_proof, client_final_without_proof(&server.nonce));
        assert_eq!(
            encode_client_final(&fin.without_proof, &fin.proof),
            CLIENT_FINAL
        );
        let ServerFinal::Verifier(signature) =
            parse_server_final(SERVER_FINAL.as_bytes()).expect("server final")
        else {
            panic!("a verifier");
        };
        assert_eq!(encode_server_final(&signature), SERVER_FINAL);
        assert_eq!(
            auth_message(&first.bare, &server.text, &fin.without_proof),
            "n=user,r=rOprNGfwEbeRWgbNEkqO,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096,c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0"
        );
    }

    #[test]
    fn client_first_refusals() {
        assert_eq!(
            parse_client_first(b"y,,n=,r=abc").map(|f| f.header),
            Ok(Gs2Header::BindingNotOffered)
        );
        // Any user name up to the next comma is accepted and ignored.
        for name in [
            "a=2Cb=3D",
            "a=zz",
            "=",
            "==",
            "a=b=c",
            "\u{e9}l\u{e8}ve",
            "",
        ] {
            assert_eq!(
                parse_client_first(format!("n,,n={name},r=abc").as_bytes()).map(|f| f.nonce),
                Ok("abc".to_string()),
                "{name}"
            );
        }
        assert_eq!(
            parse_client_first(b"n,,n=\xff\xfe,r=abc"),
            Err(Malformed),
            "not UTF-8"
        );
        for bad in [
            "p=tls-server-end-point,,n=,r=abc",
            "n,a=admin,n=,r=abc",
            "y,a=admin,n=,r=abc",
            "n,,m=ext,n=,r=abc",
            "n,,n=,r=abc,x=ext",
            "n,,n=,r=",
            "n,,n=,r=a,b",
            "n,,n=,r=a b",
            "n,,n=,r=a\u{e9}",
            "n,,r=abc",
            "n,,n=",
            "n,,",
            "n,n=,r=abc",
            "x,,n=,r=abc",
            "",
        ] {
            assert_eq!(parse_client_first(bad.as_bytes()), Err(Malformed), "{bad}");
        }
        assert_eq!(parse_client_first(b"n,,n=,r=\xff"), Err(Malformed));
    }

    #[test]
    fn server_first_refusals() {
        let ok = |t: &str| parse_server_first(t.as_bytes(), "abc");
        assert_eq!(ok("r=abcdef,s=c2FsdA==,i=1").map(|s| s.iterations), Ok(1));
        assert_eq!(
            ok("r=abcdef,s=c2FsdA==,i=600000").map(|s| s.iterations),
            Ok(600_000)
        );
        for bad in [
            "r=abc,s=c2FsdA==,i=4096",
            "r=xbcdef,s=c2FsdA==,i=4096",
            "r=abcdef,s=,i=4096",
            "r=abcdef,s=c2FsdA=,i=4096",
            "r=abcdef,s=c2FsdA==,i=0",
            "r=abcdef,s=c2FsdA==,i=600001",
            "r=abcdef,s=c2FsdA==,i=99999999999",
            "r=abcdef,s=c2FsdA==,i=+4096",
            "r=abcdef,s=c2FsdA==,i=",
            "r=abcdef,s=c2FsdA==,i=4096,x=1",
            "m=1,r=abcdef,s=c2FsdA==,i=4096",
            "r=abcdef,i=4096,s=c2FsdA==",
            "e=other-error",
        ] {
            assert_eq!(ok(bad), Err(Malformed), "{bad}");
        }
    }

    #[test]
    fn client_final_refusals() {
        let nonce = "abcdef";
        let proof = base64::encode(&[7; 32]);
        let ok = |t: String, h| parse_client_final(t.as_bytes(), h, nonce);
        assert!(ok(format!("c=biws,r=abcdef,p={proof}"), Gs2Header::NoBinding).is_ok());
        assert!(
            ok(
                format!("c=eSws,r=abcdef,p={proof}"),
                Gs2Header::BindingNotOffered
            )
            .is_ok()
        );
        for bad in [
            format!("c=eSws,r=abcdef,p={proof}"),
            format!("c=biws,r=abcde,p={proof}"),
            format!("c=biws,r=abcdefg,p={proof}"),
            format!("c=biws,r=abcdef,x=1,p={proof}"),
            format!("r=abcdef,c=biws,p={proof}"),
            format!("c=biws,r=abcdef,p={}", base64::encode(&[7; 31])),
            format!("c=biws,r=abcdef,p={}", base64::encode(&[7; 33])),
            "c=biws,r=abcdef".to_string(),
            format!("c=biw,r=abcdef,p={proof}"),
            format!("c=biws,r=abcdef,p={proof},x=1"),
        ] {
            assert_eq!(
                ok(bad.clone(), Gs2Header::NoBinding),
                Err(Malformed),
                "{bad}"
            );
        }
    }

    #[test]
    fn client_side_messages() {
        assert_eq!(encode_client_first("abc"), "n,,n=,r=abc");
        assert_eq!(client_final_without_proof("abcdef"), "c=biws,r=abcdef");
        assert_eq!(
            parse_server_final(b"e=invalid-proof"),
            Ok(ServerFinal::Error)
        );
        for bad in [&b"v="[..], b"v=AAAA", b"x=1", b""] {
            assert_eq!(parse_server_final(bad), Err(Malformed));
        }
    }
}
