//! JavaScript pnpm ecosystem.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Error, Generator, Result};

pub struct Pnpm;

impl Generator for Pnpm {
    fn lockfile_path(&self, manifest_path: &Path) -> Result<PathBuf> {
        let project_path = manifest_path
            .parent()
            .ok_or_else(|| Error::InvalidManifest(manifest_path.to_path_buf()))?;
        Ok(project_path.join("pnpm-lock.yaml"))
    }

    fn command(&self, _manifest_path: &Path) -> Command {
        let mut command = Command::new("pnpm");
        command.args(["install", "--lockfile-only", "--ignore-scripts"]);
        command
    }

    fn tool(&self) -> &'static str {
        "pnpm"
    }
}
