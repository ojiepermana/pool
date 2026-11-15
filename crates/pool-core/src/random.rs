//! The one source of random bytes in the product (spec 0004, "Workspace", "Random bytes").
//!
//! It fills a slice from the generator of `aws-lc-rs`. A failure is never
//! replaced by another value: no clock, no counter, no fixed bytes.

use std::fmt;

/// The operating system gave no random bytes. It carries no detail and prints its name only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomError;

impl fmt::Display for RandomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RandomError")
    }
}

impl std::error::Error for RandomError {}

/// Fills `out` with random bytes of the operating system.
pub fn fill_random(out: &mut [u8]) -> Result<(), RandomError> {
    aws_lc_rs::rand::fill(out).map_err(|_| RandomError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_fills_differ_and_the_error_prints_its_name() {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        fill_random(&mut a).expect("random");
        fill_random(&mut b).expect("random");
        assert_ne!(a, b);
        assert_eq!(RandomError.to_string(), "RandomError");
    }
}
