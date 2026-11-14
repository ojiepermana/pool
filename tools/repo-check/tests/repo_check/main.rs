//! Integration tests of spec 0002: the real `pool-repo-check` binary on trees on disk and on Git revisions (0002 AC-6 to AC-10), and the exit status of `scripts/check.sh` (0002 AC-1).
//!
//! The unit tests in `src/` use trees held in memory. These tests cover what they cannot: the disk reader, the Git revision reader behind `--base`, the exit status and the last line of stdout.
//!
//! Some tests lock in behavior that the verify of milestone 3 named as fragile. Each of those says `Gap:` in its comment. They describe the program as it is today; a change to the rule should change the test with it.

mod check_script;
mod ci;
mod cli;
mod common;
mod docs;
mod workspace;
