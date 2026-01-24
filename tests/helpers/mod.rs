use std::{fs::canonicalize, path::PathBuf, process::Command};

use tempfile::TempDir;

pub struct TestEnv {
    test_dir: TempDir,
}

impl TestEnv {
    pub fn new() -> Self {
        let test_dir = TempDir::new().unwrap();
        TestEnv { test_dir }
    }

    pub fn path(&self) -> PathBuf {
        canonicalize(self.test_dir.path()).unwrap()
    }

    pub fn run_command(&self, args: &[&str]) -> Result<(String, String), (i32, String)> {
        self.run_command_in(args, ".")
    }

    pub fn run_command_in(
        &self,
        args: &[&str],
        dir: &str,
    ) -> Result<(String, String), (i32, String)> {
        let output = self.command(dir).args(args).output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok((stdout, stderr))
        } else {
            Err((output.status.code().unwrap_or(-1), stderr))
        }
    }

    pub fn command(&self, dir: &str) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_croque"));
        cmd.env_clear().current_dir(self.path().join(dir));
        cmd
    }
}
