//! Scope 005 tooling debt: native build settings, private keys and bundled notices.

use crate::report::Report;
use crate::text::{baseline_lines, key_value, strip_comment};
use crate::tree::Tree;

/// The fields used from one Cargo-generated package entry.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct LockEntry {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

/// Read Cargo's canonical lockfile shape, including qualified dependency names.
/// A new table ends an entry; dependency arrays may be inline or multiline.
pub fn lock_entries(text: &str) -> Vec<LockEntry> {
    let mut packages = Vec::new();
    let mut current: Option<LockEntry> = None;
    let mut dependencies = false;
    for raw in text.lines() {
        let line = strip_comment(raw).trim();
        if line.starts_with('[') {
            if let Some(entry) = current.take() {
                packages.push(entry);
            }
            dependencies = false;
            if line == "[[package]]" {
                current = Some(LockEntry::default());
            }
            continue;
        }
        let Some(entry) = current.as_mut() else {
            continue;
        };
        let mut values = None;
        if let Some((key, value)) = key_value(line) {
            match key {
                "name" => entry.name = value.trim_matches('"').into(),
                "version" => entry.version = value.trim_matches('"').into(),
                "dependencies" => {
                    dependencies = true;
                    values = Some(value);
                }
                _ => {}
            }
        } else if dependencies {
            values = Some(line);
        }
        if let Some(values) = values {
            let mut parts = values.split('"');
            parts.next();
            while let Some(value) = parts.next() {
                if let Some(name) = value.split_whitespace().next() {
                    entry.dependencies.push(name.into());
                }
                parts.next();
            }
            if values.contains(']') {
                dependencies = false;
            }
        }
    }
    if let Some(entry) = current {
        packages.push(entry);
    }
    packages
}

pub fn check(tree: &dyn Tree, report: &mut Report) {
    native_build(tree, report);
    private_keys(tree, report);
    notices(tree, report);
}

const NATIVE_ENV: [&str; 6] = [
    "AWS_LC_SYS_USE_SYSTEM",
    "AWS_LC_SYS_USE_SYSTEM_aarch64_unknown_linux_gnu",
    "AWS_LC_SYS_USE_SYSTEM_x86_64_unknown_linux_gnu",
    "AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin",
    "AWS_LC_SYS_USE_SYSTEM_x86_64_apple_darwin",
    "LIBSQLITE3_SYS_USE_PKG_CONFIG",
];

fn native_build(tree: &dyn Tree, report: &mut Report) {
    const PATH: &str = ".cargo/config.toml";
    let Some(text) = report.need(tree, PATH) else {
        return;
    };
    // Exactly one env table with the six forced values; reject overrides in another table,
    // duplicated keys, extra environment settings and build/rustflags settings too.
    let actual: Vec<_> = text
        .lines()
        .map(strip_comment)
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let has_env_header = actual.first() == Some(&"[env]");
    let mut expected = vec!["[env]".to_string()];
    expected.extend(NATIVE_ENV.map(|key| format!("{key} = {{ value = \"0\", force = true }}")));
    let mut sorted_actual: Vec<_> = actual.into_iter().map(str::to_string).collect();
    sorted_actual.sort();
    expected.sort();
    if !has_env_header || sorted_actual != expected {
        report.add("W8", PATH, "expected exactly [env] and the six native-library settings with value = \"0\" and force = true");
    }
}

const TEST_KEY_PATH: &str = "crates/pool-core/tests/fixtures/node.key";
// Split the PEM header so this guard's own source does not contain a private key block.
const TEST_KEY: &str = concat!(
    "test key for node.test, used only by the tests of pool-core; it protects nothing\n",
    "-----BEGIN ",
    "PRIVATE KEY-----\n",
    "MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgZydDCR4/kksv+8QQ\n",
    "pt1MGziFJ6fgD3PKJXlNCsRKmSqhRANCAARL2ru3KHsZIT43LmLKSom4tXacDo1P\n",
    "N3iW24PuXkV1l3lYX47YWbbINySWd2ZN75PzgoBF14oLc3Tsk/sWEE0N\n",
    "-----END ",
    "PRIVATE KEY-----\n",
);

fn private_keys(tree: &dyn Tree, report: &mut Report) {
    // Do not exclude hidden source folders: .git and generated target outputs are the only skips.
    for path in report.walk(tree, "", &|name| name == ".git" || name == "target") {
        let bytes = match tree.read_bytes(&path) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => continue,
            Err(error) => {
                report.fault(format!("cannot read {error}"));
                continue;
            }
        };
        if path == TEST_KEY_PATH && bytes == TEST_KEY.as_bytes() {
            continue;
        }
        // Scan bytes, so a binary prefix or a file without a text extension cannot hide a key.
        let text = String::from_utf8_lossy(&bytes);
        for (index, line) in text.lines().enumerate() {
            for (start, _) in line.match_indices("-----BEGIN ") {
                let after = &line[start + "-----BEGIN ".len()..];
                if let Some((label, _)) = after.split_once("-----")
                    && label.chars().all(|c| {
                        c.is_ascii_uppercase() || c.is_ascii_digit() || matches!(c, ' ' | '-')
                    })
                    && (label == "PRIVATE KEY" || label.ends_with(" PRIVATE KEY"))
                {
                    report.add_at("W9", &path, index + 1,
                    "private key block is forbidden; only the exact committed node.test key is allowed");
                }
            }
        }
    }
}

fn notices(tree: &dyn Tree, report: &mut Report) {
    const BASELINE: &str = "tools/repo-check/baseline/native-library-versions.txt";
    let Some(lock) = report.need(tree, "Cargo.lock") else {
        return;
    };
    let Some(notice) = report.need(tree, "NOTICE") else {
        return;
    };
    let Some(mapping) = report.need(tree, BASELINE) else {
        return;
    };
    let packages = lock_entries(&lock);
    // Cargo records the wrapper crate version, not the C library version. The reviewed mapping
    // pins that relationship explicitly rather than pretending the upstream version is in Cargo.lock.
    let mappings = baseline_lines(&mapping);
    for name in ["libsqlite3-sys", "aws-lc-sys"] {
        if packages.iter().any(|p| p.name == name)
            && mappings
                .iter()
                .filter(|line| line.split_whitespace().next() == Some(name))
                .count()
                != 1
        {
            report.add(
                "W10",
                BASELINE,
                format!("expected one reviewed bundled version mapping for {name}"),
            );
        }
    }
    for line in mappings {
        let words: Vec<_> = line.split_whitespace().collect();
        let [name, version, library, native_version] = words.as_slice() else {
            report.add(
                "W10",
                BASELINE,
                format!("invalid native version mapping {line}"),
            );
            continue;
        };
        if !packages
            .iter()
            .any(|p| p.name == *name && p.version == *version)
        {
            report.add(
                "W10",
                BASELINE,
                format!("{name} {version} does not match Cargo.lock"),
            );
        }
        let heading = format!("{library} {native_version}");
        if !notice.lines().any(|line| {
            line == heading
                || line
                    .strip_prefix(&heading)
                    .is_some_and(|rest| rest.starts_with(", "))
        }) {
            report.add(
                "W10",
                "NOTICE",
                format!("missing native library version {heading}"),
            );
        }
    }
    for name in [
        "libsqlite3-sys",
        "rusqlite",
        "aws-lc-sys",
        "aws-lc-rs",
        "rustls",
    ] {
        let versions: Vec<_> = packages
            .iter()
            .filter(|p| p.name == name)
            .map(|p| p.version.as_str())
            .collect();
        if versions.is_empty() {
            continue;
        }
        let mut seen = false;
        for (index, line) in notice.lines().enumerate() {
            let words: Vec<_> = line.split_whitespace().collect();
            for pair in words.windows(2) {
                if pair[0] != name || !pair[1].starts_with(|c: char| c.is_ascii_digit()) {
                    continue;
                }
                seen = true;
                let version = pair[1].trim_end_matches([',', '.', ':', ')']);
                if !versions.contains(&version) {
                    report.add_at(
                        "W10",
                        "NOTICE",
                        index + 1,
                        format!(
                            "{name} version {version} differs from Cargo.lock ({})",
                            versions.join(", ")
                        ),
                    );
                }
            }
        }
        if !seen {
            report.add("W10", "NOTICE", format!("missing locked version of {name}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::MemTree;

    fn good() -> MemTree {
        MemTree::of(&[
            (
                ".cargo/config.toml",
                include_str!("../../../.cargo/config.toml"),
            ),
            ("Cargo.lock", include_str!("../../../Cargo.lock")),
            ("NOTICE", include_str!("../../../NOTICE")),
            (
                "tools/repo-check/baseline/native-library-versions.txt",
                include_str!("../baseline/native-library-versions.txt"),
            ),
            (TEST_KEY_PATH, TEST_KEY),
        ])
    }

    fn result(tree: &MemTree) -> Report {
        let mut r = Report::default();
        check(tree, &mut r);
        r
    }

    #[test]
    fn native_policy_baseline_passes() {
        let report = result(&good());
        assert!(
            report.problems.is_empty() && report.faults.is_empty(),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn every_native_setting_must_be_forced_to_zero_exactly_once() {
        for key in NATIVE_ENV {
            for replacement in [
                format!("{key} = {{ value = \"1\", force = true }}"),
                format!("{key} = {{ value = \"0\", force = false }}"),
                String::new(),
            ] {
                let mut tree = good();
                tree.replace(
                    ".cargo/config.toml",
                    &format!("{key} = {{ value = \"0\", force = true }}"),
                    &replacement,
                );
                assert!(
                    result(&tree).has("W8", &["six native-library settings"]),
                    "{key}"
                );
            }
        }
        let mut tree = good();
        let config = tree.get(".cargo/config.toml");
        tree.set(
            ".cargo/config.toml",
            &format!(
                "{config}\n{} = {{ value = \"0\", force = true }}",
                NATIVE_ENV[0]
            ),
        );
        assert!(result(&tree).has("W8", &["six native-library settings"]));
    }

    #[test]
    fn only_the_exact_test_key_is_exempt() {
        for label in [
            "PRIVATE KEY",
            "RSA PRIVATE KEY",
            "EC PRIVATE KEY",
            "ENCRYPTED PRIVATE KEY",
            "OPENSSH PRIVATE KEY",
        ] {
            let mut tree = good();
            tree.set(
                ".hidden/fixture.bin",
                &format!("prefix-----BEGIN {label}-----\ndata\n"),
            );
            assert!(result(&tree).has("W9", &[".hidden/fixture.bin", "private key block"]));
        }
        let mut tree = good();
        tree.set("copied-test-key.pem", TEST_KEY);
        assert!(result(&tree).has("W9", &["copied-test-key.pem"]));
        let mut tree = good();
        tree.replace(TEST_KEY_PATH, "MIGH", "MIXX");
        assert!(result(&tree).has("W9", &[TEST_KEY_PATH]));
    }

    #[test]
    fn every_private_header_is_seen_after_other_or_incomplete_headers() {
        for prefix in [
            "-----BEGIN CERTIFICATE----- ",
            "-----BEGIN ",
            "-----BEGIN INCOMPLETE ",
        ] {
            let mut tree = good();
            tree.set(
                "prefix-key.dat",
                &format!("{prefix}{}PRIVATE KEY-----\n", "-----BEGIN "),
            );
            assert!(
                result(&tree).has("W9", &["prefix-key.dat", "private key block"]),
                "{prefix}"
            );
        }
    }

    #[test]
    fn notice_and_mapping_versions_track_the_lockfile() {
        for (from, to) in [
            ("aws-lc-sys 0.47.0", "aws-lc-sys 0.46.0"),
            ("libsqlite3-sys 0.40.0", "libsqlite3-sys 0.39.0"),
            ("AWS-LC 5.12.0", "AWS-LC 0.47.0"),
            ("SQLite 3.54.1", "SQLite 3.54.0"),
        ] {
            let mut tree = good();
            tree.replace("NOTICE", from, to);
            assert!(result(&tree).has("W10", &["NOTICE"]), "{from}");
        }
        let mut tree = good();
        tree.replace("Cargo.lock", "version = \"0.47.0\"", "version = \"0.48.0\"");
        assert!(result(&tree).has("W10", &["aws-lc-sys", "Cargo.lock"]));
    }

    #[test]
    fn canonical_lock_entries_use_packages_not_aliases_and_stop_at_tables() {
        let packages = lock_entries(
            "version = 4\n[[package]]\nname = \"pool\"\nversion = \"1.0\"\ndependencies = [\n \"pool-core 0.1.0\",\n \"serde 1.0 (registry+url)\",\n]\n[[patch.unused]]\nname = \"ghost\"\n[[package]]\nname = \"pool-core\"\ndependencies = [\"pool-protocol\"]\n",
        );
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].dependencies, ["pool-core", "serde"]);
        assert_eq!(packages[1].dependencies, ["pool-protocol"]);
    }
}
