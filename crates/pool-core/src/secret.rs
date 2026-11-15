//! Secret files and the types that hold what they contain (spec 0004,
//! "State directory and protected files").
//!
//! A secret file (a credential, a verifier, the private key of the
//! listener) must be a regular file after links are followed, with no group
//! and no other permission bit, owned by the service account: the owner of
//! the state directory, which the caller passes. A certificate chain or a
//! CA file need only be a regular file that can be read. Every file is read
//! through a limit. All checks are for Unix.

use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

use pool_protocol::base64;
use rustls::RootCertStore;
use rustls::pki_types::pem::{PemObject, SectionKind};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

use crate::REDACTED;

/// The largest credential or verifier file.
pub const CREDENTIAL_MAX_BYTES: u64 = 4096;
/// The largest private key file.
pub const PRIVATE_KEY_MAX_BYTES: u64 = 65536;
/// The largest certificate chain or CA file.
pub const CERTIFICATE_MAX_BYTES: u64 = 1_048_576;
/// The longest password, after the trailing line end is dropped.
pub const PASSWORD_MAX_BYTES: usize = 1024;
/// The highest iteration count of a verifier.
pub const VERIFIER_MAX_ITERATIONS: u32 = 600_000;
/// The longest salt of a verifier, after base64 decoding.
pub const VERIFIER_MAX_SALT_BYTES: usize = 64;

/// Why a file named by the configuration cannot be used. It names no path and repeats no content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecretError {
    /// The file does not exist.
    NotFound,
    /// A secret file has the wrong type, mode or owner, or cannot be opened for that reason.
    Permissions,
    /// The file does not hold what its key needs, or is longer than its limit.
    Invalid,
}

impl SecretError {
    /// The finding code of spec 0004, "Findings of the new commands".
    pub fn code(self) -> &'static str {
        match self {
            Self::NotFound => "secret.not_found",
            Self::Permissions => "secret.permissions",
            Self::Invalid => "secret.invalid",
        }
    }

    /// The constant message of the finding.
    pub fn message(self) -> &'static str {
        match self {
            Self::NotFound => "a file named by the configuration does not exist",
            Self::Permissions => "a secret file has the wrong type, mode or owner",
            Self::Invalid => "a file named by the configuration does not hold what its key needs",
        }
    }
}

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.message())
    }
}

impl std::error::Error for SecretError {}

fn open_error(error: &io::Error) -> SecretError {
    match error.kind() {
        io::ErrorKind::NotFound => SecretError::NotFound,
        io::ErrorKind::PermissionDenied => SecretError::Permissions,
        _ => SecretError::Invalid,
    }
}

/// Checks a secret file: a regular file after links, mode with no group or other bit, owned by `owner`.
pub fn check_secret_file(path: &Path, owner: u32) -> Result<(), SecretError> {
    let metadata = std::fs::metadata(path).map_err(|e| open_error(&e))?;
    let mode = metadata.permissions().mode();
    if !metadata.is_file() || mode & 0o077 != 0 || metadata.uid() != owner {
        return Err(SecretError::Permissions);
    }
    Ok(())
}

/// Reads a whole file of at most `limit` bytes. A longer file is [`SecretError::Invalid`].
fn read_limited(path: &Path, limit: u64) -> Result<Vec<u8>, SecretError> {
    let file = File::open(path).map_err(|e| open_error(&e))?;
    if !file.metadata().map_err(|e| open_error(&e))?.is_file() {
        return Err(SecretError::Invalid);
    }
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| SecretError::Invalid)?;
    if bytes.len() as u64 > limit {
        return Err(SecretError::Invalid);
    }
    Ok(bytes)
}

fn read_secret(path: &Path, owner: u32, limit: u64) -> Result<Vec<u8>, SecretError> {
    check_secret_file(path, owner)?;
    read_limited(path, limit)
}

/// Drops one trailing line feed, or carriage return and line feed.
fn drop_line_end(bytes: &[u8]) -> &[u8] {
    bytes
        .strip_suffix(b"\r\n")
        .or_else(|| bytes.strip_suffix(b"\n"))
        .unwrap_or(bytes)
}

/// The password of a backend login or a monitor login. `Debug` prints `[REDACTED]`.
#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    /// Checks the content of a credential file: one trailing line end is
    /// dropped, then 1 to 1024 printable ASCII characters from U+0020 to U+007E.
    pub fn parse(bytes: &[u8]) -> Result<Self, SecretError> {
        let bytes = drop_line_end(bytes);
        if bytes.is_empty()
            || bytes.len() > PASSWORD_MAX_BYTES
            || !bytes.iter().all(|b| (0x20..=0x7e).contains(b))
        {
            return Err(SecretError::Invalid);
        }
        let text = std::str::from_utf8(bytes).map_err(|_| SecretError::Invalid)?;
        Ok(Self(text.to_string()))
    }

    /// The password bytes, for the key derivation only.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

/// Reads a credential file, after the secret file check.
pub fn read_credential(path: &Path, owner: u32) -> Result<Password, SecretError> {
    Password::parse(&read_secret(path, owner, CREDENTIAL_MAX_BYTES)?)
}

/// A SCRAM-SHA-256 verifier in the form PostgreSQL stores. `Debug` prints `[REDACTED]`.
#[derive(Clone, PartialEq, Eq)]
pub struct Verifier {
    pub(crate) iterations: u32,
    pub(crate) salt: Vec<u8>,
    pub(crate) stored_key: [u8; 32],
    pub(crate) server_key: [u8; 32],
}

impl Verifier {
    /// Parses `SCRAM-SHA-256$<count>:<salt>$<stored key>:<server key>`, after
    /// one trailing line end is dropped. The count is 1 to 600000, the salt
    /// 1 to 64 bytes and both keys 32 bytes after base64 decoding.
    pub fn parse(bytes: &[u8]) -> Result<Self, SecretError> {
        let invalid = SecretError::Invalid;
        let text = std::str::from_utf8(drop_line_end(bytes)).map_err(|_| invalid)?;
        let rest = text.strip_prefix("SCRAM-SHA-256$").ok_or(invalid)?;
        let (count_salt, keys) = rest.split_once('$').ok_or(invalid)?;
        let (count, salt) = count_salt.split_once(':').ok_or(invalid)?;
        let (stored, server) = keys.split_once(':').ok_or(invalid)?;
        if count.is_empty() || count.len() > 7 || !count.bytes().all(|b| b.is_ascii_digit()) {
            return Err(invalid);
        }
        let iterations: u32 = count.parse().map_err(|_| invalid)?;
        let salt = base64::decode(salt.as_bytes()).map_err(|_| invalid)?;
        let key = |text: &str| -> Result<[u8; 32], SecretError> {
            base64::decode(text.as_bytes())
                .map_err(|_| invalid)?
                .try_into()
                .map_err(|_| invalid)
        };
        if !(1..=VERIFIER_MAX_ITERATIONS).contains(&iterations)
            || salt.is_empty()
            || salt.len() > VERIFIER_MAX_SALT_BYTES
        {
            return Err(invalid);
        }
        Ok(Self {
            iterations,
            salt,
            stored_key: key(stored)?,
            server_key: key(server)?,
        })
    }

    /// The iteration count, which is not secret: a client sees it.
    pub fn iterations(&self) -> u32 {
        self.iterations
    }
}

impl fmt::Debug for Verifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

/// Reads a verifier file, after the secret file check.
pub fn read_verifier(path: &Path, owner: u32) -> Result<Verifier, SecretError> {
    Verifier::parse(&read_secret(path, owner, CREDENTIAL_MAX_BYTES)?)
}

/// The private key of the listener. `Debug` prints `[REDACTED]`.
pub struct PrivateKey(pub(crate) PrivateKeyDer<'static>);

impl PrivateKey {
    /// Parses a PEM file that holds exactly one private key in PKCS 8, PKCS 1 or SEC 1 form.
    pub fn parse(pem: &[u8]) -> Result<Self, SecretError> {
        let mut key = None;
        for section in <(SectionKind, Vec<u8>)>::pem_slice_iter(pem) {
            let (kind, der) = section.map_err(|_| SecretError::Invalid)?;
            if matches!(
                kind,
                SectionKind::PrivateKey | SectionKind::RsaPrivateKey | SectionKind::EcPrivateKey
            ) {
                if key.is_some() {
                    return Err(SecretError::Invalid);
                }
                key = PrivateKeyDer::from_pem(kind, der);
            }
        }
        key.map(Self).ok_or(SecretError::Invalid)
    }
}

impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        Self(self.0.clone_key())
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

/// Reads the private key file of the listener, after the secret file check.
pub fn read_private_key(path: &Path, owner: u32) -> Result<PrivateKey, SecretError> {
    PrivateKey::parse(&read_secret(path, owner, PRIVATE_KEY_MAX_BYTES)?)
}

fn certificates(pem: &[u8]) -> Result<Vec<CertificateDer<'static>>, SecretError> {
    let certs = CertificateDer::pem_slice_iter(pem)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SecretError::Invalid)?;
    if certs.is_empty() {
        return Err(SecretError::Invalid);
    }
    Ok(certs)
}

/// A certificate chain in PEM, leaf first. Its certificates carry names from a file: `Debug` prints the count only.
#[derive(Clone)]
pub struct CertificateChain(pub(crate) Vec<CertificateDer<'static>>);

impl fmt::Debug for CertificateChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CertificateChain")
            .field("certificates", &self.0.len())
            .finish()
    }
}

impl CertificateChain {
    /// Parses one or more certificates in PEM.
    pub fn parse(pem: &[u8]) -> Result<Self, SecretError> {
        certificates(pem).map(Self)
    }
}

/// Reads the certificate chain file of the listener.
pub fn read_certificate_chain(path: &Path) -> Result<CertificateChain, SecretError> {
    CertificateChain::parse(&read_limited(path, CERTIFICATE_MAX_BYTES)?)
}

/// The trust roots for one node: the certificates of its CA file and nothing else. `Debug` prints the count only.
#[derive(Clone)]
pub struct TrustRoots(pub(crate) RootCertStore);

impl fmt::Debug for TrustRoots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrustRoots")
            .field("roots", &self.0.len())
            .finish()
    }
}

impl TrustRoots {
    /// Parses a CA file: at least one certificate in PEM, each usable as a trust anchor.
    pub fn parse(pem: &[u8]) -> Result<Self, SecretError> {
        let mut store = RootCertStore::empty();
        for cert in certificates(pem)? {
            store.add(cert).map_err(|_| SecretError::Invalid)?;
        }
        Ok(Self(store))
    }
}

/// Reads a CA file.
pub fn read_trust_roots(path: &Path) -> Result<TrustRoots, SecretError> {
    TrustRoots::parse(&read_limited(path, CERTIFICATE_MAX_BYTES)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The form PostgreSQL stores, with made up keys of the right length.
    const VERIFIER: &str = "SCRAM-SHA-256$4096:W22ZaJ0SNY7soEsUEjb6gQ==$AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=:AgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgI=";

    #[test]
    fn passwords() {
        assert_eq!(
            Password::parse(b"pencil\n").map(|p| p.0),
            Ok("pencil".into())
        );
        assert_eq!(
            Password::parse(b"pencil\r\n").map(|p| p.0),
            Ok("pencil".into())
        );
        assert_eq!(Password::parse(b" a~").map(|p| p.0), Ok(" a~".into()));
        assert!(Password::parse(&[b'x'; 1024]).is_ok());
        for bad in [
            &b""[..],
            b"\n",
            b"pencil\n\n",
            b"pen\tcil",
            b"pen\x7fcil",
            "pénc".as_bytes(),
            &[b'x'; 1025],
            b"pencil\r",
        ] {
            assert_eq!(Password::parse(bad), Err(SecretError::Invalid), "{bad:?}");
        }
        assert_eq!(
            format!("{:?}", Password::parse(b"pencil").expect("valid")),
            REDACTED
        );
    }

    #[test]
    fn verifiers() {
        let v = Verifier::parse(VERIFIER.as_bytes()).expect("valid");
        assert_eq!(v.iterations(), 4096);
        assert_eq!(v.salt.len(), 16);
        assert!(Verifier::parse(format!("{VERIFIER}\n").as_bytes()).is_ok());
        assert_eq!(format!("{v:?}"), REDACTED);
        let key = base64::encode(&[1; 32]);
        let ok = |count: &str, salt: &[u8]| {
            Verifier::parse(
                format!("SCRAM-SHA-256${count}:{}${key}:{key}", base64::encode(salt)).as_bytes(),
            )
        };
        assert!(ok("1", &[1]).is_ok());
        assert!(ok("600000", &[1; 64]).is_ok());
        for (count, salt) in [
            ("0", &[1u8][..]),
            ("600001", &[1]),
            ("", &[1]),
            ("+1", &[1]),
            ("1", &[]),
            ("1", &[1; 65]),
        ] {
            assert_eq!(ok(count, salt), Err(SecretError::Invalid), "{count}");
        }
        for bad in [
            "md5abcdef",
            "SCRAM-SHA-1$4096:c2FsdA==$AAAA:AAAA",
            "SCRAM-SHA-256$4096:c2FsdA==$AAAA:AAAA",
            "SCRAM-SHA-256$4096$c2FsdA==",
            "pencil",
        ] {
            assert_eq!(
                Verifier::parse(bad.as_bytes()),
                Err(SecretError::Invalid),
                "{bad}"
            );
        }
    }

    /// A private directory for one test, removed when dropped.
    struct TempDir(std::path::PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("pool-core-{}-{name}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir(&dir).expect("temp dir");
            Self(dir)
        }

        fn owner(&self) -> u32 {
            std::fs::metadata(&self.0).expect("metadata").uid()
        }

        fn file(&self, name: &str, content: &[u8], mode: u32) -> std::path::PathBuf {
            let path = self.0.join(name);
            std::fs::write(&path, content).expect("write");
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(mode)).expect("chmod");
            path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn secret_files_need_owner_only_mode_a_regular_file_and_the_owner() {
        let dir = TempDir::new("modes");
        let owner = dir.owner();
        for mode in [0o600, 0o400] {
            let path = dir.file(&format!("ok-{mode:o}"), b"pencil\n", mode);
            assert_eq!(
                read_credential(&path, owner).map(|p| p.0),
                Ok("pencil".into()),
                "{mode:o}"
            );
        }
        for mode in [0o640, 0o604, 0o660, 0o644, 0o610, 0o601, 0o700 | 0o020] {
            let path = dir.file(&format!("bad-{mode:o}"), b"pencil\n", mode);
            assert_eq!(
                read_credential(&path, owner),
                Err(SecretError::Permissions),
                "{mode:o}"
            );
        }
        let good = dir.file("good", b"pencil", 0o600);
        assert_eq!(
            read_credential(&good, owner.wrapping_add(1)),
            Err(SecretError::Permissions)
        );
        assert_eq!(
            read_credential(&dir.0, owner),
            Err(SecretError::Permissions),
            "a directory"
        );
        assert_eq!(
            read_credential(&dir.0.join("missing"), owner),
            Err(SecretError::NotFound)
        );
        // A link is followed: the target is what is checked.
        let link = dir.0.join("link");
        std::os::unix::fs::symlink(&good, &link).expect("symlink");
        assert!(read_credential(&link, owner).is_ok());
        let loose = dir.file("loose", b"pencil", 0o644);
        let loose_link = dir.0.join("loose-link");
        std::os::unix::fs::symlink(&loose, &loose_link).expect("symlink");
        assert_eq!(
            read_credential(&loose_link, owner),
            Err(SecretError::Permissions)
        );
        let dangling = dir.0.join("dangling");
        std::os::unix::fs::symlink(dir.0.join("nothing"), &dangling).expect("symlink");
        assert_eq!(
            read_credential(&dangling, owner),
            Err(SecretError::NotFound)
        );
    }

    #[test]
    fn files_are_read_through_their_limit() {
        let dir = TempDir::new("limits");
        let owner = dir.owner();
        let long = dir.file("long", &[b'x'; CREDENTIAL_MAX_BYTES as usize + 1], 0o600);
        assert_eq!(read_credential(&long, owner), Err(SecretError::Invalid));
        let verifier = dir.file("verifier", format!("{VERIFIER}\n").as_bytes(), 0o600);
        assert!(read_verifier(&verifier, owner).is_ok());
        let fixtures = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
        let key = dir.file(
            "key",
            &std::fs::read(format!("{fixtures}/node.key")).expect("key"),
            0o600,
        );
        assert!(read_private_key(&key, owner).is_ok());
        assert_eq!(
            format!("{:?}", read_private_key(&key, owner).expect("key")),
            REDACTED
        );
        let open_key = dir.file(
            "open-key",
            &std::fs::read(format!("{fixtures}/node.key")).expect("key"),
            0o644,
        );
        assert_eq!(
            read_private_key(&open_key, owner).map(|_| ()),
            Err(SecretError::Permissions)
        );
        // Certificates need only be readable files.
        let chain = dir.file(
            "chain",
            &std::fs::read(format!("{fixtures}/node.pem")).expect("pem"),
            0o644,
        );
        assert!(read_certificate_chain(&chain).is_ok());
        let debug = format!(
            "{:?} {:?}",
            read_certificate_chain(&chain).expect("chain"),
            read_trust_roots(&chain).expect("roots")
        );
        assert_eq!(
            debug,
            "CertificateChain { certificates: 1 } TrustRoots { roots: 1 }"
        );
        assert!(read_trust_roots(&chain).is_ok());
        let empty = dir.file("empty", b"no certificate here\n", 0o644);
        assert_eq!(
            read_certificate_chain(&empty).map(|_| ()),
            Err(SecretError::Invalid)
        );
        assert_eq!(
            read_trust_roots(&empty).map(|_| ()),
            Err(SecretError::Invalid)
        );
        assert_eq!(
            read_trust_roots(&dir.0.join("missing")).map(|_| ()),
            Err(SecretError::NotFound)
        );
        let big = dir.file(
            "big",
            &vec![b'a'; CERTIFICATE_MAX_BYTES as usize + 1],
            0o644,
        );
        assert_eq!(
            read_trust_roots(&big).map(|_| ()),
            Err(SecretError::Invalid)
        );
    }

    #[test]
    fn a_key_file_holds_exactly_one_key() {
        let fixtures = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
        let key = std::fs::read(format!("{fixtures}/node.key")).expect("key");
        let cert = std::fs::read(format!("{fixtures}/node.pem")).expect("pem");
        assert!(PrivateKey::parse(&key).is_ok());
        let mut with_cert = cert.clone();
        with_cert.extend_from_slice(&key);
        assert!(
            PrivateKey::parse(&with_cert).is_ok(),
            "other sections are skipped"
        );
        let mut two = key.clone();
        two.extend_from_slice(&key);
        assert_eq!(
            PrivateKey::parse(&two).map(|_| ()),
            Err(SecretError::Invalid)
        );
        assert_eq!(
            PrivateKey::parse(&cert).map(|_| ()),
            Err(SecretError::Invalid)
        );
        assert_eq!(
            PrivateKey::parse(b"").map(|_| ()),
            Err(SecretError::Invalid)
        );
    }

    #[test]
    fn error_codes_and_messages() {
        assert_eq!(SecretError::NotFound.code(), "secret.not_found");
        assert_eq!(SecretError::Permissions.code(), "secret.permissions");
        assert_eq!(SecretError::Invalid.code(), "secret.invalid");
        assert_eq!(
            SecretError::Permissions.to_string(),
            "secret.permissions: a secret file has the wrong type, mode or owner"
        );
    }
}
