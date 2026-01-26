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

    fn assert_duration_segment_exists(output: &str, expected_duration_text: &str) {
        assert!(
            output.contains(&format!("{ICON}{expected_duration_text}")),
            "{output}"
        );
    }

    fn assert_duration_segment_not_exists(output: &str) {
        assert!(!output.contains(ICON), "{output}");
    }

    #[test]
    fn zero() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, PromptInput::new().duration(0.0), ".");

            assert_duration_segment_not_exists(&output);
        }
    }

    #[test]
    fn non_zero() {
        let env = &TestEnv::new();

        const HOUR: f64 = 1.0 * 60.0 * 60.0;
        const MIN: f64 = 1.0 * 60.0;
        for shell in SHELLS {
            for (duration, expected_text) in [
                (0.000_000_1, "0μs"),
                (0.000_001, "1μs"),
                (0.000_010, "10μs"),
                (0.000_100, "100μs"),
                (0.001_234, "1.23ms"),
                (0.012_345, "12.3ms"),
                (1.5, "1.50s"),
                (1.0 * MIN + 5.0, "1m 5s"),
                (1.0 * HOUR + 2.0 * MIN + 3.0, "1h 2m 3s"),
            ] {
                let output = run_prompt(env, shell, PromptInput::new().duration(duration), ".");

                assert_duration_segment_exists(&output, expected_text);
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

    fn assert_contains_success_status(output: &str) {
        assert!(output.contains(&format!(" {ICON_SUCCESS} ")), "{output}");
    }

    fn assert_not_contains_success_status(output: &str) {
        assert!(!output.contains(&format!(" {ICON_SUCCESS} ")), "{output}");
    }

    fn assert_contains_error_status(output: &str, exit_status: i32) {
        assert!(
            output.contains(&format!(" {ICON_ERROR} {exit_status} ")),
            "{output}"
        );
    }

    fn assert_not_contains_error_status(output: &str) {
        assert!(!output.contains(&format!(" {ICON_ERROR} ")), "{output}");
    }

    fn assert_contains_jobs_status(output: &str) {
        assert!(output.contains(&format!(" {ICON_JOBS} ")), "{output}");
    }

    fn assert_not_contains_jobs_status(output: &str) {
        assert!(!output.contains(&format!(" {ICON_JOBS} ")), "{output}");
    }

    #[test]
    fn status_success() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new().exit_status(0), ".");

            assert_contains_success_status(&output);
            assert_not_contains_error_status(&output);
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

                assert_contains_error_status(&output, exit_status);
                assert_not_contains_success_status(&output);
            }
        }
    }

    #[test]
    fn jobs_none() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new().jobs(0), ".");

            assert_not_contains_jobs_status(&output);
        }
    }

    #[test]
    fn jobs_exists() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            for jobs in [1, 2, 3] {
                let output = run_prompt(env, shell, &PromptInput::new().jobs(jobs), ".");

                assert_contains_jobs_status(&output);
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

    use chrono::{DateTime, TimeZone, Timelike};

    const ICON: &str = "  ";

    fn assert_time_segment_exists<Tz: TimeZone>(output: &str, now: DateTime<Tz>, eps_sec: i64) {
        let icon_index = output.find(ICON).unwrap();
        let time_index = icon_index + ICON.len();
        let time_text = &output[time_index..time_index + 8]; // HH:MM:SS
        let time_segments: Vec<_> = time_text.split(":").collect();
        assert_eq!(time_segments.len(), 3);

        let hour = time_segments[0].parse::<u32>().unwrap();
        let min = time_segments[1].parse::<u32>().unwrap();
        let sec = time_segments[2].parse::<u32>().unwrap();
        let time = now
            .with_hour(hour)
            .and_then(|t| t.with_minute(min))
            .and_then(|t| t.with_second(sec))
            .unwrap();
        let diff = (time - now).num_seconds().abs();
        assert!(
            diff < eps_sec,
            "time difference is too large: {diff} seconds"
        );
    }

    #[test]
    fn time() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new(), ".");

            let now = chrono::Local::now();
            assert_time_segment_exists(&output, now, 2);
        }
    }
}

mod user {
    use super::*;

    fn assert_user_segment_exists(output: &str) {
        let user = uzers::get_current_username()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let host = hostname::get().unwrap().to_string_lossy().to_string();

        assert!(output.contains(&format!(" {user}@{host} ")), "{output}");
    }

    #[test]
    fn user_host() {
        let env = &TestEnv::new();

        for shell in SHELLS {
            let output = run_prompt(env, shell, &PromptInput::new(), ".");

            assert_user_segment_exists(&output);
        }
    }
}
