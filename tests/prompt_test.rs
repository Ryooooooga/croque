use std::path::PathBuf;

use crate::helpers::TestEnv;

mod helpers;

const SHELLS: &[&str] = &["bash", "fish", "zsh"];

struct PromptInput {
    config: PathBuf,
    exit_status: i32,
    duration: f64,
    jobs: i32,
    width: i32,
    data_git: Option<String>,
    data_gh: Option<String>,
    data_glab: Option<String>,
}

impl PromptInput {
    fn new() -> Self {
        Self {
            config: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/config/default.yaml"),
            exit_status: 0,
            duration: 0.0,
            jobs: 0,
            width: 120,
            data_git: None,
            data_gh: None,
            data_glab: None,
        }
    }

    fn snapshot_config(&mut self) -> &mut Self {
        self.config =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/config/snapshot.yaml");
        println!("Using config: {:?}", self.config);
        self
    }

    fn exit_status(&mut self, exit_status: i32) -> &mut Self {
        self.exit_status = exit_status;
        self
    }

    fn jobs(&mut self, jobs: i32) -> &mut Self {
        self.jobs = jobs;
        self
    }

    fn duration(&mut self, duration: f64) -> &mut Self {
        self.duration = duration;
        self
    }
}

fn run_prompt(env: &TestEnv, shell: &str, input: &PromptInput, dir: &str) -> String {
    let mut cmd = env.command(dir);
    cmd.args(&[
        "prompt",
        shell,
        "--exit-status",
        &input.exit_status.to_string(),
        "--duration",
        &input.duration.to_string(),
        "--jobs",
        &input.jobs.to_string(),
        "--width",
        &input.width.to_string(),
    ]);
    if let Some(data_git) = &input.data_git {
        cmd.args(&["--data-git", data_git]);
    }
    if let Some(data_gh) = &input.data_gh {
        cmd.args(&["--data-gh", data_gh]);
    }
    if let Some(data_glab) = &input.data_glab {
        cmd.args(&["--data-glab", data_glab]);
    }

    let output = cmd
        .env("HOME", env.path())
        .env("HOST", "host")
        .env("CROQUE_CONFIG_FILE", &input.config)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(output.status.success(), "stderr: {stderr}");
    assert_eq!(stderr, "");
    assert_ne!(stdout, "");

    stdout
}

mod direnv {
    // TODO
}

mod duration {
    use super::*;

    const ICON: &str = " 󰔛 ";

    #[test]
    fn zero() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, PromptInput::new().duration(0.0), ".");

            assert!(!output.contains(ICON), "{shell}");
        }
    }

    #[test]
    fn non_zero() {
        let env = &TestEnv::new();

        const HOUR: f64 = 1.0 * 60.0 * 60.0;
        const MIN: f64 = 1.0 * 60.0;
        for shell in SHELLS {
            for (duration, expected_text) in [
                (0.000_000_1, " 0μs"),
                (0.000_001, " 1μs"),
                (0.000_010, " 10μs"),
                (0.000_100, " 100μs"),
                (0.001_234, " 1.23ms"),
                (0.012_345, " 12.3ms"),
                (1.5, "1.50s"),
                (1.0 * MIN + 5.0, "1m 5s"),
                (1.0 * HOUR + 2.0 * MIN + 3.0, "1h 2m 3s"),
            ] {
                let output = run_prompt(env, shell, PromptInput::new().duration(duration), ".");

                assert!(output.contains(ICON), "{shell} {duration}: {output}");
                assert!(
                    output.contains(expected_text),
                    "{shell} {duration}: {output} expected: {expected_text}"
                );
            }
        }
    }

    #[test]
    fn snapshot() {
        let env = &TestEnv::new();

        const HOUR: f64 = 1.0 * 60.0 * 60.0;
        const MIN: f64 = 1.0 * 60.0;
        for shell in SHELLS {
            for duration in [
                0.0,
                0.000_000_1,
                0.000_001,
                0.000_010,
                0.000_100,
                0.001_234,
                0.012_345,
                1.5,
                1.0 * MIN + 5.0,
                1.0 * HOUR + 2.0 * MIN + 3.0,
            ] {
                let output = run_prompt(
                    env,
                    shell,
                    PromptInput::new().snapshot_config().duration(duration),
                    ".",
                );

                insta::assert_snapshot!(format!("{}_{}", shell, duration), output);
            }
        }
    }
}

mod gh_pull_request {
    // TODO
}

mod git_status {
    // TODO
}

mod git_user {
    // TODO
}

mod glab_merge_request {
    // TODO
}

mod os {
    // TODO
}

mod path {
    // TODO
}

mod status {
    use super::*;

    const ICON_SUCCESS: &str = "✓";
    const ICON_ERROR: &str = "";
    const ICON_JOBS: &str = "";

    #[test]
    fn status_success() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new().exit_status(0), ".");

            assert!(
                output.contains(&format!(" {ICON_SUCCESS} ")),
                "{shell}: {output}"
            );
            assert!(
                !output.contains(ICON_ERROR),
                "{shell}: unexpected error icon: {output}"
            );
        }
    }

    #[test]
    fn status_fail() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            for exit_status in [1, 127, 130] {
                let output = run_prompt(
                    env,
                    shell,
                    &PromptInput::new().exit_status(exit_status),
                    ".",
                );

                assert!(
                    output.contains(&format!(" {ICON_ERROR} {exit_status} ")),
                    "{shell} exit status {exit_status}: {output}"
                );
                assert!(
                    !output.contains(ICON_SUCCESS),
                    "{shell} exit status {exit_status}: unexpected success icon: {output}"
                );
            }
        }
    }

    #[test]
    fn jobs_none() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new().jobs(0), ".");

            assert!(
                !output.contains(&format!(" {ICON_JOBS} ")),
                "{shell} jobs: {output}"
            );
        }
    }

    #[test]
    fn jobs_exists() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            for jobs in [1, 2, 3] {
                let output = run_prompt(env, shell, &PromptInput::new().jobs(jobs), ".");

                assert!(
                    output.contains(&format!(" {ICON_JOBS} ")),
                    "{shell} jobs {jobs}: {output}"
                );
            }
        }
    }

    #[test]
    fn snapshot() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            for (exit_status, jobs) in [(0, 0), (1, 0), (0, 1), (130, 2)] {
                let output = run_prompt(
                    env,
                    shell,
                    &PromptInput::new()
                        .snapshot_config()
                        .exit_status(exit_status)
                        .jobs(jobs),
                    ".",
                );

                insta::assert_snapshot!(
                    format!("{shell}_--exit-status={exit_status}_--jobs={jobs}"),
                    output
                );
            }
        }
    }
}

mod time {
    use super::*;

    use chrono::Timelike;

    const ICON: &str = "  ";

    #[test]
    fn time() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new(), ".");

            let icon_index = output.find(ICON).unwrap();
            let time_index = icon_index + ICON.len();
            let time_text = &output[time_index..time_index + 8]; // HH:MM:SS
            let time_segments: Vec<_> = time_text.split(":").collect();
            assert_eq!(time_segments.len(), 3);

            let local_now = chrono::Local::now();
            let hour = time_segments[0].parse::<u32>().unwrap();
            let min = time_segments[1].parse::<u32>().unwrap();
            let sec = time_segments[2].parse::<u32>().unwrap();
            let time = local_now
                .with_hour(hour)
                .and_then(|t| t.with_minute(min))
                .and_then(|t| t.with_second(sec))
                .unwrap();
            let diff = (time - local_now).num_seconds().abs();
            assert!(
                diff < 5,
                "{shell}: time difference is too large: {diff} seconds"
            );
        }
    }
}

mod user {
    use super::*;

    #[test]
    fn user_host() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new(), ".");

            let user = uzers::get_current_username()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let host = hostname::get().unwrap().to_string_lossy().to_string();

            assert!(
                output.contains(&format!(" {user}@{host} ")),
                "{shell}: {output}"
            );
        }
    }
}
