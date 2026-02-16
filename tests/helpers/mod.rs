use std::{
    fs::canonicalize,
    path::PathBuf,
    process::{Command, Stdio},
};

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

    #[allow(unused)]
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

    pub fn write_file(&self, filename: &str, content: &str) {
        std::fs::write(self.path().join(filename), content).unwrap();
    }

    pub fn command(&self, dir: &str) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_croque"));
        cmd.env_clear().current_dir(self.path().join(dir));
        cmd
    }

    #[allow(unused)]
    pub fn git(&self, dir: &str) -> GitRepo {
        GitRepo::new(self.path().join(dir))
    }
}

#[derive(Debug)]
pub struct GitRepo {
    path: PathBuf,
}

#[allow(unused)]
impl GitRepo {
    pub fn new(path: PathBuf) -> Self {
        GitRepo { path }
    }

    fn git(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.env_clear()
            .current_dir(&self.path)
            .stderr(Stdio::inherit());
        cmd
    }

    pub fn init(&self, initial_branch: &str) {
        let output = self
            .git()
            .args(&["init", "--initial-branch", initial_branch])
            .output()
            .unwrap();
        if !output.status.success() {
            panic!(
                "Failed to initialize git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        self.config_set("init.defaultBranch", initial_branch);
        self.config_set("commit.gpgSign", "false");
        self.config_set("user.name", "");
        self.config_set("user.email", "");
        self.config_set("user.signingKey", "");
    }

    pub fn config_set(&self, key: &str, value: &str) {
        let output = self.git().args(&["config", key, value]).output().unwrap();
        if !output.status.success() {
            panic!(
                "Failed to set git config {}: {}",
                key,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    pub fn add(&self, files: &[&str]) {
        let output = self.git().arg("add").args(files).output().unwrap();
        if !output.status.success() {
            panic!(
                "Failed to add files to git: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    pub fn commit(&self, message: &str) {
        let output = self
            .git()
            .args(&["commit", "-m", message])
            .output()
            .unwrap();
        if !output.status.success() {
            panic!(
                "Failed to commit: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
