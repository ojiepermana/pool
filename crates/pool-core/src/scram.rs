//! SCRAM-SHA-256 as a server toward clients and as a client toward nodes
//! (spec 0004, "Frontend authentication" and "Backend connection").
//!
//! The message text is read and written by `pool_protocol::scram`; this
//! module adds the keys, the proof and the signatures, all through the safe
//! interface of `aws-lc-rs`. Channel binding is neither offered nor used.
//! As a server POOL does no PBKDF2 work: the verifier holds the keys. As a
//! client the PBKDF2 derivation is [`derive_keys`], which the caller runs on
//! a blocking thread and keeps per credential, salt and count.

use std::fmt;

use aws_lc_rs::{constant_time, digest, hmac, pbkdf2};
use pool_protocol::base64;
use pool_protocol::scram::{
    self as text, ClientFirst, Gs2Header, PROOF_LEN, ServerFinal, ServerFirst,
};

use crate::REDACTED;
use crate::random::{RandomError, fill_random};
use crate::secret::{Password, Verifier};

/// The number of random bytes in a nonce, before base64.
pub const NONCE_BYTES: usize = 18;
/// The number of salt bytes that a made up verifier gets from its HMAC.
pub const MOCK_SALT_BYTES: usize = 16;

/// Why a SCRAM exchange did not succeed. It carries no text from the peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScramFault {
    /// A message broke the SCRAM grammar or the rules of the spec (`08P01` toward a client).
    Protocol,
    /// The proof did not match, or the name is no principal (`28P01` toward a client),
    /// or the node's signature was wrong or it answered `e=`.
    Failed,
    /// The operating system gave no random bytes.
    Random,
}

impl fmt::Display for ScramFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Protocol => "scram message refused",
            Self::Failed => "scram authentication failed",
            Self::Random => "no random bytes",
        })
    }
}

impl std::error::Error for ScramFault {}

fn hmac256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let key = hmac::Key::new(hmac::HMAC_SHA256, key);
    let mut out = [0; 32];
    out.copy_from_slice(hmac::sign(&key, data).as_ref());
    out
}

fn sha256(data: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    out.copy_from_slice(digest::digest(&digest::SHA256, data).as_ref());
    out
}

/// Fills `out` with random bytes, through [`crate::random::fill_random`].
pub(crate) fn random(out: &mut [u8]) -> Result<(), ScramFault> {
    fill_random(out).map_err(|_| ScramFault::Random)
}

/// A new nonce: 18 random bytes in base64.
pub fn new_nonce() -> Result<String, ScramFault> {
    let mut bytes = [0; NONCE_BYTES];
    random(&mut bytes)?;
    Ok(base64::encode(&bytes))
}

/// The two keys derived from a password for one salt and count. `Debug` prints `[REDACTED]`.
#[derive(Clone, PartialEq, Eq)]
pub struct ScramKeys {
    client_key: [u8; 32],
    server_key: [u8; 32],
}

impl fmt::Debug for ScramKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

/// Derives the client key and the server key with PBKDF2-HMAC-SHA-256.
///
/// This is blocking work proportional to `iterations` (at most 600000):
/// run it inside `tokio::task::spawn_blocking`.
///
/// # Panics
///
/// When `iterations` is 0; the parser of the server-first message never returns 0.
pub fn derive_keys(password: &Password, salt: &[u8], iterations: u32) -> ScramKeys {
    let rounds = std::num::NonZeroU32::new(iterations).expect("an iteration count of at least 1");
    let mut salted = [0; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        rounds,
        salt,
        password.as_bytes(),
        &mut salted,
    );
    ScramKeys {
        client_key: hmac256(&salted, b"Client Key"),
        server_key: hmac256(&salted, b"Server Key"),
    }
}

/// The verifier text that PostgreSQL would store for this password, salt and count.
///
/// For test fixtures and tools: the daemon never makes a verifier.
pub fn verifier_text(password: &Password, salt: &[u8], iterations: u32) -> String {
    let keys = derive_keys(password, salt, iterations);
    format!(
        "SCRAM-SHA-256${iterations}:{}${}:{}",
        base64::encode(salt),
        base64::encode(&sha256(&keys.client_key)),
        base64::encode(&keys.server_key)
    )
}

/// The key that makes the salt of a made up verifier for a name that is no principal.
///
/// The daemon makes it once at start from 32 random bytes. `Debug` prints `[REDACTED]`.
#[derive(Clone)]
pub struct MockKey([u8; 32]);

impl MockKey {
    /// A new key from 32 random bytes. A failure is `random.failed` for the command.
    pub fn generate() -> Result<Self, RandomError> {
        let mut key = [0; 32];
        fill_random(&mut key)?;
        Ok(Self(key))
    }

    fn salt(&self, name: &[u8]) -> Vec<u8> {
        hmac256(&self.0, name)[..MOCK_SALT_BYTES].to_vec()
    }
}

impl fmt::Debug for MockKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

/// What the server checks a client against.
#[derive(Debug, Clone)]
pub enum ServerCredential<'a> {
    /// The verifier of the principal entry with the claimed name.
    Known(&'a Verifier),
    /// A made up verifier for a name that is no principal. The exchange always fails.
    Unknown {
        /// The first 16 bytes of an HMAC of the name under the mock key.
        salt: Vec<u8>,
        /// The count of the verifier of the first principal.
        iterations: u32,
    },
}

impl<'a> ServerCredential<'a> {
    /// The made up verifier for `name`: asking twice gives the same salt.
    pub fn unknown(key: &MockKey, name: &[u8], iterations: u32) -> Self {
        Self::Unknown {
            salt: key.salt(name),
            iterations,
        }
    }

    fn salt_and_count(&self) -> (&[u8], u32) {
        match self {
            Self::Known(v) => (&v.salt, v.iterations),
            Self::Unknown { salt, iterations } => (salt, *iterations),
        }
    }
}

/// The server side after the client-first message: what it must remember for the final message.
#[derive(Debug, Clone)]
pub struct ServerExchange<'a> {
    credential: ServerCredential<'a>,
    header: Gs2Header,
    nonce: String,
    client_first_bare: String,
    server_first: String,
}

impl<'a> ServerExchange<'a> {
    /// Steps 2 and 3: reads the client-first message and returns the server-first message to send.
    pub fn start(
        credential: ServerCredential<'a>,
        client_first: &[u8],
    ) -> Result<(Self, String), ScramFault> {
        let server_nonce = new_nonce()?;
        Self::start_with_nonce(credential, client_first, &server_nonce)
    }

    pub(crate) fn start_with_nonce(
        credential: ServerCredential<'a>,
        client_first: &[u8],
        server_nonce: &str,
    ) -> Result<(Self, String), ScramFault> {
        let ClientFirst {
            header,
            bare,
            nonce,
        } = text::parse_client_first(client_first).map_err(|_| ScramFault::Protocol)?;
        let nonce = format!("{nonce}{server_nonce}");
        let (salt, iterations) = credential.salt_and_count();
        let server_first = text::encode_server_first(&nonce, salt, iterations);
        let exchange = Self {
            credential,
            header,
            nonce,
            client_first_bare: bare,
            server_first: server_first.clone(),
        };
        Ok((exchange, server_first))
    }

    /// Steps 4 to 6: reads the client-final message. On success returns the
    /// server-final message to send, and the principal is verified.
    pub fn finish(self, client_final: &[u8]) -> Result<String, ScramFault> {
        let fin = text::parse_client_final(client_final, self.header, &self.nonce)
            .map_err(|_| ScramFault::Protocol)?;
        let auth = text::auth_message(
            &self.client_first_bare,
            &self.server_first,
            &fin.without_proof,
        );
        // The same work for a known and an unknown name; the unknown one always fails.
        let (stored_key, server_key, known) = match &self.credential {
            ServerCredential::Known(v) => (v.stored_key, v.server_key, true),
            ServerCredential::Unknown { .. } => ([0; 32], [0; 32], false),
        };
        let signature = hmac256(&stored_key, auth.as_bytes());
        let mut client_key = [0; 32];
        for (out, (p, s)) in client_key
            .iter_mut()
            .zip(fin.proof.iter().zip(signature.iter()))
        {
            *out = p ^ s;
        }
        let matches =
            constant_time::verify_slices_are_equal(&sha256(&client_key), &stored_key).is_ok();
        if !(matches && known) {
            return Err(ScramFault::Failed);
        }
        Ok(text::encode_server_final(&hmac256(
            &server_key,
            auth.as_bytes(),
        )))
    }
}

/// The client side toward a node, from the client-first message to the check of the server signature.
#[derive(Debug, Clone)]
pub struct ClientExchange {
    nonce: String,
}

impl ClientExchange {
    /// Step 5: a new nonce, and the client-first message `n,,n=,r=<nonce>` to send.
    pub fn start() -> Result<(Self, String), ScramFault> {
        let nonce = new_nonce()?;
        let first = text::encode_client_first(&nonce);
        Ok((Self { nonce }, first))
    }

    /// Reads the server-first message of the node: its nonce must start with ours and its count be 1 to 600000.
    pub fn server_first(&self, data: &[u8]) -> Result<ServerFirst, ScramFault> {
        text::parse_server_first(data, &self.nonce).map_err(|_| ScramFault::Protocol)
    }

    /// The client-final message with `c=biws` and the proof, and the server signature to expect.
    pub fn client_final(
        &self,
        server_first: &ServerFirst,
        keys: &ScramKeys,
    ) -> (String, ExpectedSignature) {
        let without_proof = text::client_final_without_proof(&server_first.nonce);
        let auth = text::auth_message(
            &text::client_first_bare(&self.nonce),
            &server_first.text,
            &without_proof,
        );
        let (proof, signature) = proof_and_signature(keys, &auth);
        (
            text::encode_client_final(&without_proof, &proof),
            ExpectedSignature(signature),
        )
    }
}

/// The server signature that the node must send. `Debug` prints `[REDACTED]`.
#[derive(Clone, PartialEq, Eq)]
pub struct ExpectedSignature([u8; PROOF_LEN]);

impl fmt::Debug for ExpectedSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

impl ExpectedSignature {
    /// Checks the server-final message of the node. `e=` or a wrong signature is [`ScramFault::Failed`].
    pub fn check(&self, data: &[u8]) -> Result<(), ScramFault> {
        match text::parse_server_final(data).map_err(|_| ScramFault::Protocol)? {
            ServerFinal::Verifier(sig)
                if constant_time::verify_slices_are_equal(&sig, &self.0).is_ok() =>
            {
                Ok(())
            }
            _ => Err(ScramFault::Failed),
        }
    }
}

fn proof_and_signature(keys: &ScramKeys, auth: &str) -> ([u8; PROOF_LEN], [u8; PROOF_LEN]) {
    let stored_key = sha256(&keys.client_key);
    let client_signature = hmac256(&stored_key, auth.as_bytes());
    let mut proof = [0; PROOF_LEN];
    for (out, (k, s)) in proof
        .iter_mut()
        .zip(keys.client_key.iter().zip(client_signature.iter()))
    {
        *out = k ^ s;
    }
    (proof, hmac256(&keys.server_key, auth.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // RFC 7677 section 3.
    const SALT: &str = "W22ZaJ0SNY7soEsUEjb6gQ==";
    const CLIENT_NONCE: &str = "rOprNGfwEbeRWgbNEkqO";
    const SERVER_NONCE: &str = "%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0";
    const CLIENT_FIRST: &str = "n,,n=user,r=rOprNGfwEbeRWgbNEkqO";
    const CLIENT_FINAL: &str = "c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,p=dHzbZapWIk4jUhN+Ute9ytag9zjfMHgsqmmiz7AndVQ=";
    const SERVER_FINAL: &str = "v=6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4=";

    fn pencil() -> Password {
        Password::parse(b"pencil").expect("valid")
    }

    fn salt() -> Vec<u8> {
        base64::decode(SALT.as_bytes()).expect("base64")
    }

    #[test]
    fn rfc_7677_proof_and_signature() {
        let keys = derive_keys(&pencil(), &salt(), 4096);
        let without_proof = format!("c=biws,r={CLIENT_NONCE}{SERVER_NONCE}");
        let auth = text::auth_message(
            &CLIENT_FIRST[3..],
            &text::encode_server_first(&format!("{CLIENT_NONCE}{SERVER_NONCE}"), &salt(), 4096),
            &without_proof,
        );
        let (proof, signature) = proof_and_signature(&keys, &auth);
        assert_eq!(
            text::encode_client_final(&without_proof, &proof),
            CLIENT_FINAL
        );
        assert_eq!(text::encode_server_final(&signature), SERVER_FINAL);
    }

    #[test]
    fn rfc_7677_as_the_server() {
        let verifier =
            Verifier::parse(verifier_text(&pencil(), &salt(), 4096).as_bytes()).expect("valid");
        let (exchange, server_first) = ServerExchange::start_with_nonce(
            ServerCredential::Known(&verifier),
            CLIENT_FIRST.as_bytes(),
            SERVER_NONCE,
        )
        .expect("first");
        assert_eq!(
            server_first,
            "r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096"
        );
        assert_eq!(
            exchange.clone().finish(CLIENT_FINAL.as_bytes()),
            Ok(SERVER_FINAL.to_string())
        );
        // A proof of another password fails; a bit flip in the proof fails.
        let wrong = CLIENT_FINAL.replace("p=dHzb", "p=dHzc");
        assert_eq!(exchange.finish(wrong.as_bytes()), Err(ScramFault::Failed));
    }

    #[test]
    fn client_and_server_agree_and_a_wrong_password_fails() {
        let verifier = Verifier::parse(verifier_text(&pencil(), b"some salt", 4096).as_bytes())
            .expect("valid");
        for (password, ok) in [("pencil", true), ("pencil2", false)] {
            let (client, first) = ClientExchange::start().expect("random");
            let (server, server_first) =
                ServerExchange::start(ServerCredential::Known(&verifier), first.as_bytes())
                    .expect("first");
            let parsed = client
                .server_first(server_first.as_bytes())
                .expect("server first");
            let keys = derive_keys(
                &Password::parse(password.as_bytes()).expect("valid"),
                &parsed.salt,
                parsed.iterations,
            );
            let (fin, expected) = client.client_final(&parsed, &keys);
            match server.finish(fin.as_bytes()) {
                Ok(server_final) => {
                    assert!(ok);
                    assert_eq!(expected.check(server_final.as_bytes()), Ok(()));
                }
                Err(fault) => {
                    assert!(!ok);
                    assert_eq!(fault, ScramFault::Failed);
                }
            }
        }
    }

    #[test]
    fn an_unknown_name_gets_a_stable_salt_and_always_fails() {
        let key = MockKey::generate().expect("random");
        let a = ServerCredential::unknown(&key, b"mallory", 4096);
        let b = ServerCredential::unknown(&key, b"mallory", 4096);
        let c = ServerCredential::unknown(&key, b"eve", 4096);
        assert_eq!(a.salt_and_count(), b.salt_and_count());
        assert_ne!(a.salt_and_count().0, c.salt_and_count().0);
        assert_eq!(a.salt_and_count().0.len(), MOCK_SALT_BYTES);
        let (client, first) = ClientExchange::start().expect("random");
        let (server, server_first) = ServerExchange::start(a, first.as_bytes()).expect("first");
        let parsed = client
            .server_first(server_first.as_bytes())
            .expect("same shape as a real one");
        assert_eq!(parsed.iterations, 4096);
        let keys = derive_keys(&pencil(), &parsed.salt, parsed.iterations);
        let (fin, _) = client.client_final(&parsed, &keys);
        assert_eq!(server.finish(fin.as_bytes()), Err(ScramFault::Failed));
        assert_eq!(format!("{key:?}"), REDACTED);
    }

    #[test]
    fn protocol_faults_and_server_refusals() {
        let verifier =
            Verifier::parse(verifier_text(&pencil(), b"salt", 1).as_bytes()).expect("valid");
        assert_eq!(
            ServerExchange::start(
                ServerCredential::Known(&verifier),
                b"p=tls-server-end-point,,n=,r=a"
            )
            .map(|_| ()),
            Err(ScramFault::Protocol)
        );
        let (server, _) = ServerExchange::start(ServerCredential::Known(&verifier), b"n,,n=,r=abc")
            .expect("first");
        assert_eq!(
            server.finish(b"c=biws,r=abc,p=AAAA"),
            Err(ScramFault::Protocol)
        );
        let expected = ExpectedSignature([3; 32]);
        assert_eq!(expected.check(b"e=invalid-proof"), Err(ScramFault::Failed));
        assert_eq!(
            expected.check(text::encode_server_final(&[4; 32]).as_bytes()),
            Err(ScramFault::Failed)
        );
        assert_eq!(
            expected.check(text::encode_server_final(&[3; 32]).as_bytes()),
            Ok(())
        );
        assert_eq!(expected.check(b"v=AAAA"), Err(ScramFault::Protocol));
        assert_eq!(format!("{expected:?}"), REDACTED);
        assert_eq!(format!("{:?}", derive_keys(&pencil(), b"s", 1)), REDACTED);
    }

    #[test]
    fn nonces_are_random_and_printable() {
        let a = new_nonce().expect("random");
        let b = new_nonce().expect("random");
        assert_ne!(a, b);
        assert_eq!(a.len(), 24);
    }
}
