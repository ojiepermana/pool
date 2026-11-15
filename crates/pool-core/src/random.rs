//! Random bytes of the operating system, through `aws-lc-rs`: the one source
//! of random bytes in the product (spec 0004, "Workspace", "Random bytes").
//!
//! The commands of `apps/pool` call it for the authority ID, so that binary
//! does not depend on `aws-lc-rs` itself.

use std::fmt;

use aws_lc_rs::rand;

/// The operating system gave no random bytes. It carries no detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomError;

impl RandomError {
    /// The finding code of spec 0004, "Findings of the new commands".
    pub fn code(self) -> &'static str {
        "random.failed"
    }

    /// The constant message of the finding.
    pub fn message(self) -> &'static str {
        "random bytes could not be obtained"
    }
}

impl fmt::Display for RandomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for RandomError {}

/// Fills `out` with random bytes of the operating system, through
/// `aws-lc-rs`. It allocates nothing.
///
/// A failure is returned as [`RandomError`] and is never replaced by another
/// value: no clock, no counter and no fixed bytes. What `out` holds after a
/// failure must not be used.
pub fn fill_random(out: &mut [u8]) -> Result<(), RandomError> {
    rand::fill(out).map_err(|_| RandomError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_fills_differ_and_an_empty_one_succeeds() {
        let (mut first, mut second) = ([0_u8; 16], [0_u8; 16]);
        fill_random(&mut first).expect("random bytes");
        fill_random(&mut second).expect("random bytes");
        assert_ne!(first, second);
        assert_eq!(fill_random(&mut []), Ok(()));
        assert_eq!(RandomError.code(), "random.failed");
        assert_eq!(RandomError.message(), "random bytes could not be obtained");
        assert_eq!(format!("{RandomError:?}"), "RandomError");
    }
}
