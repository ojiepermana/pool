//! Read access to a repository tree: the working tree on disk, a Git revision, or a tree held in memory for tests.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A tree of files addressed by paths relative to its root, with `/` as separator.
pub trait Tree {
    /// Returns the content of a file, `Ok(None)` when no such file exists, or an error text when it exists but cannot be read.
    fn read(&self, path: &str) -> Result<Option<String>, String>;

    /// Reads raw bytes for guards that also inspect binary files.
    fn read_bytes(&self, path: &str) -> Result<Option<Vec<u8>>, String> {
        self.read(path).map(|text| text.map(String::into_bytes))
    }

    /// Reports whether a file or a directory exists at `path`. The empty path is the root.
    fn exists(&self, path: &str) -> bool;

    /// Lists every file under `dir` (the empty string is the root), sorted. A directory whose name makes `skip_dir` return true is not entered.
    fn walk(&self, dir: &str, skip_dir: &dyn Fn(&str) -> bool) -> Result<Vec<String>, String>;
}

/// Joins a directory and a name into a tree path.
pub fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

/// Returns the directory part of a tree path, the empty string for a file at the root.
pub fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// Returns the last component of a tree path.
pub fn file_name(path: &str) -> &str {
    path.rsplit_once('/').map_or(path, |(_, name)| name)
}

/// The working tree on disk.
pub struct FsTree {
    root: PathBuf,
}

impl FsTree {
    /// Opens the tree whose root is `root`.
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn full(&self, path: &str) -> PathBuf {
        if path.is_empty() {
            self.root.clone()
        } else {
            self.root.join(path)
        }
    }

    fn walk_into(
        &self,
        dir: &str,
        skip_dir: &dyn Fn(&str) -> bool,
        out: &mut Vec<String>,
    ) -> Result<(), String> {
        let full = self.full(dir);
        let entries = fs::read_dir(&full).map_err(|e| format!("{}: {e}", display(dir)))?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| format!("{}: {e}", display(dir)))?;
            let kind = entry
                .file_type()
                .map_err(|e| format!("{}: {e}", display(dir)))?;
            let name = entry.file_name().to_string_lossy().into_owned();
            names.push((name, kind.is_dir(), kind.is_file()));
        }
        names.sort();
        for (name, is_dir, is_file) in names {
            let path = join(dir, &name);
            if is_dir {
                if !skip_dir(&name) {
                    self.walk_into(&path, skip_dir, out)?;
                }
            } else if is_file {
                out.push(path);
            }
        }
        Ok(())
    }
}

fn display(dir: &str) -> &str {
    if dir.is_empty() { "." } else { dir }
}

impl Tree for FsTree {
    fn read(&self, path: &str) -> Result<Option<String>, String> {
        let full = self.full(path);
        if !full.is_file() {
            return Ok(None);
        }
        fs::read_to_string(&full)
            .map(Some)
            .map_err(|e| format!("{path}: {e}"))
    }

    fn read_bytes(&self, path: &str) -> Result<Option<Vec<u8>>, String> {
        let full = self.full(path);
        if !full.is_file() {
            return Ok(None);
        }
        fs::read(full).map(Some).map_err(|e| format!("{path}: {e}"))
    }

    fn exists(&self, path: &str) -> bool {
        self.full(path).exists()
    }

    fn walk(&self, dir: &str, skip_dir: &dyn Fn(&str) -> bool) -> Result<Vec<String>, String> {
        let mut out = Vec::new();
        if !self.full(dir).is_dir() {
            return Ok(out);
        }
        self.walk_into(dir, skip_dir, &mut out)?;
        Ok(out)
    }
}

/// The committed tree of one Git revision, read with `git ls-tree` and `git show`.
pub struct GitTree {
    root: PathBuf,
    revision: String,
    files: BTreeSet<String>,
}

impl GitTree {
    /// Lists the files of `revision` in the repository at `root`, or returns why `git` failed.
    ///
    /// The root must be the top of its Git work tree, so that the paths at the revision and the paths under the root are the same. The revision is resolved to a commit hash first, so a revision that starts with a hyphen is never read as an option.
    pub fn open(root: &Path, revision: &str) -> Result<Self, String> {
        let prefix = git(root, &["rev-parse", "--show-prefix"])?;
        top_of_work_tree(root, prefix.trim_end())?;
        let commit = git(
            root,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                "--end-of-options",
                &format!("{revision}^{{commit}}"),
            ],
        )
        .map_err(|_| format!("git rev-parse: {revision} is not a commit"))?;
        let commit = commit.trim().to_string();
        let listing = git(
            root,
            &[
                "ls-tree",
                "-r",
                "--full-tree",
                "--name-only",
                "-z",
                &commit,
                "--",
            ],
        )?;
        let files = listing
            .split('\0')
            .filter(|name| !name.is_empty())
            .map(str::to_string)
            .collect();
        Ok(Self {
            root: root.to_path_buf(),
            revision: commit,
            files,
        })
    }
}

/// Accepts the root only when `git rev-parse --show-prefix` printed nothing, that is when the root is the top of its work tree.
fn top_of_work_tree(root: &Path, prefix: &str) -> Result<(), String> {
    if prefix.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "the root {} is the folder {prefix} of a Git work tree, not its top; --base needs the repository root",
            root.display()
        ))
    }
}

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|e| format!("git {}: {e}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git {}: {}", args.join(" "), stderr.trim()));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| format!("git {}: output is not UTF-8", args.join(" ")))
}

impl Tree for GitTree {
    fn read(&self, path: &str) -> Result<Option<String>, String> {
        if !self.files.contains(path) {
            return Ok(None);
        }
        git(&self.root, &["show", &format!("{}:{path}", self.revision)]).map(Some)
    }

    fn exists(&self, path: &str) -> bool {
        path_exists(self.files.iter(), path)
    }

    fn walk(&self, dir: &str, skip_dir: &dyn Fn(&str) -> bool) -> Result<Vec<String>, String> {
        Ok(walk_names(self.files.iter(), dir, skip_dir))
    }
}

fn path_exists<'a>(mut files: impl Iterator<Item = &'a String>, path: &str) -> bool {
    if path.is_empty() {
        return true;
    }
    let prefix = format!("{path}/");
    files.any(|file| file == path || file.starts_with(&prefix))
}

fn walk_names<'a>(
    files: impl Iterator<Item = &'a String>,
    dir: &str,
    skip_dir: &dyn Fn(&str) -> bool,
) -> Vec<String> {
    let prefix = if dir.is_empty() {
        String::new()
    } else {
        format!("{dir}/")
    };
    files
        .filter(|file| file.starts_with(&prefix))
        .filter(|file| {
            let rest = &file[prefix.len()..];
            let mut parts: Vec<&str> = rest.split('/').collect();
            parts.pop();
            !parts.iter().any(|part| skip_dir(part))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod git_tests {
    use super::*;

    #[test]
    fn root_must_be_the_top_of_the_work_tree() {
        assert!(top_of_work_tree(Path::new("."), "").is_ok());
        let err = top_of_work_tree(Path::new("inner"), "inner/").unwrap_err();
        assert!(err.contains("inner/") && err.contains("--base"), "{err}");
    }
}

/// A tree held in memory, for tests.
#[cfg(test)]
#[derive(Clone, Default)]
pub struct MemTree {
    files: std::collections::BTreeMap<String, String>,
    unreadable: std::collections::BTreeSet<String>,
}

#[cfg(test)]
impl MemTree {
    /// Builds a tree from `(path, content)` pairs.
    pub fn of(files: &[(&str, &str)]) -> Self {
        let mut tree = Self::default();
        for (path, content) in files {
            tree.set(path, content);
        }
        tree
    }

    /// Adds or replaces a file.
    pub fn set(&mut self, path: &str, content: &str) {
        self.files.insert(path.to_string(), content.to_string());
    }

    /// Makes reading an existing file fail, as an unreadable file or a failed `git show` would.
    pub fn set_unreadable(&mut self, path: &str) {
        self.unreadable.insert(path.to_string());
    }

    /// Removes a file.
    pub fn remove(&mut self, path: &str) {
        self.files.remove(path);
    }

    /// Returns the content of a file that the test put there.
    pub fn get(&self, path: &str) -> String {
        self.files.get(path).cloned().unwrap_or_default()
    }

    /// Replaces the first occurrence of `from` in a file with `to`.
    pub fn replace(&mut self, path: &str, from: &str, to: &str) {
        let content = self.get(path);
        assert!(content.contains(from), "{path} does not contain {from}");
        self.set(path, &content.replacen(from, to, 1));
    }
}

#[cfg(test)]
impl Tree for MemTree {
    fn read(&self, path: &str) -> Result<Option<String>, String> {
        if self.unreadable.contains(path) && self.files.contains_key(path) {
            return Err(format!("{path}: unreadable in this test"));
        }
        Ok(self.files.get(path).cloned())
    }

    fn exists(&self, path: &str) -> bool {
        path_exists(self.files.keys(), path)
    }

    fn walk(&self, dir: &str, skip_dir: &dyn Fn(&str) -> bool) -> Result<Vec<String>, String> {
        Ok(walk_names(self.files.keys(), dir, skip_dir))
    }
}
