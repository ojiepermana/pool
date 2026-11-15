//! Identifier types shared by more than one crate.
//!
//! Every identifier is its own type that wraps one value. Two different kinds
//! never compare with each other and never convert into each other. An
//! identifier is built only by a function that checks its syntax: there is no
//! `From<String>` and no `Default`. The rule is owned by spec 0003, section
//! "Identifier and generation types".

use std::fmt;

/// The largest length, in bytes, of a [`ClusterId`], [`NodeId`], [`IdentityId`]
/// or [`PrincipalId`].
const MAX_NAME_ID_BYTES: usize = 63;

/// Checks the syntax shared by [`ClusterId`], [`NodeId`], [`IdentityId`] and
/// [`PrincipalId`]:
/// 1 to 63 bytes, the first `a` to `z`, the rest `a` to `z`, `0` to `9`, `_`
/// or `-`.
fn is_name_id(text: &str) -> bool {
    let bytes = text.as_bytes();
    let Some((first, rest)) = bytes.split_first() else {
        return false;
    };
    bytes.len() <= MAX_NAME_ID_BYTES
        && first.is_ascii_lowercase()
        && rest.iter().all(|&byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}

macro_rules! name_id {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            /// Checks the text and builds the identifier. Returns `None` when
            /// the text is not 1 to 63 bytes, the first `a` to `z` and the rest
            /// `a` to `z`, `0` to `9`, `_` or `-`.
            pub fn parse(text: &str) -> Option<$name> {
                is_name_id(text).then(|| $name(text.to_owned()))
            }

            /// The identifier text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

name_id! {
    /// The identifier of one cluster, the `id` of a `[[cluster]]` entry.
    ///
    /// It compares only with another `ClusterId`:
    ///
    /// ```compile_fail
    /// fn same(node: &pool_config::NodeId, cluster: &pool_config::ClusterId) -> bool {
    ///     node == cluster
    /// }
    /// ```
    ClusterId
}

name_id! {
    /// The identifier of one node (a configured PostgreSQL server), the `id`
    /// of a `[[node]]` entry. It compares only with another `NodeId`.
    NodeId
}

name_id! {
    /// The identifier of one entry of the identity map, the `id` of an
    /// `[[identity]]` entry. It may appear in telemetry in place of the login
    /// it stands for. It compares only with another `IdentityId`.
    IdentityId
}

name_id! {
    /// The identifier of one frontend principal, the `id` of a
    /// `[[principal]]` entry. Logs may show it in place of the user name. It
    /// compares only with another `PrincipalId`:
    ///
    /// ```compile_fail
    /// fn same(principal: &pool_config::PrincipalId, identity: &pool_config::IdentityId) -> bool {
    ///     principal == identity
    /// }
    /// ```
    PrincipalId
}

/// The PostgreSQL system identifier of a cluster: a whole number from 1 to
/// 18446744073709551615. In text it is decimal digits with no leading zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemIdentifier(u64);

impl SystemIdentifier {
    /// Checks the text and builds the identifier. Returns `None` for text that
    /// is empty, holds anything but decimal digits, starts with `0`, or is
    /// larger than 18446744073709551615.
    pub fn parse(text: &str) -> Option<SystemIdentifier> {
        let bytes = text.as_bytes();
        let first = *bytes.first()?;
        if first == b'0' || !bytes.iter().all(u8::is_ascii_digit) {
            return None;
        }
        text.parse::<u64>().ok().map(SystemIdentifier)
    }

    /// The number.
    pub fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for SystemIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The identity of one journal: 16 bytes. In text it is exactly 32
/// characters from `0` to `9` and `a` to `f`.
///
/// It is not a secret. The bootstrap command prints it, and the key
/// `control.authority_id` names it as the authority witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorityId([u8; 16]);

impl AuthorityId {
    /// Checks the text and builds the identifier. Returns `None` unless the
    /// text is exactly 32 characters from `0` to `9` and `a` to `f`.
    pub fn parse(text: &str) -> Option<AuthorityId> {
        let digits = text.as_bytes();
        if digits.len() != 32 {
            return None;
        }
        let mut bytes = [0_u8; 16];
        let (pairs, _) = digits.as_chunks::<2>();
        for (byte, &[high, low]) in bytes.iter_mut().zip(pairs) {
            let (high, low) = (lower_hex_value(high)?, lower_hex_value(low)?);
            *byte = (high << 4) | low;
        }
        Some(AuthorityId(bytes))
    }

    /// Builds the identifier from its 16 bytes.
    pub fn from_bytes(bytes: [u8; 16]) -> AuthorityId {
        AuthorityId(bytes)
    }

    /// The 16 bytes.
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for AuthorityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// The value of one lowercase hexadecimal digit.
fn lower_hex_value(digit: u8) -> Option<u8> {
    match digit {
        b'0'..=b'9' => Some(digit - b'0'),
        b'a'..=b'f' => Some(digit - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_ids_accept_their_syntax() {
        for text in ["a", "main", "pg1", "orders-app", "a_b-c9", &"a".repeat(63)] {
            assert_eq!(
                ClusterId::parse(text).map(|id| id.as_str().to_owned()),
                Some(text.to_owned())
            );
            assert!(NodeId::parse(text).is_some());
            assert!(IdentityId::parse(text).is_some());
            assert!(PrincipalId::parse(text).is_some());
        }
    }

    #[test]
    fn name_ids_refuse_text_outside_their_syntax() {
        let long = "a".repeat(64);
        for text in [
            "", "1pg", "-a", "_a", "PG1", "pG", "a b", "a.b", "a/b", "é", "a\n", &long,
        ] {
            assert_eq!(ClusterId::parse(text), None, "{text:?}");
            assert_eq!(NodeId::parse(text), None, "{text:?}");
            assert_eq!(IdentityId::parse(text), None, "{text:?}");
            assert_eq!(PrincipalId::parse(text), None, "{text:?}");
        }
    }

    #[test]
    fn name_ids_compare_exactly() {
        assert_eq!(NodeId::parse("pg1"), NodeId::parse("pg1"));
        assert_ne!(NodeId::parse("pg1"), NodeId::parse("pg10"));
        assert!(NodeId::parse("pg1") < NodeId::parse("pg2"));
        assert_eq!(
            NodeId::parse("pg1").map(|id| id.to_string()),
            Some("pg1".to_owned())
        );
    }

    #[test]
    fn system_identifier_accepts_one_to_u64_max() {
        assert_eq!(
            SystemIdentifier::parse("1").map(SystemIdentifier::get),
            Some(1)
        );
        assert_eq!(
            SystemIdentifier::parse("7412345678901234567").map(SystemIdentifier::get),
            Some(7_412_345_678_901_234_567)
        );
        assert_eq!(
            SystemIdentifier::parse("18446744073709551615").map(SystemIdentifier::get),
            Some(u64::MAX)
        );
        assert_eq!(
            SystemIdentifier::parse("42").map(|id| id.to_string()),
            Some("42".to_owned())
        );
    }

    #[test]
    fn system_identifier_refuses_other_text() {
        for text in [
            "",
            "0",
            "007",
            "18446744073709551616",
            "99999999999999999999",
            "12a",
            "+1",
            "-1",
            " 1",
            "1 ",
        ] {
            assert_eq!(SystemIdentifier::parse(text), None, "{text:?}");
        }
    }

    #[test]
    fn authority_id_round_trips() {
        let text = "0123456789abcdef0123456789abcdef";
        let id = AuthorityId::parse(text);
        assert_eq!(id.map(|id| id.to_string()), Some(text.to_owned()));
        assert_eq!(
            id.map(|id| *id.as_bytes()),
            Some([
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef
            ])
        );
        let bytes = [0xff; 16];
        assert_eq!(AuthorityId::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(
            AuthorityId::from_bytes(bytes).to_string(),
            "ffffffffffffffffffffffffffffffff"
        );
        assert_eq!(id, AuthorityId::parse(text));
        assert_ne!(id, Some(AuthorityId::from_bytes([0; 16])));
    }

    #[test]
    fn authority_id_refuses_other_text() {
        for text in [
            "",
            "0123456789abcdef0123456789abcde",
            "0123456789abcdef0123456789abcdef0",
            "0123456789ABCDEF0123456789abcdef",
            "0123456789abcdef0123456789abcdeg",
            "0123456789abcdef 123456789abcdef",
            "not-an-authority-id",
        ] {
            assert_eq!(AuthorityId::parse(text), None, "{text:?}");
        }
    }
}

#[cfg(test)]
mod edge_tests {
    //! Edges added by /test (0003 AC-6): identifiers count bytes, accept only
    //! ASCII, and compare exactly.

    use super::*;

    #[test]
    fn ac6_name_ids_refuse_letters_that_only_look_ascii() {
        for text in [
            "\u{440}g1",
            "p\u{261}1",
            "\u{ff50}\u{ff47}1",
            "pg\u{200b}1",
            "pg\u{661}",
            "pg1\0",
            "pg1\u{7f}",
            "\u{130}",
        ] {
            assert_eq!(ClusterId::parse(text), None, "{text:?}");
            assert_eq!(NodeId::parse(text), None, "{text:?}");
            assert_eq!(IdentityId::parse(text), None, "{text:?}");
        }
    }

    #[test]
    fn ac6_name_ids_accept_a_trailing_dash_or_underscore_and_keep_the_text() {
        for text in ["a-", "a_", "a--", "z9", "a-_-_9"] {
            assert_eq!(
                NodeId::parse(text).map(|id| id.to_string()),
                Some(text.to_owned())
            );
        }
        // Different kinds hold the same text and stay different types; each
        // compares only with its own kind.
        assert_eq!(ClusterId::parse("main"), ClusterId::parse("main"));
        assert_ne!(ClusterId::parse("main"), ClusterId::parse("main-"));
        assert_ne!(NodeId::parse("pg1"), NodeId::parse("pg1_"));
    }

    #[test]
    fn ac6_system_identifier_refuses_other_digits_and_orders_by_number() {
        for text in ["\u{661}", "1\u{661}", "\u{ff11}", "1.0", "1,000", "1_000"] {
            assert_eq!(SystemIdentifier::parse(text), None, "{text:?}");
        }
        assert!(SystemIdentifier::parse("9") < SystemIdentifier::parse("10"));
        assert_eq!(
            SystemIdentifier::parse("18446744073709551615").map(|id| id.to_string()),
            Some("18446744073709551615".to_owned())
        );
    }

    #[test]
    fn ac6_authority_id_counts_bytes_and_round_trips_every_byte() {
        // Thirty two bytes that are not thirty two hex digits.
        assert_eq!(AuthorityId::parse(&"\u{e9}".repeat(16)), None);
        assert_eq!(
            AuthorityId::parse(&format!("{}\u{e9}", "a".repeat(30))),
            None
        );
        // Every byte value survives text and back.
        for value in 0..=u8::MAX {
            let bytes = [value; 16];
            let text = AuthorityId::from_bytes(bytes).to_string();
            assert_eq!(text.len(), 32);
            assert_eq!(
                AuthorityId::parse(&text),
                Some(AuthorityId::from_bytes(bytes))
            );
        }
        // One differing byte at either end is another identity.
        let mut first = [0_u8; 16];
        first[0] = 1;
        let mut last = [0_u8; 16];
        last[15] = 1;
        assert_ne!(
            AuthorityId::from_bytes(first),
            AuthorityId::from_bytes(last)
        );
        assert_ne!(
            AuthorityId::from_bytes(first),
            AuthorityId::from_bytes([0; 16])
        );
    }
}
