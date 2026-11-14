//! Read access to a repository tree: the working tree on disk, or a tree held in memory for tests.

use std::fs;
use std::path::PathBuf;

/// A tree of files addressed by paths relative to its root, with `/` as separator.
pub trait Tree {
    /// Returns the content of a file, `Ok(None)` when no such file exists, or an error text when it exists but cannot be read.
    fn read(&self, path: &str) -> Result<Option<String>, String>;

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

#[cfg(test)]
fn path_exists<'a>(mut files: impl Iterator<Item = &'a String>, path: &str) -> bool {
    if path.is_empty() {
        return true;
    }
    let prefix = format!("{path}/");
    files.any(|file| file == path || file.starts_with(&prefix))
}

#[cfg(test)]
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

/// A tree held in memory, for tests.
#[cfg(test)]
#[derive(Clone, Default)]
pub struct MemTree {
    files: std::collections::BTreeMap<String, String>,
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
        Ok(self.files.get(path).cloned())
    }

    fn exists(&self, path: &str) -> bool {
        path_exists(self.files.keys(), path)
    }

    fn walk(&self, dir: &str, skip_dir: &dyn Fn(&str) -> bool) -> Result<Vec<String>, String> {
        Ok(walk_names(self.files.keys(), dir, skip_dir))
    }
}
